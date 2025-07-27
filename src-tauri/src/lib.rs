use once_cell::sync::Lazy;
use std::sync::Mutex;
use tauri::Manager;
use tauri_plugin_shell::process::{CommandChild, CommandEvent};
use tauri_plugin_shell::ShellExt;

pub static MASTER_INSTANCE: Lazy<Mutex<Option<CommandChild>>> = Lazy::new(|| Mutex::new(None));

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            let app_handle = app.app_handle().clone();
            println!("Starting PocketBase...");
            tauri::async_runtime::spawn(async move {
                let data_dir = app_handle.path().app_data_dir().unwrap().join("pb_data");
                std::fs::create_dir_all(&data_dir).ok();
                println!("Using PocketBase data directory: {:?}", data_dir);

                let sidecar = app_handle.shell().sidecar("pocketbase").unwrap().args([
                    "serve",
                    "--dir",
                    data_dir.to_str().unwrap(),
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

            Ok(())
        })
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
