use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::{Mutex, mpsc};
use crate::api::oauth_server::{AuthCode, start_server};

#[derive(Serialize, Deserialize, Debug)]
struct TokenResponse {
    access_token: String,
    expires_in: u64,
}
/// Obtains an OAuth 2.0 access token from LinkedIn using the Authorization Code flow.
///
/// This function starts a local server to receive the authorization code from LinkedIn,
/// and then exchanges that code for an access token.
///
/// # Arguments
///
/// * `client_id` - The client ID obtained from LinkedIn's developer portal.
/// * `client_secret` - The client secret associated with the client ID.
/// * `redirect_uri` - The redirect URI configured in the LinkedIn app settings.
///
/// # Returns
///
/// Returns a `Result<String, Box<dyn std::error::Error>>`, where:
/// * `Ok(String)` contains the access token.
/// * `Err` contains the error message if the process fails.
///
/// # Errors
///
/// This function may return errors if:
/// * The authorization URL is not visited and authorized by the user.
/// * The exchange of the authorization code for an access token fails.
/// * The local server fails to capture the authorization code.
///
/// # Example
///
/// ```rust
/// let access_token = get_access_token("your_client_id", "your_client_secret", "your_redirect_uri").await?;
/// println!("Access Token: {}", access_token);
/// ```
pub async fn get_access_token(client_id: &str, client_secret: &str, redirect_uri: &str) -> Result<String, Box<dyn std::error::Error>> {
    let (tx, mut rx) = mpsc::channel(1);
    let auth_code = AuthCode(Arc::new(Mutex::new(None)));

    tokio::spawn(async move {
        start_server(auth_code, tx).await;
    });

    let auth_url = format!(
        "https://www.linkedin.com/oauth/v2/authorization?response_type=code&client_id={}&redirect_uri={}&scope=openid%20profile%20email%20w_member_social",
        client_id, redirect_uri
    );

    println!("Por favor, visite esta URL para autorizar a aplicação:");
    println!("{}", auth_url);
    println!("Esperando por autorização...");

    let auth_code = rx.recv().await.ok_or("Falha ao receber o código de autorização")?;

    let client = Client::new();
    let params = [
        ("grant_type", "authorization_code"),
        ("code", &auth_code),
        ("client_id", client_id),
        ("client_secret", client_secret),
        ("redirect_uri", redirect_uri),
    ];

    let response = client.post("https://www.linkedin.com/oauth/v2/accessToken")
        .form(&params)
        .send()
        .await?;

    if response.status().is_success() {
        let token_response: TokenResponse = response.json().await?;
        println!("Token de acesso obtido: {}", token_response.access_token);
        Ok(token_response.access_token)
    } else {
        Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("Falha ao obter o token de acesso: {:?}", response.text().await?),
        )))
    }
}
