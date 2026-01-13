// STEP 7-7 ~ 7-9: 미들웨어와 인증
// Cargo.toml:
// [dependencies]
// axum = "0.7"
// tokio = { version = "1", features = ["full"] }
// serde = { version = "1", features = ["derive"] }
// serde_json = "1"
// tower-http = { version = "0.5", features = ["cors", "trace"] }
// jsonwebtoken = "9"
// chrono = "0.4"

use axum::{
    extract::Request,
    http::{header, StatusCode},
    middleware::{self, Next},
    response::{IntoResponse, Response},
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::time::Instant;

// ========================================
// 7-7. 로깅 미들웨어
// ========================================

async fn logging_middleware(req: Request, next: Next) -> Response {
    let method = req.method().clone();
    let uri = req.uri().clone();
    let start = Instant::now();

    println!("--> {} {}", method, uri);

    let response = next.run(req).await;

    let duration = start.elapsed();
    println!("<-- {} {} {:?} [{:?}]", method, uri, response.status(), duration);

    response
}

// ========================================
// 7-9. JWT 인증
// ========================================

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,        // subject (user id)
    exp: usize,         // expiration time
    iat: usize,         // issued at
    role: String,       // user role
}

const JWT_SECRET: &[u8] = b"your-secret-key-here";

fn create_token(user_id: &str, role: &str) -> Result<String, jsonwebtoken::errors::Error> {
    use jsonwebtoken::{encode, EncodingKey, Header};

    let now = chrono::Utc::now().timestamp() as usize;
    let exp = now + 3600; // 1시간 후 만료

    let claims = Claims {
        sub: user_id.to_string(),
        exp,
        iat: now,
        role: role.to_string(),
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(JWT_SECRET),
    )
}

fn verify_token(token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    use jsonwebtoken::{decode, DecodingKey, Validation};

    decode::<Claims>(
        token,
        &DecodingKey::from_secret(JWT_SECRET),
        &Validation::default(),
    )
    .map(|data| data.claims)
}

// 인증 미들웨어
async fn auth_middleware(req: Request, next: Next) -> Result<Response, AuthError> {
    // Authorization 헤더 확인
    let auth_header = req
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|h| h.to_str().ok());

    let token = match auth_header {
        Some(h) if h.starts_with("Bearer ") => &h[7..],
        _ => return Err(AuthError::MissingToken),
    };

    // 토큰 검증
    match verify_token(token) {
        Ok(_claims) => Ok(next.run(req).await),
        Err(_) => Err(AuthError::InvalidToken),
    }
}

// 인증 에러
enum AuthError {
    MissingToken,
    InvalidToken,
}

impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            AuthError::MissingToken => (StatusCode::UNAUTHORIZED, "Missing authorization token"),
            AuthError::InvalidToken => (StatusCode::UNAUTHORIZED, "Invalid token"),
        };

        (status, Json(json!({ "error": message }))).into_response()
    }
}

// ========================================
// 핸들러
// ========================================

// 로그인 (토큰 발급)
#[derive(Deserialize)]
struct LoginRequest {
    username: String,
    password: String,
}

#[derive(Serialize)]
struct LoginResponse {
    token: String,
    token_type: String,
    expires_in: u32,
}

async fn login(Json(payload): Json<LoginRequest>) -> Result<Json<LoginResponse>, AuthError> {
    // 실제로는 DB에서 사용자 확인
    if payload.username == "admin" && payload.password == "password" {
        let token = create_token(&payload.username, "admin")
            .map_err(|_| AuthError::InvalidToken)?;

        Ok(Json(LoginResponse {
            token,
            token_type: "Bearer".to_string(),
            expires_in: 3600,
        }))
    } else {
        Err(AuthError::InvalidToken)
    }
}

// 공개 엔드포인트
async fn public_route() -> Json<serde_json::Value> {
    Json(json!({
        "message": "This is a public route",
        "auth_required": false
    }))
}

// 보호된 엔드포인트
async fn protected_route() -> Json<serde_json::Value> {
    Json(json!({
        "message": "This is a protected route",
        "auth_required": true,
        "data": {
            "secret": "Only authenticated users can see this"
        }
    }))
}

async fn user_profile() -> Json<serde_json::Value> {
    Json(json!({
        "id": 1,
        "name": "Admin User",
        "email": "admin@example.com",
        "role": "admin"
    }))
}

// ========================================
// 메인
// ========================================

#[tokio::main]
async fn main() {
    // 공개 라우트
    let public_routes = Router::new()
        .route("/", get(public_route))
        .route("/login", post(login));

    // 보호된 라우트 (인증 필요)
    let protected_routes = Router::new()
        .route("/protected", get(protected_route))
        .route("/profile", get(user_profile))
        .layer(middleware::from_fn(auth_middleware));

    // 전체 앱
    let app = Router::new()
        .merge(public_routes)
        .nest("/api", protected_routes)
        .layer(middleware::from_fn(logging_middleware));

    println!("Server running at http://localhost:3000");
    println!("\nEndpoints:");
    println!("  GET  /          - Public route");
    println!("  POST /login     - Get JWT token");
    println!("  GET  /api/protected - Protected route (requires token)");
    println!("  GET  /api/profile   - User profile (requires token)");
    println!("\nTest login:");
    println!("  curl -X POST http://localhost:3000/login \\");
    println!("    -H 'Content-Type: application/json' \\");
    println!("    -d '{{\"username\":\"admin\",\"password\":\"password\"}}'");

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
