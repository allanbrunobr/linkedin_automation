use bson::Bson;
use chrono::{DateTime, FixedOffset, NaiveDateTime, TimeZone, Utc};
use config::settings::{get_local_time, get_timezone_offset};
use futures_util::TryStreamExt;
use log::{error, info};
use mongodb::{
    bson::{doc, DateTime as BsonDateTime},
    Client,
};
use serde::{Deserialize, Serialize};
use std::fmt;
use std::sync::Arc;
use warp::http::Method;
use warp::reject::Reject;
use warp::Filter;
use warp::{Rejection};

mod config;

/// Structure representing a scheduled post.
///
/// The post includes a title, content, and a scheduled time in string format (`YYYY-MM-DD HH:MM`).
/// The status of the post is set to "pending" by default.
#[derive(Debug, Deserialize, Serialize)]
struct Post {
    title: String,
    content: String,
    scheduled_time: String,
    #[serde(default = "default_status")]
    status: String,
}

/// Structure to handle query parameters in the post query route.
///
/// The parameters include a start date and an end date to filter the scheduled posts.
#[derive(Debug, Deserialize)]
struct PostQueryParams {
    start_date: String,
    end_date: String,
}

/// Custom error structure for handling date parsing errors.
#[derive(Debug)]
struct ParseDateError;

impl fmt::Display for ParseDateError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Failed to parse date")
    }
}

impl Reject for ParseDateError {}

/// Function to provide the default status for a post, which is "pending".
fn default_status() -> String {
    "pending".to_string()
}

/// The main entry point for the LinkedIn post scheduler API server.
/// This function sets up and runs the server that handles scheduling, updating,
/// querying, and deleting posts using MongoDB as the backend database.
///
/// # Functionality
///
/// 1. **Logging Initialization**: Initializes the logging environment using `env_logger`
///    to provide detailed runtime information for monitoring and debugging.
///
/// 2. **MongoDB Connection**: Establishes a connection to a MongoDB database,
///    specifically targeting the `lkdin-posts` database and the `posts` collection,
///    where scheduled posts are stored.
///
/// 3. **CORS Configuration**: Configures Cross-Origin Resource Sharing (CORS) to allow
///    the frontend (running on a different origin) to communicate with this backend server,
///    enabling HTTP methods such as GET, POST, PUT, and DELETE.
///
/// 4. **Routing Setup**: Sets up the various HTTP routes using the Warp framework:
///    - `POST /schedule`: Schedule a new post by inserting it into the MongoDB collection.
///    - `GET /posts`: Query scheduled posts within a specified date range.
///    - `PUT /posts/{id}`: Update an existing post by its ID.
///    - `DELETE /posts/{id}`: Delete a post by its ID.
///
/// 5. **Server Execution**: The server is started and listens on `http://localhost:8080/`,
///    serving the defined routes with the CORS configuration.
///
/// # Errors
///
/// This function will terminate with an error in the following situations:
///
/// - If the connection to the MongoDB database fails.
/// - If the server fails to start due to issues such as port conflicts or other I/O errors.
///
/// # Example
///
/// To run the server, simply execute the binary. Ensure that MongoDB is running
/// and accessible at the configured URI (`mongodb://localhost:27017` by default).
///
/// ```bash
/// cargo run --bin web_server
/// ```
///
/// Once the server is running, you can interact with it using tools like `curl` or Postman,
/// or through a frontend application that communicates with this backend.
///
/// # Returns
///
/// This function is asynchronous and returns a `Result<(), Box<dyn std::error::Error>>`.
/// It will return `Ok(())` if the server runs successfully, or an `Err` if an error occurs during setup or execution.
#[tokio::main]
async fn main() {
    // Initialize the logger with a default level of "info".
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    // Create a MongoDB client and connect to the "lkdin-posts" database.
    let mongo_client = Client::with_uri_str("mongodb://localhost:27017")
        .await
        .unwrap();
    let db = mongo_client.database("lkdin-posts");
    let posts = Arc::new(db.collection("posts"));

    info!("Server running on http://localhost:8080/");

    // Configure CORS to allow any origin, and specific HTTP methods and headers.
    let cors = warp::cors()
        .allow_any_origin()
        .allow_methods(&[Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_headers(vec!["Content-Type"]);

    info!("CORS configured.");

    let update_post = update_post_route(Arc::clone(&posts));
    let schedule_post = schedule_post_route(Arc::clone(&posts));
    let query_posts = query_posts_route(Arc::clone(&posts));
    let delete_post = delete_post_route(Arc::clone(&posts));

    let routes = schedule_post
        .or(query_posts)
        .or(delete_post)
        .or(update_post)
        .with(cors);

    warp::serve(routes)
        .run(([0, 0, 0, 0], 8080))
        .await;
}

/// Configures and manages the route for scheduling new LinkedIn posts.
///
/// # Functionality
///
/// This function sets up an HTTP POST route for `/schedule` that receives and processes
/// requests to schedule new LinkedIn posts. The process includes:
///
/// 1. **Post Reception**: Accepts a JSON payload containing post details.
///
/// 2. **Date Parsing**: Converts the provided date/time string to a `NaiveDateTime` object.
///
/// 3. **Timezone Adjustment**: Applies the Brazil timezone offset to the provided date.
///
/// 4. **UTC Conversion**: Converts the Brazil timezone date to UTC.
///
/// 5. **Storage**: Inserts the post into the MongoDB database.
///
/// # Parameters
///
/// - `posts`: A shared `Arc<Collection<Document>>` for concurrent access to the MongoDB collection.
///
/// # Returns
///
/// Returns a `warp::Filter` that can be combined with other routes in the web server.
///
/// # Errors
///
/// The function may return a `Rejection` in the following situations:
///
/// - Failure to parse the provided date/time string.
/// - Error in obtaining the timezone offset.
/// - Failure to insert the document into MongoDB.
///
/// # Payload Format
///
/// The JSON payload is expected to contain the following fields:
/// - `title`: Title of the post (string)
/// - `content`: Content of the post (string)
/// - `scheduled_time`: Scheduled date and time in "YYYY-MM-DD HH:MM" format (string)
/// - `status`: Status of the post (string, typically "pending")
///
/// # Logging
///
/// The function uses logging macros to record important information:
/// - Reception of a new post for scheduling.
/// - Date converted to Brazil timezone.
/// - Timestamp in milliseconds being stored.
///
/// # Example Usage
///
/// ```rust
/// let schedule_route = schedule_post(posts.clone());
/// let routes = schedule_route.or(other_routes);
/// warp::serve(routes).run(([127, 0, 0, 1], 8080)).await;
/// ```
///
/// Where `posts` is a shared `Arc<Collection<Document>>` and `other_routes`
/// are other routes defined in your web server.
pub fn schedule_post_route(
    posts: Arc<mongodb::Collection<mongodb::bson::Document>>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::post()
        .and(warp::path("schedule"))
        .and(warp::body::json())
        .and_then(move |post: Post| {
            let posts = Arc::clone(&posts);
            async move {
                info!("Receiving a new post for scheduling: {:?}", post);

                let naive_date = NaiveDateTime::parse_from_str(&post.scheduled_time, "%Y-%m-%d %H:%M")
                    .map_err(|e| {
                        error!("Error parsing date: {}", e);
                        warp::reject::custom(ParseDateError)
                    })?;

                let brazil_offset = get_timezone_offset().map_err(|e| {
                    error!("Error getting timezone offset: {}", e);
                    warp::reject::custom(ParseDateError)
                })?;
                let brazil_date: DateTime<_> = brazil_offset.from_local_datetime(&naive_date).unwrap();

                info!("Date in Brazil time: {}", brazil_date);

                let milliseconds = brazil_date.timestamp_millis();

                info!("Date stored as milliseconds: {}", milliseconds);

                let doc = doc! {
                    "title": post.title,
                    "content": post.content,
                    "scheduled_time": Bson::Int64(milliseconds),
                    "status": post.status,
                };
                posts.insert_one(doc).await.map_err(|e| {
                    error!("Error inserting post: {}", e);
                    warp::reject::custom(ParseDateError)
                })?;
                Ok::<_, Rejection>(warp::reply::with_status(
                    "Post scheduled",
                    warp::http::StatusCode::OK,
                ))
            }
        })
}

/// Configures and manages the route for querying scheduled LinkedIn posts.
///
/// # Functionality
///
/// This function sets up an HTTP GET route for `/posts` that retrieves scheduled posts
/// within a specified date range. The process includes:
///
/// 1. **Date Range Parsing**: Converts start and end date strings to millisecond timestamps.
/// 2. **Timezone Conversion**: Adjusts timestamps to the local timezone.
/// 3. **Database Query**: Retrieves posts from MongoDB based on the date range and status.
///
/// # Parameters
///
/// - `posts`: A shared `Arc<Collection<Document>>` for concurrent access to the MongoDB collection.
///
/// # Returns
///
/// Returns a `warp::Filter` that can be combined with other routes in the web server.
///
/// # Query Parameters
///
/// Expected query parameters:
/// - `start_date`: Start date of the range (format: "YYYY-MM-DD")
/// - `end_date`: End date of the range (format: "YYYY-MM-DD")
///
/// # Errors
///
/// May return a `Rejection` in the following situations:
/// - Failure to parse date strings.
/// - Error in timezone conversion.
/// - Failure to query the MongoDB database.
///
/// # Logging
///
/// Logs the following information:
/// - Query date range in milliseconds.
/// - Any errors encountered during the process.
///
/// # Example Usage
///
/// ```rust
/// let query_route = query_posts(posts.clone());
/// let routes = query_route.or(other_routes);
/// warp::serve(routes).run(([127, 0, 0, 1], 8080)).await;
/// ```
pub fn query_posts_route(
    posts: Arc<mongodb::Collection<mongodb::bson::Document>>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::get()
        .and(warp::path("posts"))
        .and(warp::query::<PostQueryParams>())
        .and_then(move |params: PostQueryParams| {
            let posts = Arc::clone(&posts);
            async move {
                let to_millis = |date_str: &str, is_end_of_day: bool| -> Result<i64, Rejection> {
                    let time_str = if is_end_of_day { "23:59:59" } else { "00:00:00" };
                    let datetime_str = format!("{} {}", date_str, time_str);
                    let naive_dt = NaiveDateTime::parse_from_str(&datetime_str, "%Y-%m-%d %H:%M:%S")
                        .map_err(|_| warp::reject::custom(ParseDateError))?;
                    let utc_dt = Utc.from_local_datetime(&naive_dt).unwrap();
                    get_local_time(utc_dt)
                        .map(|local_time| local_time.timestamp_millis())
                        .map_err(|e| {
                            error!("Error getting local time: {}", e);
                            warp::reject::custom(ParseDateError)
                        })
                };

                let start_millis = to_millis(&params.start_date, false)?;
                let end_millis = to_millis(&params.end_date, true)?;

                info!("Querying posts from {} to {}", start_millis, end_millis);

                let filter = doc! {
                    "scheduled_time": {
                        "$gte": start_millis,
                        "$lte": end_millis,
                    },
                    "status": "pending"
                };

                let mut cursor = posts.find(filter).await.map_err(|e| {
                    error!("Error querying posts: {}", e);
                    warp::reject::custom(ParseDateError)
                })?;
                let mut results = Vec::new();
                while let Ok(Some(post)) = cursor.try_next().await {
                    results.push(post);
                }
                Ok::<_, Rejection>(warp::reply::json(&results))
            }
        })
}

/// Configures and manages the route for deleting a scheduled LinkedIn post.
///
/// # Functionality
///
/// This function sets up an HTTP DELETE route for `/posts/{id}` that removes a specific post
/// from the database based on its ID. The process includes:
///
/// 1. **ID Parsing**: Converts the provided string ID to a MongoDB ObjectId.
/// 2. **Database Deletion**: Removes the specified post from the MongoDB collection.
///
/// # Parameters
///
/// - `posts`: A shared `Arc<Collection<Document>>` for concurrent access to the MongoDB collection.
///
/// # Returns
///
/// Returns a `warp::Filter` that can be combined with other routes in the web server.
///
/// # Path Parameters
///
/// - `id`: The unique identifier of the post to be deleted.
///
/// # Errors
///
/// May return a `Rejection` if:
/// - The provided ID cannot be parsed into a valid ObjectId.
/// - The deletion operation fails in the database.
///
/// # Response
///
/// Returns a 200 OK status with the message "Post deleted" upon successful deletion.
///
/// # Example Usage
///
/// ```rust
/// let delete_route = delete_post(posts.clone());
/// let routes = delete_route.or(other_routes);
/// warp::serve(routes).run(([127, 0, 0, 1], 8080)).await;
/// ```
pub fn delete_post_route(
    posts: Arc<mongodb::Collection<mongodb::bson::Document>>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::delete()
        .and(warp::path!("posts" / String))
        .and_then(move |id: String| {
            let posts = Arc::clone(&posts);
            async move {
                let object_id = bson::oid::ObjectId::parse_str(&id).unwrap();
                posts.delete_one(doc! { "_id": object_id }).await.unwrap();
                Ok::<_, warp::Rejection>(warp::reply::with_status(
                    "Post deleted",
                    warp::http::StatusCode::OK,
                ))
            }
        })
}

/// Configures and manages the route for updating a scheduled LinkedIn post.
///
/// # Functionality
///
/// This function sets up an HTTP PUT route for `/posts/{id}` that updates an existing post
/// in the database. The process includes:
///
/// 1. **ID Parsing**: Converts the provided string ID to a MongoDB ObjectId.
/// 2. **Date Parsing**: Converts the updated scheduled time to a timestamp.
/// 3. **Timezone Adjustment**: Applies the Brazil timezone offset to the scheduled time.
/// 4. **Database Update**: Updates the specified post in the MongoDB collection.
///
/// # Parameters
///
/// - `posts`: A shared `Arc<Collection<Document>>` for concurrent access to the MongoDB collection.
///
/// # Returns
///
/// Returns a `warp::Filter` that can be combined with other routes in the web server.
///
/// # Path Parameters
///
/// - `id`: The unique identifier of the post to be updated.
///
/// # Request Body
///
/// Expects a JSON body with the following fields:
/// - `title`: Updated title of the post.
/// - `content`: Updated content of the post.
/// - `scheduled_time`: Updated scheduled time (format: "YYYY-MM-DD HH:MM").
/// - `status`: Updated status of the post.
///
/// # Errors
///
/// May return a `Rejection` in the following situations:
/// - Failure to parse the post ID.
/// - Failure to parse the updated date/time.
/// - Error in obtaining the timezone offset.
/// - Failure to update the document in MongoDB.
///
/// # Logging
///
/// Logs the following information:
/// - Attempt to update a post with a specific ID.
/// - Converted date in Brazil time.
/// - Success or failure of the update operation.
///
/// # Responses
///
/// - Returns a 200 OK status with "Post updated" message upon successful update.
/// - Returns a 404 Not Found status if no post matches the given ID.
/// - Returns a 500 Internal Server Error status if the update operation fails.
///
/// # Example Usage
///
/// ```rust
/// let update_route = update_post(posts.clone());
/// let routes = update_route.or(other_routes);
/// warp::serve(routes).run(([127, 0, 0, 1], 8080)).await;
/// ```
pub fn update_post_route(
    posts: Arc<mongodb::Collection<mongodb::bson::Document>>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::put()
        .and(warp::path!("posts" / String))
        .and(warp::body::json())
        .and_then(move |id: String, updated_post: Post| {
            let posts = Arc::clone(&posts);
            async move {
                info!("Updating post with ID: {}", id);

                let object_id = bson::oid::ObjectId::parse_str(&id).map_err(|e| {
                    error!("Failed to parse ObjectId from {}: {:?}", id, e);
                    warp::reject::custom(ParseDateError)
                })?;

                let naive_date = NaiveDateTime::parse_from_str(&updated_post.scheduled_time, "%Y-%m-%d %H:%M")
                    .map_err(|e| {
                        error!("Error parsing date: {}", e);
                        warp::reject::custom(ParseDateError)
                    })?;

                let brazil_offset = get_timezone_offset().map_err(|e| {
                    error!("Error getting timezone offset: {}", e);
                    warp::reject::custom(ParseDateError)
                })?;

                let brazil_date: DateTime<_> = brazil_offset.from_local_datetime(&naive_date).unwrap();

                info!("Date in Brazil time: {}", brazil_date);

                let milliseconds = brazil_date.timestamp_millis();

                let update_doc = doc! {
                    "$set": {
                        "title": updated_post.title,
                        "content": updated_post.content,
                        "scheduled_time": Bson::Int64(milliseconds),
                        "status": updated_post.status,
                    }
                };

                match posts.update_one(doc! { "_id": object_id }, update_doc).await {
                    Ok(update_result) => {
                        if update_result.matched_count > 0 {
                            info!("Post with ID {} updated successfully", id);
                            Ok::<_, Rejection>(warp::reply::with_status(
                                "Post updated",
                                warp::http::StatusCode::OK,
                            ))
                        } else {
                            error!("No post found with ID {}", id);
                            Ok::<_, Rejection>(warp::reply::with_status(
                                "Post not found",
                                warp::http::StatusCode::NOT_FOUND,
                            ))
                        }
                    }
                    Err(e) => {
                        error!("Failed to update post with ID {}: {:?}", id, e);
                        Ok::<_, Rejection>(warp::reply::with_status(
                            "Failed to update post",
                            warp::http::StatusCode::INTERNAL_SERVER_ERROR,
                        ))
                    }
                }
            }
        })
}