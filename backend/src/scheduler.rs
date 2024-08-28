use crate::config::settings::load_config;
use api::{connections::get_profile_id, post::publish_article};
use chrono::{DateTime, FixedOffset, TimeZone, Utc};
use futures_util::StreamExt;
use futures_util::TryStreamExt;
use log::{error, info};
use mongodb::{
    bson::{doc, DateTime as BsonDateTime, Document},
    Client,
};
use tokio::time::{self, Duration};

mod api;
mod config;

/// The main entry point for the LinkedIn post scheduler.
/// This function continuously checks the MongoDB collection for posts scheduled
/// to be published at or before the current time and publishes them on LinkedIn.
///
/// # Returns
///
/// This function returns a `Result<(), Box<dyn std::error::Error>>`. The function
/// will return `Ok(())` if it runs successfully, or an `Err` if an error occurs during execution.
///
/// # Errors
///
/// This function will return an error in the following situations:
///
/// - If the configuration file `config.toml` cannot be loaded.
/// - If the connection to the MongoDB database fails.
/// - If there's an error retrieving posts from the MongoDB collection.
/// - If there's an error publishing an article to LinkedIn.
/// - If there's an error updating the status of a post in MongoDB after publishing.
///
/// # Functionality
///
/// 1. **Logging Initialization**: The function initializes the logging environment
///    using `env_logger` to provide detailed runtime information for monitoring and debugging.
///
/// 2. **Configuration Loading**: It loads the configuration settings from `config.toml`
///    using the `load_config` function, including the `access_token` for LinkedIn API access.
///
/// 3. **MongoDB Connection**: It establishes a connection to a MongoDB database, specifically
///    targeting the `lkdin-posts` database and the `posts` collection, where scheduled posts are stored.
///
/// 4. **Infinite Loop**: The function enters an infinite loop to continuously check for posts that
///    need to be published:
///
///    - The current time is obtained in UTC and then converted to the local time in Brazil (UTC-3).
///
///    - It constructs a MongoDB filter to find posts with a `scheduled_time` less than or equal to the current time
///      and with a `status` of "pending".
///
///    - The function retrieves these posts using a cursor and iterates through them.
///
///    - For each post:
///      - The scheduled time is retrieved and converted to the local time in Brazil.
///      - If the scheduled time is still in the future, the function waits until the post's scheduled time.
///
///      - The post's `title` and `content` are retrieved and used to publish the article to LinkedIn using the `publish_article` function.
///
///      - If the post is published successfully, the function updates the `status` of the post to "published" in MongoDB.
///
///    - The loop then waits for 20 seconds before checking for new posts to publish.
///
/// # Example Usage
///
/// To run this scheduler, simply execute the compiled binary:
///
/// ```bash
/// cargo run --bin scheduler
/// ```
///
/// The scheduler will start running, continuously checking the MongoDB database for posts to publish
/// and logging the results to the console.
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    let config = load_config("config.toml")?;
    let mongo_client = Client::with_uri_str("mongodb://localhost:27017").await?;
    let db = mongo_client.database("lkdin-posts");
    let posts: mongodb::Collection<Document> = db.collection("posts");
    let access_token = config.access_token.clone();

    loop {
        let now = Utc::now();
        let brazil_offset = FixedOffset::west_opt(3 * 3600).unwrap();
        let local_now = now.with_timezone(&brazil_offset);
        info!("Checking posts at local time: {}", local_now);

        let now_millis = local_now.timestamp_millis();

        let filter = doc! { "scheduled_time": { "$lte": now_millis }, "status": "pending" };
        let mut cursor = posts.find(filter).await?;

        while let Ok(Some(post)) = cursor.try_next().await {
            let scheduled_time_millis = post.get_i64("scheduled_time").unwrap_or(0);
            let scheduled_time = brazil_offset.timestamp_millis(scheduled_time_millis);

            info!("Scheduled time from MongoDB: {}", scheduled_time);

            if scheduled_time > local_now {
                let delay_duration = scheduled_time.signed_duration_since(local_now);
                info!("Delaying until scheduled time: {}", scheduled_time);
                time::sleep(Duration::from_secs(delay_duration.num_seconds() as u64)).await;
            }

            let title = post.get_str("title").unwrap_or("Untitled").to_string();
            let content = post
                .get_str("content")
                .unwrap_or("No content provided")
                .to_string();

            if let Err(e) = publish_article(
                &access_token,
                &get_profile_id(&access_token, None).await?,
                &title,
                &content,
                None,
            )
                .await
            {
                error!("Error publishing article: {}", e);
            } else {
                let update = doc! { "$set": { "status": "published" } };
                posts
                    .update_one(doc! {"_id": post.get_object_id("_id").unwrap()}, update)
                    .await?;
                info!("Post published successfully: {}", title);
            }
        }

        time::sleep(Duration::from_secs(20)).await;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockito::Server;
    use mongodb::bson::oid::ObjectId;
    use std::sync::Once;

    static INIT: Once = Once::new();

    /// Initializes the test environment by configuring the logger.
    ///
    /// This function is called once at the beginning of the test suite to ensure that the logger is properly configured for testing purposes.
    ///
    /// # Arguments
    ///
    /// This function does not take any arguments.
    ///
    /// # Returns
    ///
    /// This function does not return any value.
    ///
    /// # Example
    ///
    /// ```rust
    /// initialize();
    /// ```
    fn initialize() {
        INIT.call_once(|| {
            let _ = env_logger::builder().is_test(true).try_init();
        });
    }

    /// Creates a mock MongoDB client for testing purposes.
    ///
    /// This function sets up an in-memory MongoDB instance that can be used for testing without affecting a real database.
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing a `mongodb::Client` if successful, or an error if the client creation fails.
    async fn create_mock_mongo_client() -> Result<Client, mongodb::error::Error> {
        Client::with_uri_str("mongodb://localhost:27017").await
    }

    /// Tests the retrieval of scheduled posts from a MongoDB collection.
    ///
    /// This asynchronous test function performs the following steps:
    ///
    /// 1. **Initialization**: Calls the `initialize` function to set up the test environment,
    ///    which typically includes configuring logging or other necessary setups for testing.
    ///
    /// 2. **MongoDB Client Setup**: Creates a mock MongoDB client using the `create_mock_mongo_client`
    ///    function. This client is connected to a test database (`test_lkdin-posts`) to ensure that
    ///    the test does not affect the production data.
    ///
    /// 3. **Document Insertion**: Inserts a test post document into the `posts` collection.
    ///    The document includes a title, content, scheduled time (current UTC time), and a status of "pending".
    ///
    /// 4. **Document Count Check**: Counts the number of documents in the collection after insertion
    ///    to ensure the test setup is correct.
    ///
    /// 5. **Document Retrieval**: Queries the `posts` collection to find documents that have a `scheduled_time`
    ///    less than or equal to the current time and a status of "pending". This simulates the typical behavior
    ///    of retrieving posts that are scheduled to be published.
    ///
    /// 6. **Assertion**: Iterates over the retrieved documents to find a match with the inserted test post.
    ///    It asserts that the title, content, and status of the retrieved document match the inserted values.
    ///    The test will fail if no matching document is found or if the retrieved document's fields do not match.
    ///
    /// # Panics
    ///
    /// This function will panic if:
    ///
    /// - The MongoDB client cannot be created.
    /// - The document insertion fails.
    /// - The document retrieval process encounters an error.
    /// - No matching documents are found after the retrieval query.
    ///
    /// # Example
    ///
    /// To run this test, use the following command:
    ///
    /// ```bash
    /// cargo test -- --test-threads=1
    /// ```
    ///
    /// The test will output information about the insertion result, the number of documents in the collection,
    /// and details about the retrieved document(s).
    ///
    #[tokio::test]
    async fn test_retrieve_scheduled_posts() {
        initialize();

        let client = create_mock_mongo_client().await.unwrap();
        let db = client.database("test_lkdin-posts");
        let posts: mongodb::Collection<Document> = db.collection("posts");

        let now = Utc::now();
        let bson_now = BsonDateTime::from_chrono(now);
        let test_post = doc! {
            "_id": ObjectId::new(),
            "title": "Test Post",
            "content": "Test Content",
            "scheduled_time": bson_now,
            "status": "pending"
        };

        let insert_result = posts.insert_one(test_post.clone()).await.unwrap();
        println!("Insert result: {:?}", insert_result);

        let count = posts.count_documents(doc! {}).await.unwrap();
        println!("Number of documents in collection: {}", count);

        let filter = doc! {
            "scheduled_time": { "$lte": bson_now },
            "status": "pending"
        };
        println!("Filter: {:?}", filter);
        println!("Inserted document: {:?}", test_post);

        let mut cursor = posts.find(filter.clone()).await.unwrap();
        let mut found = false;
        while let Some(doc_result) = cursor.next().await {
            match doc_result {
                Ok(doc) => {
                    println!("Found document: {:?}", doc);
                    assert_eq!(doc.get_str("title").unwrap(), "Test Post");
                    assert_eq!(doc.get_str("content").unwrap(), "Test Content");
                    assert_eq!(doc.get_str("status").unwrap(), "pending");
                    found = true;
                }
                Err(e) => println!("Error while iterating: {:?}", e),
            }
        }

        assert!(found, "No matching documents were found");
    }

    /// Tests error handling during post publication.
    ///
    /// This test verifies that the scheduler correctly handles errors that occur during the publication process.
    ///
    /// # Steps
    ///
    /// 1. Initializes the test environment.
    /// 2. Creates a mock MongoDB client and a mock LinkedIn API server that returns an error.
    /// 3. Inserts a test post into the database.
    /// 4. Attempts to publish the post.
    /// 5. Verifies that the post status remains "pending" in the database after a failed publication attempt.
    ///
    /// # Panics
    ///
    /// This test will panic if the assertions fail, indicating that error handling during publication is not working as expected.
    #[tokio::test]
    async fn test_publication_error_handling() {
        initialize();

        let client = create_mock_mongo_client().await.unwrap();
        let db = client.database("test_lkdin-posts");
        let posts: mongodb::Collection<Document> = db.collection("posts");

        let mut server = Server::new_async().await;
        let mock = server
            .mock("POST", "/v2/ugcPosts")
            .with_status(500)
            .with_body("Internal Server Error")
            .create_async()
            .await;

        let test_post = doc! {
            "_id": ObjectId::new(),
            "title": "Error Test Post",
            "content": "Error Test Content",
            "scheduled_time": Utc::now().to_rfc3339(),
            "status": "pending"
        };

        posts.insert_one(test_post.clone()).await.unwrap();

        let filter = doc! { "status": "pending" };
        let retrieved_post = posts.find_one(filter).await.unwrap().unwrap();

        let title = retrieved_post.get_str("title").unwrap();
        let content = retrieved_post.get_str("content").unwrap();

        let access_token = "mock_token";
        let profile_id = "mock_profile_id";
        let result = publish_article(
            access_token,
            profile_id,
            title,
            content,
            Some(&server.url()),
        )
        .await;

        assert!(result.is_err());

        let updated_post = posts
            .find_one(doc! {"_id": retrieved_post.get_object_id("_id").unwrap()})
            .await
            .unwrap()
            .unwrap();
        assert_eq!(updated_post.get_str("status").unwrap(), "pending");

        mock.assert_async().await;
    }
}
