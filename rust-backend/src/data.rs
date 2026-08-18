use std::sync::RwLock;
use crate::models::{StatsResponse, StatsUsers, StatsTasks, Task, User};

/// In-memory data store, equivalent to the Rust DataStore struct.
pub struct DataStore {
    users: RwLock<Vec<User>>,
    tasks: RwLock<Vec<Task>>,
}

impl DataStore {
    /// Initialise the store with the same seed data as the Rust backend.
    pub fn new() -> Self {
        let users = vec![
            User { id: 1, name: "John Doe".into(),    email: "john@example.com".into(), role: "developer".into() },
            User { id: 2, name: "Jane Smith".into(),  email: "jane@example.com".into(), role: "designer".into()  },
            User { id: 3, name: "Bob Johnson".into(), email: "bob@example.com".into(),  role: "manager".into()   },
        ];

        let tasks = vec![
            Task { id: 1, title: "Implement authentication".into(), status: "pending".into(),     user_id: 1 },
            Task { id: 2, title: "Design user interface".into(),    status: "in-progress".into(), user_id: 2 },
            Task { id: 3, title: "Review code changes".into(),      status: "completed".into(),   user_id: 3 },
        ];

        DataStore {
            users: RwLock::new(users),
            tasks: RwLock::new(tasks),
        }
    }

    pub fn get_users(&self) -> Vec<User> {
        self.users.read().unwrap().clone()
    }

    pub fn get_user_by_id(&self, id: u32) -> Option<User> {
        self.users
            .read()
            .unwrap()
            .iter()
            .find(|u| u.id == id)
            .cloned()
    }

    /// Filter tasks by optional `status` and/or `user_id` query params.
    pub fn get_tasks(&self, status: Option<&str>, user_id: Option<u32>) -> Vec<Task> {
        self.tasks
            .read()
            .unwrap()
            .iter()
            .filter(|t| {
                let match_status = status.map_or(true, |s| t.status == s);
                let match_user   = user_id.map_or(true, |id| t.user_id == id);
                match_status && match_user
            })
            .cloned()
            .collect()
    }

    pub fn get_stats(&self) -> StatsResponse {
        let users = self.users.read().unwrap();
        let tasks = self.tasks.read().unwrap();

        let mut pending = 0usize;
        let mut in_progress = 0usize;
        let mut completed = 0usize;

        for task in tasks.iter() {
            match task.status.as_str() {
                "pending"     => pending     += 1,
                "in-progress" => in_progress += 1,
                "completed"   => completed   += 1,
                _ => {}
            }
        }

        StatsResponse {
            users: StatsUsers { total: users.len() },
            tasks: StatsTasks {
                total: tasks.len(),
                pending,
                in_progress,
                completed,
            },
        }
    }
}
