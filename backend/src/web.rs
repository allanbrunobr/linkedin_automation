use warp::Filter;
use mongodb::{Client, bson::{doc, DateTime as BsonDateTime}};
use serde::{Deserialize, Serialize};
use warp::cors;
use log::{info, error};

#[derive(Debug, Deserialize, Serialize)]
struct Post {
    title: String,
    content: String,
    scheduled_time: String,
    #[serde(default = "default_status")]
    status: String,
}

fn default_status() -> String {
    "pending".to_string()
}

/// The main entry point for the LinkedIn post scheduling server.
/// This server listens for incoming HTTP POST requests containing post data,
/// validates and schedules the posts by storing them in a MongoDB collection.
///
/// # Functionality
///
/// 1. **Logging Initialization**: The function initializes the logging environment
///    using `env_logger` to provide runtime information for monitoring and debugging.
///
/// 2. **MongoDB Connection**: Establishes a connection to a MongoDB database, specifically
///    targeting the `lkdin-posts` database and the `posts` collection, where scheduled posts are stored.
///
/// 3. **CORS Configuration**: Configures Cross-Origin Resource Sharing (CORS) to allow requests
///    from any origin, with specific allowed methods (POST, GET, DELETE) and headers (Content-Type, Authorization).
///
/// 4. **HTTP Server Setup**: The server is set up using the `warp` framework to handle HTTP POST requests
///    at the `/schedule` endpoint:
///
///    - The server expects the body of the request to be in JSON format, representing the post data.
///    - The incoming post data is validated and converted into a BSON document for MongoDB storage.
///    - The post's `scheduled_time` is parsed from an RFC 3339 string format into a BSON datetime object.
///    - If the date format is invalid, the server responds with a `400 Bad Request` status.
///    - If the post is successfully scheduled, it's inserted into the MongoDB collection, and the server
///      responds with a `200 OK` status indicating that the post was scheduled successfully.
///
/// 5. **Running the Server**: The server is started on all available network interfaces, listening on port `8080`.
///
/// # Example Usage
///
/// To run this server, simply execute the compiled binary:
///
/// ```bash
/// cargo run --bin web_server
/// ```
///
/// The server will start running, and you can send POST requests to `http://localhost:8080/schedule`
/// to schedule posts.
///
/// # POST Data Format
///
/// The server expects the following JSON structure in the body of the POST request:
///
/// ```json
/// {
///   "title": "Post Title",
///   "content": "Post Content",
///   "scheduled_time": "2024-08-25T15:30:00Z",
///   "status": "pending"
/// }
/// ```
///
/// The `scheduled_time` field should be in RFC 3339 format (e.g., "2024-08-25T15:30:00Z").
///
/// # Errors
///
/// The server will return a `400 Bad Request` response if the `scheduled_time` is not in a valid
/// RFC 3339 format.
///
/// If there is an issue inserting the post into MongoDB, the server will log the error and may panic,
/// depending on the error handling strategy.
///
/// # Example Request
///
/// You can schedule a post using `curl`:
///
/// ```bash
/// curl -X POST http://localhost:8080/schedule \
/// -H "Content-Type: application/json" \
/// -d '{
///   "title": "My Post Title",
///   "content": "This is the content of the post.",
///   "scheduled_time": "2024-08-25T15:30:00Z",
///   "status": "pending"
/// }'
/// ```
#[tokio::main]
async fn main() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    let mongo_client = Client::with_uri_str("mongodb://localhost:27017").await.unwrap();
    let db = mongo_client.database("lkdin-posts");
    let posts = db.collection("posts");

    info!("Server running on http://localhost:8080/");

    let cors = cors()
        .allow_any_origin()
        .allow_methods(vec!["POST", "GET", "DELETE"])
        .allow_headers(vec!["Content-Type", "Authorization"])
        .build();

    info!("CORS configured.");

    let schedule_post = warp::post()
        .and(warp::path("schedule"))
        .and(warp::body::json())
        .and_then(move |post: Post| {
            let posts = posts.clone();
            async move {
                info!("Receiving a new post for scheduling: {:?}", post);

                let bson_dt = match BsonDateTime::parse_rfc3339_str(&post.scheduled_time) {
                    Ok(dt) => dt,
                    Err(e) => {
                        error!("Error parsing date: {}", e);
                        return Ok::<_, warp::Rejection>(warp::reply::with_status("Invalid date format", warp::http::StatusCode::BAD_REQUEST));
                    }
                };

                let doc = doc! {
                    "title": post.title,
                    "content": post.content,
                    "scheduled_time": bson_dt,
                    "status": post.status,
                };
                posts.insert_one(doc).await.unwrap();
                Ok::<_, warp::Rejection>(warp::reply::with_status("Post scheduled", warp::http::StatusCode::OK))
            }
        })
        .with(cors);

    warp::serve(schedule_post)
        .run(([0, 0, 0, 0], 8080))
        .await;
}
