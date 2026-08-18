use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: u32,
    pub name: String,
    pub email: String,
    pub role: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: u32,
    pub title: String,
    pub status: String,
    #[serde(rename = "userId")]
    pub user_id: u32,
}

#[derive(Debug, Serialize)]
pub struct UsersResponse {
    pub users: Vec<User>,
    pub count: usize,
}

#[derive(Debug, Serialize)]
pub struct TasksResponse {
    pub tasks: Vec<Task>,
    pub count: usize,
}

#[derive(Debug, Serialize)]
pub struct StatsUsers {
    pub total: usize,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StatsTasks {
    pub total: usize,
    pub pending: usize,
    pub in_progress: usize,
    pub completed: usize,
}

#[derive(Debug, Serialize)]
pub struct StatsResponse {
    pub users: StatsUsers,
    pub tasks: StatsTasks,
}

#[derive(Debug, Serialize)]
pub struct HealthResponse {
    pub status: String,
    pub message: String,
}
