use axum::{extract::State, Json};
use chrono::Utc;
use serde_json::{json, Value};
use crate::auth::AppState;

pub async fn health_check(State(state): State<AppState>) -> Json<Value> {
    Json(json!({
        "status": "healthy",
        "service": "zuvio-api",
        "version": "0.1.0",
        "environment": state.config.environment,
        "demo_mode": state.config.demo_mode,
        "market": "Santiago de Chile",
        "fleet_propulsion": "100% BEV (Battery Electric Vehicles)",
        "timestamp": Utc::now()
    }))
}

