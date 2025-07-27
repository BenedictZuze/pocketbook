use pocketbase_sdk::client::Client as PocketBaseClient;

#[derive(Debug, Clone)]
pub struct ProjectManager {
    pub client: PocketBaseClient,
}

impl ProjectManager {
    pub fn new(client: PocketBaseClient) -> Self {
        ProjectManager { client }
    }
}
