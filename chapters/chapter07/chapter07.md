# STEP 7. Rust 웹 개발

> Actix-web/Axum을 활용한 웹 API 개발

---

## 학습 목표
- Actix-web/Axum 프레임워크 이해
- REST API 개발
- 데이터베이스 연동
- JWT 인증 구현

---

## 7-1. Axum 기초

### 설정

```toml
# Cargo.toml
[dependencies]
axum = "0.7"
tokio = { version = "1", features = ["full"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
```

### Hello World

```rust
use axum::{routing::get, Router};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
```

### Express.js 비교
```javascript
const express = require('express');
const app = express();

app.get('/', (req, res) => {
    res.send('Hello, World!');
});

app.listen(3000);
```

---

## 7-2. 라우팅

### 핵심 개념

```rust
use axum::{
    routing::{get, post, put, delete},
    Router,
};

fn create_router() -> Router {
    Router::new()
        .route("/", get(root))
        .route("/users", get(list_users).post(create_user))
        .route("/users/:id", get(get_user).put(update_user).delete(delete_user))
        .nest("/api", api_routes())
}

fn api_routes() -> Router {
    Router::new()
        .route("/health", get(health_check))
}

async fn root() -> &'static str {
    "Root"
}

async fn health_check() -> &'static str {
    "OK"
}
```

---

## 7-3. Path/Query 파라미터

### 핵심 개념

```rust
use axum::extract::{Path, Query};
use serde::Deserialize;

// Path 파라미터
async fn get_user(Path(id): Path<u32>) -> String {
    format!("User ID: {}", id)
}

// 여러 Path 파라미터
async fn get_post(Path((user_id, post_id)): Path<(u32, u32)>) -> String {
    format!("User {} Post {}", user_id, post_id)
}

// Query 파라미터
#[derive(Deserialize)]
struct Pagination {
    page: Option<u32>,
    limit: Option<u32>,
}

async fn list_users(Query(params): Query<Pagination>) -> String {
    let page = params.page.unwrap_or(1);
    let limit = params.limit.unwrap_or(10);
    format!("Page: {}, Limit: {}", page, limit)
}
```

---

## 7-4. JSON 요청/응답

### 핵심 개념

```rust
use axum::Json;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct CreateUser {
    name: String,
    email: String,
}

#[derive(Serialize)]
struct User {
    id: u32,
    name: String,
    email: String,
}

// JSON 요청 받기
async fn create_user(Json(payload): Json<CreateUser>) -> Json<User> {
    let user = User {
        id: 1,
        name: payload.name,
        email: payload.email,
    };
    Json(user)
}

// JSON 응답
async fn get_user(Path(id): Path<u32>) -> Json<User> {
    Json(User {
        id,
        name: "John".to_string(),
        email: "john@example.com".to_string(),
    })
}
```

---

## 7-5. 상태 공유 (State)

### 핵심 개념

```rust
use axum::extract::State;
use std::sync::Arc;
use tokio::sync::RwLock;

struct AppState {
    db: RwLock<Vec<User>>,
}

#[tokio::main]
async fn main() {
    let state = Arc::new(AppState {
        db: RwLock::new(vec![]),
    });

    let app = Router::new()
        .route("/users", get(list_users).post(create_user))
        .with_state(state);

    // ...
}

async fn list_users(State(state): State<Arc<AppState>>) -> Json<Vec<User>> {
    let users = state.db.read().await;
    Json(users.clone())
}

async fn create_user(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CreateUser>,
) -> Json<User> {
    let mut users = state.db.write().await;
    let user = User {
        id: users.len() as u32 + 1,
        name: payload.name,
        email: payload.email,
    };
    users.push(user.clone());
    Json(user)
}
```

---

## 7-6. 에러 처리

### 핵심 개념

```rust
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;

// 커스텀 에러 타입
enum AppError {
    NotFound,
    BadRequest(String),
    Internal(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            AppError::NotFound => (StatusCode::NOT_FOUND, "Not found"),
            AppError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg.as_str()),
            AppError::Internal(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg.as_str()),
        };

        let body = Json(json!({
            "error": message
        }));

        (status, body).into_response()
    }
}

async fn get_user(Path(id): Path<u32>) -> Result<Json<User>, AppError> {
    if id == 0 {
        return Err(AppError::BadRequest("Invalid ID".to_string()));
    }

    // DB에서 찾기...
    Err(AppError::NotFound)
}
```

---

## 7-7. 미들웨어

### 핵심 개념

```rust
use axum::{
    middleware::{self, Next},
    extract::Request,
    response::Response,
};

async fn logging_middleware(req: Request, next: Next) -> Response {
    println!("Request: {} {}", req.method(), req.uri());
    let start = std::time::Instant::now();

    let response = next.run(req).await;

    println!("Response time: {:?}", start.elapsed());
    response
}

fn create_router() -> Router {
    Router::new()
        .route("/", get(root))
        .layer(middleware::from_fn(logging_middleware))
}
```

### Tower 미들웨어
```rust
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;

let app = Router::new()
    .route("/", get(root))
    .layer(CorsLayer::permissive())
    .layer(TraceLayer::new_for_http());
```

---

## 7-8. 데이터베이스 (SQLx)

### 설정

```toml
# Cargo.toml
[dependencies]
sqlx = { version = "0.7", features = ["runtime-tokio", "postgres"] }
```

### 사용

```rust
use sqlx::postgres::PgPoolOptions;

#[tokio::main]
async fn main() {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://user:pass@localhost/db")
        .await
        .unwrap();

    let users = sqlx::query_as!(
        User,
        "SELECT id, name, email FROM users"
    )
    .fetch_all(&pool)
    .await
    .unwrap();
}
```

---

## 7-9. JWT 인증

### 핵심 개념

```rust
use jsonwebtoken::{encode, decode, Header, Validation, EncodingKey, DecodingKey};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: usize,
}

fn create_token(user_id: &str) -> String {
    let claims = Claims {
        sub: user_id.to_string(),
        exp: 10000000000,  // 만료 시간
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret("secret".as_ref())
    ).unwrap()
}

fn verify_token(token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    decode::<Claims>(
        token,
        &DecodingKey::from_secret("secret".as_ref()),
        &Validation::default()
    ).map(|data| data.claims)
}
```

---

## 7-10. Actix-web 대안

### 기본 구조

```rust
use actix_web::{web, App, HttpServer, HttpResponse};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
            .route("/users", web::get().to(list_users))
            .route("/users", web::post().to(create_user))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

async fn index() -> HttpResponse {
    HttpResponse::Ok().body("Hello!")
}
```

---

## 예제 파일
- `examples/axum_basic.rs` - Axum 기초
- `examples/rest_api.rs` - REST API 구현
- `examples/middleware.rs` - 미들웨어와 에러 처리

---

## 다음 단계
→ STEP 8: Rust WebAssembly
