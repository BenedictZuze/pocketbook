use once_cell::sync::Lazy;
use pocketbase_sdk::client::Client as PocketBaseClient;
use std::sync::Mutex;
use tauri::{AppHandle, Manager};
use tauri_plugin_shell::process::{CommandChild, CommandEvent};
use tauri_plugin_shell::ShellExt;
pub mod project_manager;
pub mod types;
pub mod utils;
use crate::project_manager::ProjectManager;
use crate::types::{PocketBaseProject, ProjectStatus};
use crate::utils::{create_projects_collection, kill_pid};

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

                let email =
                    std::env::var("MASTER_EMAIL").unwrap_or("master@example.com".to_string());
                let password =
                    std::env::var("MASTER_PASSWORD").unwrap_or("masterpassword".to_string());
                let upsert_cmd = app_handle.shell().sidecar("pocketbase").unwrap().args([
                    "superuser",
                    "upsert",
                    email.as_str(),
                    password.as_str(),
                    "--dir",
                    data_dir.to_str().unwrap(),
                ]);
                let (mut rx, _upsert_proc) = upsert_cmd
                    .spawn()
                    .expect("Failed to spawn superuser upsert");

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
                }

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

            // Set up projects collection in PocketBase
            tauri::async_runtime::spawn(async move {
                let admin_email =
                    std::env::var("MASTER_EMAIL").unwrap_or("master@example.com".to_string());
                let admin_password =
                    std::env::var("MASTER_PASSWORD").unwrap_or("masterpassword".to_string());
                let admin_client = PocketBaseClient::new("http://localhost:8090")
                    .auth_with_password(
                        "_superusers",
                        admin_email.as_str(),
                        admin_password.as_str(),
                    )
                    .unwrap();
                let projects_collection = admin_client
                    .records("projects")
                    .list()
                    .call::<PocketBaseProject>();
                if let Err(err) = projects_collection {
                    let err = err.to_string();
                    if err.contains("status code 404") {
                        println!("Projects collection not found, creating...");
                        let admin_token = admin_client.auth_token.unwrap();
                        create_projects_collection(&admin_token)
                            .await
                            .expect("Failed to create projects collection");
                    } else {
                        eprintln!("Error checking projects collection: {}", err);
                    }
                }
                println!("Projects collection is ready.");
            });

            // Authenticate with the master PocketBase instance
            let email = std::env::var("MASTER_EMAIL").unwrap_or("master@example.com".to_string());
            let password = std::env::var("MASTER_PASSWORD").unwrap_or("masterpassword".to_string());
            app.manage(ProjectManager::new(
                PocketBaseClient::new("http://localhost:8090")
                    .auth_with_password("users", email.as_str(), password.as_str())
                    .unwrap(),
            ));

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            start_pocketbase_instance,
            stop_pocketbase_instance,
            resume_pocketbase_instance,
        ])
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
    let (mut _rx, child) = sidecar.spawn().unwrap();
    let project_manager = app_handle.state::<ProjectManager>();
    let project = PocketBaseProject {
        name: project_name.clone(),
        port,
        status: ProjectStatus::Running,
        is_healthy: true,
        pid: child.pid().to_string(),
        data_directory: Some(data_dir.to_str().unwrap().to_string()),
        created_at: chrono::Utc::now(),
        last_started: None,
        // logs: vec![],
    };
    project_manager
        .start_project(project)
        .await
        .map_err(|e| e.to_string())
        .unwrap();
    Ok(())
}

#[tauri::command]
async fn stop_pocketbase_instance(
    app_handle: AppHandle,
    project_name: String,
) -> Result<(), String> {
    let project_manager = app_handle.state::<ProjectManager>();
    let pid = project_manager
        .stop_project(project_name)
        .await
        .map_err(|e| e.to_string())
        .unwrap();
    let pid: u32 = pid
        .parse()
        .map_err(|_| "Invalid PID format".to_string())
        .unwrap();
    if let Err(e) = kill_pid(pid) {
        print!("Failed to stop PocketBase process: {}", e);
        return Err(format!("Failed to stop PocketBase process: {}", e));
    }
    Ok(())
}

#[tauri::command]
async fn resume_pocketbase_instance(
    app_handle: AppHandle,
    project_name: String,
) -> Result<(), String> {
    let project_manager = app_handle.state::<ProjectManager>();
    let port = project_manager
        .get_project(project_name.clone())
        .await
        .map_err(|e| e.to_string())
        .unwrap();
    let data_dir = app_handle
        .path()
        .app_data_dir()
        .unwrap()
        .join(format!("pb_data/{}", project_name.clone()));
    let sidecar = app_handle.shell().sidecar("pocketbase").unwrap().args([
        "serve",
        "--dir",
        data_dir.to_str().unwrap(),
        "--http",
        format!("127.0.0.1:{}", port).as_str(),
    ]);
    println!("Starting PocketBase instance: {:?}", sidecar);
    let (mut _rx, child) = sidecar.spawn().unwrap();
    let pid = child.pid().to_string();
    project_manager
        .resume_project(project_name, pid)
        .await
        .map_err(|e| e.to_string())
        .unwrap();
    Ok(())
}
