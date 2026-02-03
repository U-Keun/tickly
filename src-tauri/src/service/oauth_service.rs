use std::sync::Arc;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;
use tokio::sync::oneshot;

use super::supabase_client::{SupabaseAuthResponse, SupabaseClient};

/// OAuth state for managing the PKCE flow
pub struct OAuthState {
    pub code_verifier: String,
    pub redirect_url: String,
}

pub struct OAuthService;

impl OAuthService {
    /// Start the Google OAuth flow
    /// Returns (oauth_url, code_verifier, port)
    pub async fn start_google_oauth(
        client: &SupabaseClient,
    ) -> Result<(String, String, u16), String> {
        // Find an available port
        let listener = TcpListener::bind("127.0.0.1:0")
            .await
            .map_err(|e| format!("Failed to bind to port: {}", e))?;

        let port = listener
            .local_addr()
            .map_err(|e| format!("Failed to get local address: {}", e))?
            .port();

        let redirect_url = format!("http://127.0.0.1:{}/callback", port);

        // Generate PKCE codes
        let code_verifier = SupabaseClient::generate_code_verifier();
        let code_challenge = SupabaseClient::generate_code_challenge(&code_verifier);

        // Get OAuth URL
        let oauth_url = client.get_google_oauth_url(&redirect_url, &code_challenge);

        // Keep the listener alive by spawning a task
        // The actual handling will be done by wait_for_callback
        drop(listener);

        Ok((oauth_url, code_verifier, port))
    }

    /// Wait for the OAuth callback and extract the authorization code
    pub async fn wait_for_callback(port: u16, timeout_secs: u64) -> Result<String, String> {
        let listener = TcpListener::bind(format!("127.0.0.1:{}", port))
            .await
            .map_err(|e| format!("Failed to bind to port {}: {}", port, e))?;

        let (tx, rx) = oneshot::channel::<Result<String, String>>();
        let tx = Arc::new(tokio::sync::Mutex::new(Some(tx)));

        // Spawn a task to handle the callback
        let tx_clone = tx.clone();
        tokio::spawn(async move {
            match tokio::time::timeout(
                std::time::Duration::from_secs(timeout_secs),
                listener.accept(),
            )
            .await
            {
                Ok(Ok((mut socket, _))) => {
                    let mut buffer = [0u8; 4096];
                    if let Ok(n) = socket.read(&mut buffer).await {
                        let request = String::from_utf8_lossy(&buffer[..n]);

                        // Parse the request to extract the code
                        let result = Self::extract_code_from_request(&request);

                        // Send response to browser
                        let response = match &result {
                            Ok(_) => {
                                "HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=utf-8\r\n\r\n\
                                <html><body style='font-family: system-ui; text-align: center; padding-top: 50px;'>\
                                <h1>Login Successful!</h1>\
                                <p>You can close this window and return to Tickly.</p>\
                                <script>setTimeout(() => window.close(), 2000);</script>\
                                </body></html>"
                            }
                            Err(e) => {
                                let error_html = format!(
                                    "HTTP/1.1 400 Bad Request\r\nContent-Type: text/html; charset=utf-8\r\n\r\n\
                                    <html><body style='font-family: system-ui; text-align: center; padding-top: 50px;'>\
                                    <h1>Login Failed</h1>\
                                    <p>{}</p>\
                                    </body></html>",
                                    e
                                );
                                &error_html.clone()
                            }
                        };

                        let _ = socket.write_all(response.as_bytes()).await;
                        let _ = socket.flush().await;

                        if let Some(tx) = tx_clone.lock().await.take() {
                            let _ = tx.send(result);
                        }
                    }
                }
                Ok(Err(e)) => {
                    if let Some(tx) = tx_clone.lock().await.take() {
                        let _ = tx.send(Err(format!("Accept failed: {}", e)));
                    }
                }
                Err(_) => {
                    if let Some(tx) = tx_clone.lock().await.take() {
                        let _ = tx.send(Err("OAuth callback timeout".to_string()));
                    }
                }
            }
        });

        rx.await.map_err(|_| "Channel closed".to_string())?
    }

    /// Extract the authorization code from the HTTP request
    fn extract_code_from_request(request: &str) -> Result<String, String> {
        // Parse the first line to get the path
        let first_line = request.lines().next().ok_or("Empty request")?;
        let parts: Vec<&str> = first_line.split_whitespace().collect();

        if parts.len() < 2 {
            return Err("Invalid request format".to_string());
        }

        let path = parts[1];

        // Parse query parameters
        if let Some(query_start) = path.find('?') {
            let query = &path[query_start + 1..];

            for param in query.split('&') {
                let kv: Vec<&str> = param.splitn(2, '=').collect();
                if kv.len() == 2 {
                    if kv[0] == "code" {
                        return Ok(kv[1].to_string());
                    }
                    if kv[0] == "error" {
                        let error_desc = query
                            .split('&')
                            .find(|p| p.starts_with("error_description="))
                            .and_then(|p| p.strip_prefix("error_description="))
                            .unwrap_or(kv[1]);
                        return Err(format!(
                            "OAuth error: {}",
                            urlencoding::decode(error_desc).unwrap_or_else(|_| error_desc.into())
                        ));
                    }
                }
            }
        }

        Err("No authorization code in callback".to_string())
    }

    /// Complete the OAuth flow by exchanging the code for tokens
    pub async fn complete_oauth(
        client: &SupabaseClient,
        code: &str,
        code_verifier: &str,
    ) -> Result<SupabaseAuthResponse, String> {
        client.exchange_code_for_token(code, code_verifier).await
    }
}

mod urlencoding {
    use std::borrow::Cow;

    pub fn decode(s: &str) -> Result<Cow<str>, ()> {
        let mut result = String::with_capacity(s.len());
        let mut chars = s.chars().peekable();

        while let Some(c) = chars.next() {
            if c == '%' {
                let hex: String = chars.by_ref().take(2).collect();
                if hex.len() == 2 {
                    if let Ok(byte) = u8::from_str_radix(&hex, 16) {
                        result.push(byte as char);
                        continue;
                    }
                }
                result.push('%');
                result.push_str(&hex);
            } else if c == '+' {
                result.push(' ');
            } else {
                result.push(c);
            }
        }

        Ok(Cow::Owned(result))
    }
}
