use reqwest::Client;
use serde_json::json;
use std::process::Command;

#[cfg(target_family = "unix")]
pub fn kill_pid(pid: u32) -> std::io::Result<()> {
    Command::new("kill")
        .arg("-9")
        .arg(pid.to_string())
        .status()?;
    Ok(())
}

#[cfg(target_family = "windows")]
pub fn kill_pid(pid: u32) -> std::io::Result<()> {
    Command::new("taskkill")
        .arg("/PID")
        .arg(pid.to_string())
        .arg("/F")
        .status()?;
    Ok(())
}

pub async fn create_projects_collection(
    admin_token: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();

    let response = client.post("http://localhost:8090/api/collections")
        .header("Authorization", format!("Bearer {}", admin_token))
        .json(&json!({
            "name": "projects",
            "type": "base",
            "schema": [
                { "name": "name", "type": "text", "required": true, "options": {} },
    { "name": "port", "type": "number", "required": true, "options": {} },
    { "name": "status", "type": "select", "options": { "values": ["running", "stopped"] }, "required": true },
    { "name": "isHealthy", "type": "bool", "options": {} },
    { "name": "pid", "type": "text", "options": {} },
    { "name": "dataDirectory", "type": "text", "options": {} },
    { "name": "createdAt", "type": "date", "options": {} },
    { "name": "lastStarted", "type": "date", "options": {} }
            ]
        }))
        .send()
        .await?;

    if response.status().is_success() {
        println!("✅ Collection created.");
    } else {
        let err_text = response.text().await?;
        println!("❌ Failed to create collection: {}", err_text);
    }

    Ok(())
}
