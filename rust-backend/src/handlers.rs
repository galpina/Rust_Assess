use actix_web::{web, HttpResponse, Responder};
use serde::Deserialize;

use crate::data::DataStore;
use crate::models::{HealthResponse, TasksResponse, UsersResponse};

// ── /health ──────────────────────────────────────────────────────────────────

pub async fn handle_health() -> impl Responder {
    HttpResponse::Ok().json(HealthResponse {
        status: "ok".into(),
        message: "Rust backend is running".into(),
    })
}

// ── /api/users ────────────────────────────────────────────────────────────────

pub async fn handle_users(store: web::Data<DataStore>) -> impl Responder {
    let users = store.get_users();
    let count = users.len();
    HttpResponse::Ok().json(UsersResponse { users, count })
}

// ── /api/users/{id} ───────────────────────────────────────────────────────────

pub async fn handle_user_by_id(
    store: web::Data<DataStore>,
    path: web::Path<u32>,
) -> impl Responder {
    let id = path.into_inner();
    match store.get_user_by_id(id) {
        Some(user) => HttpResponse::Ok().json(user),
        None => HttpResponse::NotFound().body("User not found"),
    }
}

// ── /api/tasks ────────────────────────────────────────────────────────────────

#[derive(Deserialize)]
pub struct TaskQuery {
    pub status: Option<String>,
    #[serde(rename = "userId")]
    pub user_id: Option<u32>,
}

pub async fn handle_tasks(
    store: web::Data<DataStore>,
    query: web::Query<TaskQuery>,
) -> impl Responder {
    let tasks = store.get_tasks(
        query.status.as_deref(),
        query.user_id,
    );
    let count = tasks.len();
    HttpResponse::Ok().json(TasksResponse { tasks, count })
}

// ── /api/stats ────────────────────────────────────────────────────────────────

pub async fn handle_stats(store: web::Data<DataStore>) -> impl Responder {
    HttpResponse::Ok().json(store.get_stats())
}
