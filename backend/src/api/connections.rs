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
pub async fn get_profile_id(access_token: &str, base_url: Option<String>) -> Result<String, Box<dyn std::error::Error>> {
    let client = Client::new();
    let url = format!("{}/v2/userinfo", base_url.unwrap_or_else(|| "https://api.linkedin.com".to_string()));

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

#[cfg(test)]
mod tests {
    use super::*;
    use mockito::Server;
    use std::sync::Once;

    static INIT: Once = Once::new();

    /// Initializes the test environment by setting up a logger for test cases.
///
/// This function is called once at the beginning of the test suite to ensure that the logger is properly configured for testing purposes.
///
/// # Arguments
///
/// This function does not take any arguments.
///
/// # Returns
///
/// This function does not return any value.
///
/// # Example
///
/// ```rust
/// initialize();
/// ```
pub fn initialize() {
    INIT.call_once(|| {
        let _ = env_logger::builder().is_test(true).try_init();
    });
}

    /// Tests the successful retrieval of a profile ID.
    ///
    /// This test ensures that the `get_profile_id` function correctly handles a successful response from the LinkedIn API.
    ///
    /// # Steps
    ///
    /// 1. Initializes the test environment.
    /// 2. Creates a mock server to simulate the LinkedIn API.
    /// 3. Sets up a mock response for a successful profile ID retrieval.
    /// 4. Calls the `get_profile_id` function with the mock server URL.
    /// 5. Asserts that the mock was called and the correct profile ID was returned.
    ///
    /// # Returns
    ///
    /// This function does not return any value.
    ///
    /// # Panics
    ///
    /// This test will panic if the assertions fail, indicating that the `get_profile_id` function did not behave as expected.
    #[tokio::test]
    async fn test_get_profile_id_success() {
        initialize();

        let mut server = Server::new_async().await;

        let mock = server.mock("GET", "/v2/userinfo")
            .match_header("Authorization", mockito::Matcher::Regex("Bearer .+".to_string()))
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"sub": "mock-profile-id"}"#)
            .create_async()
            .await;

        let result = get_profile_id("mock-token", Some(server.url())).await;

        mock.assert_async().await;
        assert_eq!(result.unwrap(), "mock-profile-id");
    }

    /// Tests the failure scenario when retrieving a profile ID.
    ///
    /// This test ensures that the `get_profile_id` function correctly handles an error response from the LinkedIn API.
    ///
    /// # Steps
    ///
    /// 1. Initializes the test environment.
    /// 2. Creates a mock server to simulate the LinkedIn API.
    /// 3. Sets up a mock response for a failed profile ID retrieval.
    /// 4. Calls the `get_profile_id` function with the mock server URL.
    /// 5. Asserts that the mock was called and an error was returned.
    ///
    /// # Returns
    ///
    /// This function does not return any value.
    ///
    /// # Panics
    ///
    /// This test will panic if the assertions fail, indicating that the `get_profile_id` function did not handle the error scenario as expected.

    #[tokio::test]
    async fn test_get_profile_id_failure() {
        initialize();

        let mut server = Server::new_async().await;

        let mock = server.mock("GET", "/v2/userinfo")
            .match_header("Authorization", mockito::Matcher::Regex("Bearer .+".to_string()))
            .with_status(500)
            .with_body("Internal Server Error")
            .create_async()
            .await;

        let result = get_profile_id("mock-token", Some(server.url())).await;

        mock.assert_async().await;
        assert!(result.is_err());
    }
}