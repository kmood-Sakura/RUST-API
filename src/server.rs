// src/server.rs
use crate::config::AppConfig;
use crate::database::Database;
use crate::request::extract_query_params;
use crate::response::{ErrorResponse, success_response, health_response};
use axum::{
    extract::{Query, State},
    response::Response,
    routing::get,
    Router,
};
use std::collections::HashMap;
use tower::ServiceBuilder;
use tower_http::cors::CorsLayer;
use tracing::{error, info, warn};

#[derive(Clone)]
pub struct AppState {
    pub database: Database,
}

pub struct Server {
    config: AppConfig,
    database: Database,
}

impl Server {
    pub fn new(config: AppConfig, database: Database) -> Self {
        Self { config, database }
    }

    pub async fn run(self) -> anyhow::Result<()> {
        let host = self.config.server.host.clone();
        let port = self.config.server.port;
        
        let app_state = AppState {
            database: self.database,
        };

        let app = Self::create_router(app_state);

        let listener = tokio::net::TcpListener::bind(format!("{}:{}", host, port)).await?;

        info!("Server running on {}:{}", host, port);

        axum::serve(listener, app).await?;

        Ok(())
    }

    fn create_router(state: AppState) -> Router {
        Router::new()
            .route("/", get(api_health))
            .route("/health", get(database_health))
            .route("/goods", get(get_goods))
            .layer(
                ServiceBuilder::new()
                    .layer(CorsLayer::permissive())
            )
            .with_state(state)
    }
}

// Route: GET / - API health check
async fn api_health() -> Response {
    info!("API health check requested");
    success_response(
        serde_json::json!({
            "status": "working",
            "service": "Rust API",
            "version": "1.0.0"
        }),
        "API is working correctly"
    )
}

// Route: GET /health - Database health check
async fn database_health(State(state): State<AppState>) -> Response {
    info!("Database health check requested");
    
    match state.database.health_check().await {
        Ok(_) => {
            info!("Database health check passed");
            health_response(true)
        }
        Err(e) => {
            error!("Database health check failed: {}", e);
            health_response(false)
        }
    }
}

// Route: GET /goods - Get goods with query parameters
async fn get_goods(
    State(state): State<AppState>,
    query: Query<HashMap<String, String>>,
) -> Response {
    info!("Goods search requested with params: {:?}", query.0);

    // Extract and validate query parameters
    let query_params = extract_query_params(query);
    
    // Check if no parameters provided
    if query_params.goods_id.is_none()
        && query_params.material_code.is_none()
        && query_params.goods_name.is_none()
        && query_params.price.is_none()
        && query_params.volumn_l.is_none()
        && query_params.mass_g.is_none()
        && query_params.min_volumn_l.is_none()
        && query_params.max_volumn_l.is_none()
        && query_params.min_mass_g.is_none()
        && query_params.max_mass_g.is_none()
        && query_params.min_price.is_none()
        && query_params.max_price.is_none()
    {
        warn!("No query parameters provided for goods search");
        return ErrorResponse::bad_request(
            "Query parameters required. Use goods_name=* or material_code=* to get all goods, or specify search criteria like goods_id, material_code, goods_name, price, volumn_l, mass_g, min_volumn_l, max_volumn_l, min_mass_g, max_mass_g, min_price, max_price"
        );
    }

    // Validate and parse query parameters (SQL injection protection)
    let search_params = match query_params.validate_and_parse() {
        Ok(params) => params,
        Err(error) => {
            warn!("Invalid query parameters: {}", error);
            return ErrorResponse::bad_request(&format!("Invalid query parameters: {}", error));
        }
    };

    // Perform database search
    match state.database.goods_table.search(search_params).await {
        Ok(goods) => {
            info!("Found {} goods matching search criteria", goods.len());
            success_response(goods, "Goods retrieved successfully")
        }
        Err(e) => {
            error!("Database error during goods search: {}", e);
            ErrorResponse::internal_server_error("Failed to search goods")
        }
    }
}