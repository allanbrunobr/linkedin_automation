use std::sync::Arc;
use tokio::sync::{Mutex, mpsc};
use warp::Filter;
use warp::Rejection;

#[derive(Clone)]
pub struct AuthCode(pub Arc<Mutex<Option<String>>>);

/// Starts a local web server to handle the OAuth 2.0 authorization code callback.
///
/// This function sets up a Warp server that listens for incoming requests on the `/callback` path.
/// When the LinkedIn OAuth 2.0 flow redirects the user to this path with an authorization code,
/// the server extracts the code, stores it in the provided `AuthCode` structure, and sends it back
/// via the provided `mpsc::Sender`.
///
/// # Arguments
///
/// * `auth_code` - An `AuthCode` structure wrapped in `Arc` and `Mutex` that stores the received authorization code.
/// * `tx` - An `mpsc::Sender<String>` used to send the authorization code back to the calling function.
///
/// # Example
///
/// ```rust
/// let (tx, rx) = mpsc::channel(1);
/// let auth_code = AuthCode(Arc::new(Mutex::new(None)));
/// start_server(auth_code, tx).await;
/// ```
///
/// # Warp Filters
///
/// This function utilizes Warp filters to manage the request handling:
/// - `warp::path("callback")`: Matches the `/callback` path.
/// - `warp::query::raw()`: Extracts the raw query string from the request.
/// - `with_auth`: Provides the `AuthCode` structure to the filter chain.
/// - `with_tx`: Provides the `mpsc::Sender` to the filter chain.
///
/// # Behavior
///
/// - When a request with a query parameter containing the authorization code (e.g., `?code=...`) is received:
///   1. The code is extracted and stored in the `auth_code`.
///   2. The code is sent back via the `mpsc::Sender`.
///   3. A success message is displayed to the user.
///
/// - If the query parameter does not contain a code, a message indicating that no authorization code was found is returned.
///
/// # Errors
///
/// This function does not return errors directly, but if the server fails to run, it would typically panic.
///
/// # Network Binding
///
/// The server binds to `127.0.0.1` on port `3000`. Make sure this port is available on your machine.
///
/// # Notes
///
/// Ensure that your LinkedIn app is configured to redirect to `http://localhost:3000/callback`.
///
/// # Example Output
///
/// ```
/// Authorization code received: abc123xyz. You can close this window now.
/// ```
pub async fn start_server(auth_code: AuthCode, tx: mpsc::Sender<String>) {
    let auth_code = Arc::new(auth_code);
    let tx = Arc::new(tx);

    let with_auth = warp::any().map(move || auth_code.clone());
    let with_tx = warp::any().map(move || tx.clone());

    let callback = warp::path("callback")
        .and(warp::query::raw())
        .and(with_auth)
        .and(with_tx)
        .and_then(|query: String, auth_code: Arc<AuthCode>, tx: Arc<mpsc::Sender<String>>| async move {
            if let Some(code) = query.split("code=").nth(1) {
                let code = code.split('&').next().unwrap_or(code).to_string();
                let mut guard = auth_code.0.lock().await;
                *guard = Some(code.clone());
                tx.send(code.clone()).await.ok();
                Ok::<_, Rejection>(format!("Authorization code received: {}. You can close this window now.", code))
            } else {
                Ok::<_, Rejection>("No authorization code found in the request.".to_string())
            }
        });

    warp::serve(callback)
        .run(([127, 0, 0, 1], 3000))
        .await;
}