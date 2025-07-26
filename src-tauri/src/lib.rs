use tauri::Manager;
use tauri_plugin_shell::process::CommandEvent;
use tauri_plugin_shell::ShellExt;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_shell::init())
        .setup(|app| {
            let app_handle = app.app_handle().clone();
            tauri::async_runtime::spawn(async move {
                let sidecar = app_handle.shell().sidecar("pocketbase").unwrap();
                let (mut rx, _child) = sidecar.spawn().expect("Failed to start PocketBase");

                while let Some(event) = rx.recv().await {
                    if let CommandEvent::Stdout(line) = event {
                        println!("{}", String::from_utf8_lossy(&line));
                    }
                }
                // No need to wait or kill here
            });

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
