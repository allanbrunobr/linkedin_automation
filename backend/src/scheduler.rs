use api::{connections::get_profile_id, post::publish_article};
use crate::config::settings::load_config;
use chrono::{Utc, DateTime, FixedOffset};
use mongodb::{Client, bson::{doc, Document, DateTime as BsonDateTime}};
use tokio::time::{self, Duration};
use futures_util::TryStreamExt;
use log::{info, error};

mod api;
mod config;

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
