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
        Ok(project.pid.clone())
    }

    pub async fn resume_project(
        &self,
        project_name: String,
        pid: String,
    ) -> Result<(String, String, String), String> {
        let mut project = self
            .client
            .records("projects")
            .view(&project_name)
            .call::<PocketBaseProject>()
            .unwrap();
        project.status = ProjectStatus::Running;
        project.pid = pid;
        self.client
            .records("projects")
            .update(&project_name, project.clone())
            .call()
            .unwrap();
        let pid = project.clone().pid.clone();
        let data_dir = project.clone().data_directory.clone().unwrap_or_default();
        let port = project.clone().port.to_string();
        Ok((pid, data_dir, port))
    }

    pub async fn get_project(&self, project_name: String) -> Result<(String, String), String> {
        let project = self
            .client
            .records("projects")
            .view(&project_name)
            .call::<PocketBaseProject>()
            .unwrap();
        Ok((project.data_directory.unwrap_or_default(), project.pid))
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
