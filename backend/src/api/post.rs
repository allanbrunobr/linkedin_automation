use reqwest::Client;
use reqwest::header::{HeaderMap, AUTHORIZATION, CONTENT_TYPE};
use log::{info, error};
use std::path::Path;
use std::fs::File;
use std::io::Read;


/// Publishes an article on LinkedIn using the provided access token and profile ID.
///
/// # Arguments
///
/// * `access_token` - A string slice that holds the LinkedIn API access token. This token must have the necessary permissions to publish content on behalf of the user.
/// * `profile_id` - A string slice that holds the LinkedIn profile ID of the author of the article.
/// * `title` - A string slice that contains the title of the article to be published.
/// * `_content` - A string slice that contains the body of the article. Although currently not used, this parameter is meant for future expansion where article content can be directly embedded.
///
/// # Returns
///
/// This function returns a `Result<(), Box<dyn std::error::Error>>`.
/// * If the article is published successfully, it returns `Ok(())`.
/// * If the publication fails, it returns an `Err` with the associated error message.
///
/// # Errors
///
/// This function will return an error if:
/// * The LinkedIn API request fails (e.g., due to network issues).
/// * The API responds with a failure status code (e.g., if the access token is invalid or the request body is incorrect).
///
/// # Example
///
/// ```rust
/// let access_token = "your_access_token";
/// let profile_id = "your_profile_id";
/// let title = "My First Article with Rust and LinkedIn API";
/// let content = "<h1>This is a Heading</h1><p>This is a paragraph of the article.</p>";
///
/// publish_article(access_token, profile_id, title, content).await?;
/// ```
pub async fn publish_article(access_token: &str, profile_id: &str, title: &str, _content: &str) -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    let url = "https://api.linkedin.com/v2/ugcPosts";

    let mut headers = HeaderMap::new();
    headers.insert(AUTHORIZATION, format!("Bearer {}", access_token).parse()?);
    headers.insert(CONTENT_TYPE, "application/json".parse()?);

    let body = serde_json::json!({
        "author": format!("urn:li:person:{}", profile_id),
        "lifecycleState": "PUBLISHED",
        "specificContent": {
            "com.linkedin.ugc.ShareContent": {
                "shareCommentary": {
                    "text": _content,
                },
                "shareMediaCategory": "NONE",
                "media": []
            }
        },
        "visibility": {
            "com.linkedin.ugc.MemberNetworkVisibility": "PUBLIC"
        }
    });

    info!("Sending POST request to LinkedIn with body: {:?}", body);

    let response = client.post(url)
        .headers(headers)
        .json(&body)
        .send()
        .await?;

    if response.status().is_success() {
        info!("Article published successfully!");
        Ok(())
    } else {
        println!("Failed to publish article: {:?}", response.text().await?);
        Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, "Failed to publish article")))
    }
}