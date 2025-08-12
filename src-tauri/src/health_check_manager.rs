use crate::types::PocketBaseProject;
use pocketbase_sdk::client::Client;
use std::{collections::HashMap, sync::Arc, time::Duration};
use tauri::async_runtime::{JoinHandle, RwLock};
use tokio::time::interval;

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

        // Spawn a task that polls the health endpoint periodically
        let handle = tauri::async_runtime::spawn(async move {
            let mut ticker = interval(check_interval);
            loop {}
        });
    }
}
