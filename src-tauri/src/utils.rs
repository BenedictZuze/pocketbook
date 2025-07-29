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
            "fields": [
                { "name": "name", "type": "text", "required": true },
                { "name": "port", "type": "number", "required": true },
                { "name": "status", "type": "select", "options": { "values": ["running", "stopped"] }, "required": false },
                { "name": "isHealthy", "type": "bool", "required": false },
                { "name": "pid", "type": "text", "required": true },
                { "name": "dataDirectory", "type": "text", "required": true },
                { "name": "createdAt", "type": "autodate", "required": false, "onCreate": true },
                { "name": "lastStarted", "type": "autodate", "required": false, "onCreate": true, "onUpdate": true }
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
