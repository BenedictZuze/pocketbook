use crate::types::{PocketBaseProject, ProjectStatus};
use pocketbase_sdk::client::{Auth, Client as PocketBaseClient};

#[derive(Debug, Clone)]
pub struct ProjectManager {
    pub client: PocketBaseClient<Auth>,
}

impl ProjectManager {
    pub fn new(client: PocketBaseClient<Auth>) -> Self {
        ProjectManager { client }
    }

    pub async fn start_project(&self, project: PocketBaseProject) -> Result<String, String> {
        let create_response = self.client.records("projects").create(project).call();
        match create_response {
            Ok(response) => return Ok(response.id),
            Err(err) => {
                println!("Failed to create project: {}", err);
                return Err(format!("Failed to create project: {}", err));
            }
        };
    }

    pub async fn stop_project(&self, project_name: String) -> Result<String, String> {
        let mut project = self
            .client
            .records("projects")
            .view(&project_name)
            .call::<PocketBaseProject>()
            .unwrap();
        project.status = ProjectStatus::Stopped;
        Ok(project.name.clone())
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
