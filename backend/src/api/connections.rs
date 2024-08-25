use reqwest::Client;
use reqwest::header::{HeaderMap, AUTHORIZATION};
use serde_json::Value;


/// Retrieves the LinkedIn profile ID of the authenticated user.
///
/// This function sends a GET request to the LinkedIn API's `/userinfo` endpoint to fetch the user's profile information.
/// It extracts the `sub` field from the response, which corresponds to the user's profile ID.
///
/// # Arguments
///
/// * `access_token` - A string slice containing the LinkedIn API access token. This token must have the necessary permissions to retrieve user information.
///
/// # Returns
///
/// This function returns a `Result<String, Box<dyn std::error::Error>>`.
/// * If successful, it returns the profile ID as a `String`.
/// * If unsuccessful, it returns an `Err` with the associated error message.
///
/// # Errors
///
/// This function will return an error if:
/// * The LinkedIn API request fails (e.g., due to network issues).
/// * The API responds with a failure status code (e.g., if the access token is invalid).
/// * The `sub` field is not found in the response, indicating that the profile ID could not be retrieved.
///
/// # Example
///
/// ```rust
/// let access_token = "your_access_token";
/// let profile_id = get_profile_id(&access_token).await?;
/// println!("Profile ID: {}", profile_id);
/// ```
pub async fn get_profile_id(access_token: &str) -> Result<String, Box<dyn std::error::Error>> {
    let client = Client::new();
    let url = "https://api.linkedin.com/v2/userinfo";

    let mut headers = HeaderMap::new();
    headers.insert(AUTHORIZATION, format!("Bearer {}", access_token).parse()?);

    println!("Sending request to {}", url);
    let response = client.get(url)
        .headers(headers)
        .send()
        .await?;

    println!("Response status: {}", response.status());

    if response.status().is_success() {
        let profile_info: Value = response.json().await?;
        println!("User info: {:?}", profile_info);

        let profile_id = profile_info["sub"].as_str().unwrap_or("").to_string();
        if profile_id.is_empty() {
            Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, "Profile ID not found")))
        } else {
            println!("Profile ID: {}", profile_id);
            Ok(profile_id)
        }
    } else {
        let error_text = response.text().await?;
        println!("Failed to retrieve user info: {:?}", error_text);
        Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, format!("Failed to retrieve user info: {}", error_text))))
    }
}