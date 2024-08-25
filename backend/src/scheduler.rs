use api::{connections::get_profile_id, post::publish_article};
use crate::config::settings::load_config;
use chrono::{Utc, DateTime, FixedOffset};
use mongodb::{Client, bson::{doc, Document, DateTime as BsonDateTime}};
use tokio::time::{self, Duration};
use futures_util::TryStreamExt;
use log::{info, error};

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
        let local_now = now.with_timezone(&FixedOffset::west_opt(3 * 3600).unwrap()); // Convert to Brazil time (UTC-3)
        info!("Checking posts at local time: {}", local_now);

        let bson_now = BsonDateTime::from_millis(now.timestamp_millis());

        let filter = doc! { "scheduled_time": { "$lte": bson_now }, "status": "pending" };
        let mut cursor = posts.find(filter).await?;

        while let Ok(Some(post)) = cursor.try_next().await {
            let scheduled_time_str = post.get_str("scheduled_time").unwrap_or("");
            info!("Scheduled time from MongoDB: {}", scheduled_time_str);

            let scheduled_time = DateTime::parse_from_rfc3339(scheduled_time_str)
                .map(|dt| dt.with_timezone(&FixedOffset::west_opt(3 * 3600).unwrap()))  // Convert to Brazil time (UTC-3)
                .unwrap_or(local_now);

            if scheduled_time > local_now {
                let delay_duration = scheduled_time - local_now;
                info!("Delaying until scheduled time: {}", scheduled_time);
                time::sleep(Duration::from_secs(delay_duration.num_seconds() as u64)).await;
            }

            let title = post.get_str("title").unwrap_or("Untitled").to_string();
            let content = post.get_str("content").unwrap_or("No content provided").to_string();

            if let Err(e) = publish_article(&access_token, &get_profile_id(&access_token).await?, &title, &content).await {
                error!("Error publishing article: {}", e);
            } else {
                let update = doc! { "$set": { "status": "published" } };
                posts.update_one(doc! {"_id": post.get_object_id("_id").unwrap()}, update).await?;
                info!("Post published successfully: {}", title);
            }
        }

        time::sleep(Duration::from_secs(20)).await;
    }
}
