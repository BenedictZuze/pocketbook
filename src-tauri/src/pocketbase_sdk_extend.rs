// This is a temporary implementation of the PocketBase superuser login functionality
// pocketbase_sdk has the admin implementation, which is outdated as Pocketbase has moved to a superuser model

use anyhow::{anyhow, Result};
use reqwest::Client as HttpClient;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
struct SuperuserLogin {
    identity: String,
    password: String,
}

#[derive(Debug, Deserialize)]
struct AuthResponse {
    token: String,
}

pub async fn superuser_login(base_url: &str, identity: &str, password: &str) -> Result<String> {
    let client = HttpClient::new();
    let url = format!("{}/api/_superusers/auth-with-password", base_url);

    let login_payload = SuperuserLogin {
        identity: identity.to_string(),
        password: password.to_string(),
    };

    let res = client
        .post(&url)
        .json(&login_payload)
        .send()
        .await
        .map_err(|e| anyhow!("Request failed: {}", e))?;

    if !res.status().is_success() {
        let status = res.status();
        let body = res
            .text()
            .await
            .unwrap_or_else(|_| "<failed to read body>".into());
        return Err(anyhow!("Login failed: {} - {}", status, body));
    }

    let auth_response: AuthResponse = res.json().await?;
    Ok(auth_response.token)
}
