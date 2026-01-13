// STEP 7-1 ~ 7-4: Axum 기초
// Cargo.toml:
// [dependencies]
// axum = "0.7"
// tokio = { version = "1", features = ["full"] }
// serde = { version = "1", features = ["derive"] }
// serde_json = "1"

use axum::{
    extract::{Path, Query},
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};

#[tokio::main]
async fn main() {
    // 라우터 설정
    let app = Router::new()
        // 기본 라우트
        .route("/", get(root))
        .route("/hello/:name", get(hello))
        // Users API
        .route("/users", get(list_users).post(create_user))
        .route("/users/:id", get(get_user))
        // Nested routes
        .nest("/api", api_routes());

    println!("Server running at http://localhost:3000");

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

// API 서브 라우터
fn api_routes() -> Router {
    Router::new()
        .route("/health", get(health_check))
        .route("/version", get(version))
}

// ========================================
// 7-1. 기본 핸들러
// ========================================

async fn root() -> &'static str {
    "Hello, Axum!"
}

async fn health_check() -> &'static str {
    "OK"
}

async fn version() -> Json<Version> {
    Json(Version {
        version: "1.0.0".to_string(),
        name: "Rust API".to_string(),
    })
}

#[derive(Serialize)]
struct Version {
    version: String,
    name: String,
}

// ========================================
// 7-3. Path 파라미터
// ========================================

async fn hello(Path(name): Path<String>) -> String {
    format!("Hello, {}!", name)
}

async fn get_user(Path(id): Path<u32>) -> Json<User> {
    // 실제로는 DB에서 조회
    Json(User {
        id,
        name: format!("User {}", id),
        email: format!("user{}@example.com", id),
    })
}

// ========================================
// 7-3. Query 파라미터
// ========================================

#[derive(Deserialize)]
struct Pagination {
    page: Option<u32>,
    limit: Option<u32>,
    sort: Option<String>,
}

async fn list_users(Query(params): Query<Pagination>) -> Json<UserList> {
    let page = params.page.unwrap_or(1);
    let limit = params.limit.unwrap_or(10);
    let sort = params.sort.unwrap_or_else(|| "id".to_string());

    // 실제로는 DB에서 조회
    let users = vec![
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
    ];

    Json(UserList {
        users,
        page,
        limit,
        sort,
        total: 2,
    })
}

// ========================================
// 7-4. JSON 요청/응답
// ========================================

#[derive(Serialize, Clone)]
struct User {
    id: u32,
    name: String,
    email: String,
}

#[derive(Serialize)]
struct UserList {
    users: Vec<User>,
    page: u32,
    limit: u32,
    sort: String,
    total: u32,
}

#[derive(Deserialize)]
struct CreateUser {
    name: String,
    email: String,
}

#[derive(Serialize)]
struct CreateUserResponse {
    id: u32,
    name: String,
    email: String,
    message: String,
}

async fn create_user(Json(payload): Json<CreateUser>) -> Json<CreateUserResponse> {
    // 실제로는 DB에 저장
    Json(CreateUserResponse {
        id: 1,
        name: payload.name,
        email: payload.email,
        message: "User created successfully".to_string(),
    })
}
