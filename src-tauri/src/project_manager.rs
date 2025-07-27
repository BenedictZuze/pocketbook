use crate::types::{PocketBaseProject, ProjectStatus};
use chrono::Utc;
use pocketbase_sdk::client::{Auth, Client as PocketBaseClient};

#[derive(Debug, Clone)]
pub struct ProjectManager {
    pub client: PocketBaseClient<Auth>,
}

impl ProjectManager {
    pub fn new(client: PocketBaseClient<Auth>) -> Self {
        ProjectManager { client }
    }

    pub async fn start_project(&self, project_name: String) -> Result<PocketBaseProject, String> {
        // Logic to start a project using the PocketBase client
        // This is a placeholder implementation
        Ok(PocketBaseProject {
            id: "project_id".to_string(),
            name: project_name,
            port: 8090,
            status: ProjectStatus::Running,
            is_healthy: true,
            data_directory: Some("data_directory".to_string()),
            created_at: Utc::now(),
            last_started: Some(Utc::now()),
            logs: vec![],
        })
    }

    pub async fn stop_project(&self, project_name: String) -> Result<PocketBaseProject, String> {
        // Logic to start a project using the PocketBase client
        // This is a placeholder implementation
        Ok(PocketBaseProject {
            id: "project_id".to_string(),
            name: project_name,
            port: 8090,
            status: ProjectStatus::Stopped,
            is_healthy: true,
            data_directory: Some("data_directory".to_string()),
            created_at: Utc::now(),
            last_started: Some(Utc::now()),
            logs: vec![],
        })
    }

    pub async fn list_projects(&self) -> Result<Vec<PocketBaseProject>, String> {
        let projects = self
            .client
            .records("projects")
            .list()
            .call::<PocketBaseProject>()
            .unwrap();
        let projects = projects.items;
        Ok(projects)
    }
}
