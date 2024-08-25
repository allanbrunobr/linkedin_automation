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
