use once_cell::sync::Lazy;
use std::sync::Mutex;
use tauri::{AppHandle, Manager};
use tauri_plugin_shell::process::{CommandChild, CommandEvent};
use tauri_plugin_shell::ShellExt;
pub mod project_manager;
pub mod types;
use crate::project_manager::ProjectManager;
use crate::types::{PocketBaseProject, ProjectStatus};
use pocketbase_sdk::client::Client as PocketBaseClient;

pub static MASTER_INSTANCE: Lazy<Mutex<Option<CommandChild>>> = Lazy::new(|| Mutex::new(None));

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            dotenvy::dotenv().unwrap();
            let app_handle = app.app_handle().clone();
            println!("Starting PocketBase...");
            tauri::async_runtime::spawn(async move {
                let data_dir = app_handle
                    .path()
                    .app_data_dir()
                    .unwrap()
                    .join("pb_data/master");
                std::fs::create_dir_all(&data_dir).ok();
                println!("Using PocketBase data directory: {:?}", data_dir);

                let sidecar = app_handle.shell().sidecar("pocketbase").unwrap().args([
                    "serve",
                    "--dir",
                    data_dir.to_str().unwrap(),
                    "--http",
                    "127.0.0.1:8090",
                ]);
                println!("PocketBase sidecar: {:?}", sidecar);
                let (mut rx, child) = sidecar.spawn().expect("Failed to start PocketBase");

                *MASTER_INSTANCE.lock().unwrap() = Some(child);

                while let Some(event) = rx.recv().await {
                    match event {
                        CommandEvent::Stdout(line) => {
                            println!("[PB STDOUT] {}", String::from_utf8_lossy(&line));
                        }
                        CommandEvent::Stderr(line) => {
                            eprintln!("[PB STDERR] {}", String::from_utf8_lossy(&line));
                        }
                        other => {
                            println!("[PB EVENT] {:?}", other);
                        }
                    }
                } // No need to wait or kill here
            });
            let email = std::env::var("MASTER_EMAIL").unwrap_or("master@example.com".to_string());
            let password = std::env::var("MASTER_PASSWORD").unwrap_or("masterpassword".to_string());
            app.manage(ProjectManager::new(
                PocketBaseClient::new("http://localhost:8090")
                    .auth_with_password("users", email.as_str(), password.as_str())
                    .unwrap(),
            ));

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![start_pocketbase_instance,])
        .build(tauri::generate_context!())
        .expect("error while building tauri application")
        .run(|_app_handle, event| match event {
            tauri::RunEvent::ExitRequested { .. } => {
                println!("App is exiting, Closing Master Instance...");
                if let Some(child) = MASTER_INSTANCE.lock().unwrap().take() {
                    let _ = child.kill();
                }
            }
            _ => {}
        })
}

#[tauri::command]
async fn start_pocketbase_instance(
    app_handle: AppHandle,
    project_name: String,
) -> Result<(), String> {
    let data_dir = app_handle
        .path()
        .app_data_dir()
        .unwrap()
        .join(format!("pb_data/{}", project_name));
    std::fs::create_dir_all(&data_dir).map_err(|e| e.to_string())?;
    let port = portpicker::pick_unused_port().ok_or("No available port found")?;
    let sidecar = app_handle.shell().sidecar("pocketbase").unwrap().args([
        "serve",
        "--dir",
        data_dir.to_str().unwrap(),
        "--http",
        format!("127.0.0.1:{}", port).as_str(),
    ]);
    println!("Starting PocketBase instance: {:?}", sidecar);
    let (mut _rx, mut _child) = sidecar.spawn().unwrap();
    let project_manager = app_handle.state::<ProjectManager>();
    let project = PocketBaseProject {
        id: project_name.clone(),
        name: project_name.clone(),
        port,
        status: ProjectStatus::Running,
        is_healthy: true,
        data_directory: Some(data_dir.to_str().unwrap().to_string()),
        created_at: chrono::Utc::now(),
        last_started: None,
        logs: vec![],
    };
    project_manager
        .start_project(project)
        .await
        .map_err(|e| e.to_string())
        .unwrap();
    Ok(())
}
