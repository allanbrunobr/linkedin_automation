use futures_util::TryStreamExt;
use log::{error, info};
use mongodb::{
    bson::{doc, DateTime as BsonDateTime},
    Client,
};
use serde::{Deserialize, Serialize};
use std::fmt;
use std::sync::Arc;
use bson::Bson;
use chrono::{DateTime, FixedOffset, NaiveDateTime, TimeZone, Utc};
use warp::cors;
use warp::reject::Reject;
use warp::Filter;
use warp::http::Method;


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
/// cargo run
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

    // Define the routes for the API endpoints.

    // Route to schedule a new post.
    let schedule_post = {
        let posts = Arc::clone(&posts);
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

                    let brazil_offset = FixedOffset::west_opt(3 * 3600).unwrap();
                    let brazil_date = brazil_offset.from_local_datetime(&naive_date).unwrap();

                    info!("Date in Brazil time: {}", brazil_date);

                    let milliseconds = brazil_date.timestamp_millis();

                    info!("Date stored as milliseconds: {}", milliseconds);

                    let doc = doc! {
                    "title": post.title,
                    "content": post.content,
                    "scheduled_time": Bson::Int64(milliseconds),
                    "status": post.status,
                };
                    posts.insert_one(doc).await.unwrap();
                    Ok::<_, warp::Rejection>(warp::reply::with_status(
                        "Post scheduled",
                        warp::http::StatusCode::OK,
                    ))
                }
            })
            .with(cors.clone())
    };

    // Route to query posts within a specified date range.
    let query_posts = {
        let posts = Arc::clone(&posts);
        warp::get()
            .and(warp::path("posts"))
            .and(warp::query::<PostQueryParams>())
            .and_then(move |params: PostQueryParams| {
                let posts = Arc::clone(&posts);
                async move {
                    let brazil_offset = FixedOffset::west_opt(3 * 3600).unwrap();

                    let to_millis = |date_str: &str, is_end_of_day: bool| -> Result<i64, warp::Rejection> {
                        let time_str = if is_end_of_day { "23:59:59" } else { "00:00:00" };
                        let datetime_str = format!("{} {}", date_str, time_str);
                        NaiveDateTime::parse_from_str(&datetime_str, "%Y-%m-%d %H:%M:%S")
                            .map_err(|_| warp::reject::custom(ParseDateError))
                            .map(|ndt| brazil_offset.from_local_datetime(&ndt).unwrap().timestamp_millis())
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

                    let mut cursor = posts.find(filter).await.unwrap();
                    let mut results = Vec::new();
                    while let Ok(Some(post)) = cursor.try_next().await {
                        results.push(post);
                    }
                    Ok::<_, warp::Rejection>(warp::reply::json(&results))
                }
            })
            .with(cors.clone())
    };

    // Route to delete a post by its ID.
    let delete_post = {
        let posts = Arc::clone(&posts);
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
            .with(cors.clone())
    };

    // Route to update a post by its ID.
    let update_post = {
        let posts = Arc::clone(&posts);
        warp::put()
            .and(warp::path!("posts" / String))
            .and(warp::body::json())
            .and_then(move |id: String, updated_post: Post| {
                let posts = Arc::clone(&posts);
                async move {
                    info!("Updating post with ID: {}", id);

                    let object_id = match bson::oid::ObjectId::parse_str(&id) {
                        Ok(oid) => oid,
                        Err(e) => {
                            error!("Failed to parse ObjectId from {}: {:?}", id, e);
                            return Ok::<_, warp::Rejection>(warp::reply::with_status(
                                "Invalid ID format",
                                warp::http::StatusCode::BAD_REQUEST,
                            ));
                        }
                    };

                    let naive_date = NaiveDateTime::parse_from_str(&updated_post.scheduled_time, "%Y-%m-%d %H:%M")
                        .map_err(|e| {
                            error!("Error parsing date: {}", e);
                            warp::reject::custom(ParseDateError)
                        })?;

                    let brazil_offset = FixedOffset::west_opt(3 * 3600).unwrap();
                    let brazil_date = brazil_offset.from_local_datetime(&naive_date).unwrap();

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
                                Ok::<_, warp::Rejection>(warp::reply::with_status(
                                    "Post updated",
                                    warp::http::StatusCode::OK,
                                ))
                            } else {
                                error!("No post found with ID {}", id);
                                Ok::<_, warp::Rejection>(warp::reply::with_status(
                                    "Post not found",
                                    warp::http::StatusCode::NOT_FOUND,
                                ))
                            }
                        }
                        Err(e) => {
                            error!("Failed to update post with ID {}: {:?}", id, e);
                            Ok::<_, warp::Rejection>(warp::reply::with_status(
                                "Failed to update post",
                                warp::http::StatusCode::INTERNAL_SERVER_ERROR,
                            ))
                        }
                    }
                }
            })
            .with(cors.clone())
    };


    // Start the server on port 8080, serving the defined routes with CORS configuration.
    warp::serve(
        schedule_post
            .or(query_posts)
            .or(delete_post)
            .or(update_post).with(cors),
    )
    .run(([0, 0, 0, 0], 8080))
    .await;
}
