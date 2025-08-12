use crate::types::PocketBaseProject;
use reqwest::Client;
use std::{collections::HashMap, sync::Arc, time::Duration};
use tauri::async_runtime::{JoinHandle, RwLock};
use tokio::time::interval;

/// Callback type: (project_id, is_healthy) -> ()
/// Called whenever the observed health changes (including first observation).
pub type HealthCallback = Arc<dyn Fn(String, bool) + Send + Sync + 'static>;

#[derive(Clone)]
pub struct HealthCheckManager {
    client: Client,
    /// Map project_id -> JoinHandle for the monitor task
    handles: Arc<RwLock<HashMap<String, JoinHandle<()>>>>,
    /// Map project_id -> last known health state
    state: Arc<RwLock<HashMap<String, bool>>>,
    health_check_callback: HealthCallback,
}

impl HealthCheckManager {
    pub fn new(client: Client, health_check_callback: HealthCallback) -> Self {
        Self {
            client,
            handles: Arc::new(RwLock::new(HashMap::new())),
            state: Arc::new(RwLock::new(HashMap::new())),
            health_check_callback,
        }
    }

    /// Start monitoring a project. If a monitor for this `project.id` already exists, this is a no-op.
    /// `check_interval` controls how often the health endpoint is polled.
    pub async fn start_monitoring(&self, project: PocketBaseProject, check_interval: Duration) {
        let project_id = project.pid.clone();

        // don't start another monitor if one exists
        {
            let handles = self.handles.read().await;
            if handles.contains_key(&project_id) {
                return;
            }
        }

        let client = self.client.clone();
        let state_map = self.state.clone();
        let health_check_callback = self.health_check_callback.clone();

        // need to reference project_id in the async block
        let project_id_clone = project_id.clone();

        // Spawn a task that polls the health endpoint periodically
        let handle = tauri::async_runtime::spawn(async move {
            let mut ticker = interval(check_interval);
            loop {
                ticker.tick().await;

                let url = format!("http://127.0.0.1:{}/api/health", project.port);
                let is_healthy = match client.get(&url).send().await {
                    Ok(resp) => resp.status().is_success(),
                    Err(_) => false,
                };

                // update last-known state and call callback only on change
                let mut state = state_map.write().await;
                match state.get(&project_id) {
                    Some(prev) if *prev == is_healthy => continue,
                    _ => {
                        // changed or first time
                        state.insert(project_id.clone(), is_healthy);
                        // Invoke user-provided callback (non-blocking)
                        // Note: callback should be fast or offload heavy work inside callback.
                        (health_check_callback)(project_id.clone(), is_healthy);
                    }
                }
            }
        });

        // store the handle
        let mut handles = self.handles.write().await;
        handles.insert(project_id_clone, handle);
    }
}
