use futures_util::TryStreamExt;
use log::{error, info};
use mongodb::{
    bson::{doc, DateTime as BsonDateTime},
    Client,
};
use serde::{Deserialize, Serialize};
use std::fmt;
use std::sync::Arc;
use warp::cors;
use warp::reject::Reject;
use warp::Filter;

#[derive(Debug, Deserialize, Serialize)]
struct Post {
    title: String,
    content: String,
    scheduled_time: String,
    #[serde(default = "default_status")]
    status: String,
}

#[derive(Debug, Deserialize)]
struct PostQueryParams {
    start_date: String,
    end_date: String,
}

#[derive(Debug)]
struct ParseDateError;

impl fmt::Display for ParseDateError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Failed to parse date")
    }
}

impl Reject for ParseDateError {}
fn default_status() -> String {
    "pending".to_string()
}

#[tokio::main]
async fn main() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    let mongo_client = Client::with_uri_str("mongodb://localhost:27017")
        .await
        .unwrap();
    let db = mongo_client.database("lkdin-posts");
    let posts = Arc::new(db.collection("posts"));

    info!("Server running on http://localhost:8080/");

    let cors = cors()
        .allow_any_origin()
        .allow_methods(vec!["POST", "GET", "DELETE", "PUT"])
        .allow_headers(vec!["Content-Type", "Authorization"])
        .build();

    info!("CORS configured.");

    let schedule_post = {
        let posts = Arc::clone(&posts);
        warp::post()
            .and(warp::path("schedule"))
            .and(warp::body::json())
            .and_then(move |post: Post| {
                let posts = Arc::clone(&posts);
                async move {
                    info!("Receiving a new post for scheduling: {:?}", post);

                    let bson_dt = match BsonDateTime::parse_rfc3339_str(&post.scheduled_time) {
                        Ok(dt) => dt,
                        Err(e) => {
                            error!("Error parsing date: {}", e);
                            return Ok::<_, warp::Rejection>(warp::reply::with_status(
                                "Invalid date format",
                                warp::http::StatusCode::BAD_REQUEST,
                            ));
                        }
                    };

                    let doc = doc! {
                        "title": post.title,
                        "content": post.content,
                        "scheduled_time": bson_dt,
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

    let query_posts = {
        let posts = Arc::clone(&posts);
        warp::get()
            .and(warp::path("posts"))
            .and(warp::query::<PostQueryParams>()) // Define query parameters for date range
            .and_then(move |params: PostQueryParams| {
                let posts = Arc::clone(&posts);
                async move {
                    let start_date_str = format!("{}T00:00:00.000Z", params.start_date);
                    let end_date_str = format!("{}T23:59:59.999Z", params.end_date);

                    match (
                        BsonDateTime::parse_rfc3339_str(&start_date_str),
                        BsonDateTime::parse_rfc3339_str(&end_date_str),
                    ) {
                        (Ok(start_dt), Ok(end_dt)) => {
                            let filter = doc! {
                                "scheduled_time": {
                                    "$gte": start_dt,
                                    "$lte": end_dt,
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
                        (Err(_), _) | (_, Err(_)) => {
                            error!("Error parsing date");
                            Err(warp::reject::custom(ParseDateError))
                        }
                    }
                }
            })
            .with(cors.clone())
    };

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

                    let bson_dt =
                        match BsonDateTime::parse_rfc3339_str(&updated_post.scheduled_time) {
                            Ok(dt) => dt,
                            Err(e) => {
                                error!(
                                    "Failed to parse scheduled_time {}: {:?}",
                                    updated_post.scheduled_time, e
                                );
                                return Ok::<_, warp::Rejection>(warp::reply::with_status(
                                    "Invalid date format",
                                    warp::http::StatusCode::BAD_REQUEST,
                                ));
                            }
                        };

                    let update_doc = doc! {
                        "$set": {
                            "title": updated_post.title,
                            "content": updated_post.content,
                            "scheduled_time": bson_dt,
                            "status": updated_post.status,
                        }
                    };

                    match posts
                        .update_one(doc! { "_id": object_id }, update_doc)
                        .await
                    {
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

    warp::serve(
        schedule_post
            .or(query_posts)
            .or(delete_post)
            .or(update_post),
    )
    .run(([0, 0, 0, 0], 8080))
    .await;
}
