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

    let cors = warp::cors()
        .allow_any_origin()
        .allow_methods(&[Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_headers(vec!["Content-Type"]);


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



    warp::serve(
        schedule_post
            .or(query_posts)
            .or(delete_post)
            .or(update_post).with(cors),
    )
    .run(([0, 0, 0, 0], 8080))
    .await;
}
