// STEP 7-5, 7-6: State와 에러 처리
// Cargo.toml:
// [dependencies]
// axum = "0.7"
// tokio = { version = "1", features = ["full"] }
// serde = { version = "1", features = ["derive"] }
// serde_json = "1"

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{get, post, put, delete},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::sync::Arc;
use tokio::sync::RwLock;

// ========================================
// 타입 정의
// ========================================

#[derive(Clone, Serialize)]
struct User {
    id: u32,
    name: String,
    email: String,
}

#[derive(Deserialize)]
struct CreateUser {
    name: String,
    email: String,
}

#[derive(Deserialize)]
struct UpdateUser {
    name: Option<String>,
    email: Option<String>,
}

// ========================================
// 7-5. 상태 공유
// ========================================

struct AppState {
    users: RwLock<Vec<User>>,
    next_id: RwLock<u32>,
}

impl AppState {
    fn new() -> Self {
        Self {
            users: RwLock::new(vec![
                User {
                    id: 1,
                    name: "Alice".to_string(),
                    email: "alice@example.com".to_string(),
                },
                User {
                    id: 2,
                    name: "Bob".to_string(),
                    email: "bob@example.com".to_string(),
                },
            ]),
            next_id: RwLock::new(3),
        }
    }
}

type SharedState = Arc<AppState>;

// ========================================
// 7-6. 에러 처리
// ========================================

enum AppError {
    NotFound(String),
    BadRequest(String),
    Internal(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AppError::NotFound(msg) => (StatusCode::NOT_FOUND, msg),
            AppError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg),
            AppError::Internal(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
        };

        let body = Json(json!({
            "success": false,
            "error": error_message
        }));

        (status, body).into_response()
    }
}

// ========================================
// 핸들러
// ========================================

// 모든 사용자 조회
async fn list_users(State(state): State<SharedState>) -> Json<serde_json::Value> {
    let users = state.users.read().await;
    Json(json!({
        "success": true,
        "data": *users,
        "count": users.len()
    }))
}

// 특정 사용자 조회
async fn get_user(
    Path(id): Path<u32>,
    State(state): State<SharedState>,
) -> Result<Json<serde_json::Value>, AppError> {
    let users = state.users.read().await;

    users
        .iter()
        .find(|u| u.id == id)
        .map(|user| {
            Json(json!({
                "success": true,
                "data": user
            }))
        })
        .ok_or_else(|| AppError::NotFound(format!("User {} not found", id)))
}

// 사용자 생성
async fn create_user(
    State(state): State<SharedState>,
    Json(payload): Json<CreateUser>,
) -> Result<(StatusCode, Json<serde_json::Value>), AppError> {
    // 유효성 검사
    if payload.name.is_empty() {
        return Err(AppError::BadRequest("Name cannot be empty".to_string()));
    }

    if !payload.email.contains('@') {
        return Err(AppError::BadRequest("Invalid email format".to_string()));
    }

    // ID 생성
    let mut next_id = state.next_id.write().await;
    let id = *next_id;
    *next_id += 1;

    // 사용자 생성
    let user = User {
        id,
        name: payload.name,
        email: payload.email,
    };

    let mut users = state.users.write().await;
    users.push(user.clone());

    Ok((
        StatusCode::CREATED,
        Json(json!({
            "success": true,
            "message": "User created",
            "data": user
        })),
    ))
}

// 사용자 수정
async fn update_user(
    Path(id): Path<u32>,
    State(state): State<SharedState>,
    Json(payload): Json<UpdateUser>,
) -> Result<Json<serde_json::Value>, AppError> {
    let mut users = state.users.write().await;

    let user = users
        .iter_mut()
        .find(|u| u.id == id)
        .ok_or_else(|| AppError::NotFound(format!("User {} not found", id)))?;

    if let Some(name) = payload.name {
        user.name = name;
    }

    if let Some(email) = payload.email {
        if !email.contains('@') {
            return Err(AppError::BadRequest("Invalid email format".to_string()));
        }
        user.email = email;
    }

    Ok(Json(json!({
        "success": true,
        "message": "User updated",
        "data": user.clone()
    })))
}

// 사용자 삭제
async fn delete_user(
    Path(id): Path<u32>,
    State(state): State<SharedState>,
) -> Result<Json<serde_json::Value>, AppError> {
    let mut users = state.users.write().await;

    let index = users
        .iter()
        .position(|u| u.id == id)
        .ok_or_else(|| AppError::NotFound(format!("User {} not found", id)))?;

    users.remove(index);

    Ok(Json(json!({
        "success": true,
        "message": format!("User {} deleted", id)
    })))
}

// ========================================
// 메인
// ========================================

#[tokio::main]
async fn main() {
    let state = Arc::new(AppState::new());

    let app = Router::new()
        .route("/users", get(list_users).post(create_user))
        .route(
            "/users/:id",
            get(get_user).put(update_user).delete(delete_user),
        )
        .with_state(state);

    println!("REST API running at http://localhost:3000");
    println!("Endpoints:");
    println!("  GET    /users      - List all users");
    println!("  POST   /users      - Create user");
    println!("  GET    /users/:id  - Get user");
    println!("  PUT    /users/:id  - Update user");
    println!("  DELETE /users/:id  - Delete user");

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
