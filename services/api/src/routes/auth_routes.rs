use crate::auth::{AppState, AuthenticatedUser};
use crate::domain::models::Role;
use axum::{extract::State, http::StatusCode, Json};
use serde::Deserialize;
use serde_json::{json, Value};

#[derive(Debug, Deserialize)]
pub struct DemoSessionRequest {
    pub role: String, // "passenger" | "driver" | "admin"
}

pub async fn demo_session(
    State(state): State<AppState>,
    Json(payload): Json<DemoSessionRequest>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    let target_role = match payload.role.to_lowercase().as_str() {
        "admin" => Role::Admin,
        "driver" => Role::Driver,
        "passenger" => Role::Passenger,
        _ => {
            return Err((
                StatusCode::BAD_REQUEST,
                Json(json!({ "error": "Rol no válido. Opciones: 'passenger', 'driver', 'admin'" })),
            ))
        }
    };

    let users = state
        .repo
        .list_users()
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "error": e.to_string() }))))?;

    let user = users
        .into_iter()
        .find(|u| u.role == target_role)
        .ok_or_else(|| {
            (
                StatusCode::NOT_FOUND,
                Json(json!({ "error": "No se encontró usuario para este rol" })),
            )
        })?;

    let token = format!("demo-token-{}", payload.role.to_lowercase());

    Ok(Json(json!({
        "user": user,
        "token": token,
        "demo_mode": true
    })))
}

pub async fn get_current_profile(
    user: AuthenticatedUser,
    State(state): State<AppState>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    let profile = state
        .repo
        .find_user_by_id(&user.id)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "error": e.to_string() }))))?
        .ok_or_else(|| {
            (
                StatusCode::NOT_FOUND,
                Json(json!({ "error": "Perfil no encontrado" })),
            )
        })?;

    let driver_profile = if user.is_driver() {
        state.repo.find_driver_profile_by_user_id(&user.id).ok().flatten()
    } else {
        None
    };

    let vehicle = if let Some(ref dp) = driver_profile {
        state.repo.find_vehicle_by_driver_id(&dp.id).ok().flatten()
    } else {
        None
    };

    Ok(Json(json!({
        "user": profile,
        "driver_profile": driver_profile,
        "vehicle": vehicle
    })))
}
