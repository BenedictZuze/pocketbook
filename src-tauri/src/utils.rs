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

    let response = client
        .post("http://localhost:8090/api/collections")
        .header("Authorization", format!("Bearer {}", admin_token))
        .json(&json!({
            "name": "projects",
            "type": "base",
            "fields": [
            {
                "autogeneratePattern": "",
                "hidden": false,
                "max": 0,
                "min": 0,
                "name": "name",
                "pattern": "",
                "presentable": false,
                "primaryKey": false,
                "required": true,
                "system": false,
                "type": "text"
            },
            {
                "hidden": false,
                "max": null,
                "min": null,
                "name": "port",
                "onlyInt": true,
                "presentable": false,
                "required": true,
                "system": false,
                "type": "number"
            },
            {
                "hidden": false,
                "maxSelect": 1,
                "name": "status",
                "presentable": false,
                "required": false,
                "system": false,
                "type": "select",
                "values": [
                    "running",
                    "stopped"
                ]
            },
            {
                "hidden": false,
                "name": "isHealthy",
                "presentable": false,
                "required": false,
                "system": false,
                "type": "bool"
            },
            {
                "autogeneratePattern": "",
                "hidden": false,
                "max": 0,
                "min": 0,
                "name": "pid",
                "pattern": "",
                "presentable": false,
                "primaryKey": false,
                "required": true,
                "system": false,
                "type": "text"
            },
            {
                "autogeneratePattern": "",
                "hidden": false,
                "max": 0,
                "min": 0,
                "name": "dataDirectory",
                "pattern": "",
                "presentable": false,
                "primaryKey": false,
                "required": true,
                "system": false,
                "type": "text"
            },
            {
                "hidden": false,
                "name": "created",
                "onCreate": true,
                "onUpdate": false,
                "presentable": false,
                "system": false,
                "type": "autodate"
            },
            {
                "hidden": false,
                "name": "updated",
                "onCreate": true,
                "onUpdate": true,
                "presentable": false,
                "system": false,
                "type": "autodate"
            },
            {
                "hidden": false,
                "name": "createdAt",
                "onCreate": true,
                "onUpdate": false,
                "presentable": false,
                "system": false,
                "type": "autodate"
            },
            {
                "hidden": false,
                "name": "lastStarted",
                "onCreate": true,
                "onUpdate": true,
                "presentable": false,
                "system": false,
                "type": "autodate"
            }
        ],
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
