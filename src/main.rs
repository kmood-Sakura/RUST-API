use axum::{
    extract::Query,
    response::Json,
    routing::get,
    Router,
};
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Serialize)]
struct ApiResponse {
    message: String,
    status: String,
}

#[derive(Deserialize)]
struct QueryParams {
    name: Option<String>,
}

async fn health_check() -> Json<ApiResponse> {
    Json(ApiResponse {
        message: "API is running!".to_string(),
        status: "ok".to_string(),
    })
}

async fn hello(Query(params): Query<QueryParams>) -> Json<ApiResponse> {
    let name = params.name.unwrap_or_else(|| "World".to_string());
    Json(ApiResponse {
        message: format!("Hello, {}!", name),
        status: "ok".to_string(),
    })
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(health_check))
        .route("/hello", get(hello));

    // Render.com provides PORT environment variable
    let port = env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    let addr = format!("0.0.0.0:{}", port);

    println!("Server running on {}", addr);

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}