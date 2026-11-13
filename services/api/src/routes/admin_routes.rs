use crate::auth::{AppState, AuthenticatedUser};
use crate::domain::models::*;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use chrono::Utc;
use serde::Deserialize;
use serde_json::{json, Value};
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct ReviewDriverRequest {
    pub status: String, // "approved" | "rejected"
    pub notes: Option<String>,
}

pub async fn list_pending_drivers(
    user: AuthenticatedUser,
    State(state): State<AppState>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    if !user.is_admin() {
        return Err((
            StatusCode::FORBIDDEN,
            Json(json!({ "error": "Acceso restringido al equipo de Administración y Operaciones" })),
        ));
    }

    let pending = state
        .repo
        .list_pending_drivers()
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "error": e.to_string() }))))?;

    let items: Vec<Value> = pending
        .into_iter()
        .map(|(profile, user, vehicle)| {
            json!({
                "profile": profile,
                "user": user,
                "vehicle": vehicle
            })
        })
        .collect();

    Ok(Json(json!(items)))
}

pub async fn review_driver(
    user: AuthenticatedUser,
    Path(driver_id): Path<Uuid>,
    State(state): State<AppState>,
    Json(payload): Json<ReviewDriverRequest>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    if !user.is_admin() {
        return Err((
            StatusCode::FORBIDDEN,
            Json(json!({ "error": "Solo administradores pueden aprobar o rechazar conductores" })),
        ));
    }

    let status = match payload.status.to_lowercase().as_str() {
        "approved" => DriverStatus::Approved,
        "rejected" => DriverStatus::Rejected,
        _ => {
            return Err((
                StatusCode::BAD_REQUEST,
                Json(json!({ "error": "Estado inválido. Opciones: 'approved', 'rejected'" })),
            ))
        }
    };

    let updated = state
        .repo
        .update_driver_status(&driver_id, status, payload.notes.clone(), &user.id)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "error": e.to_string() }))))?;

    // If driver is approved, also approve their electric vehicle if present
    if status == DriverStatus::Approved {
        if let Ok(Some(v)) = state.repo.find_vehicle_by_driver_id(&driver_id) {
            let _ = state
                .repo
                .update_vehicle_status(&v.id, VehicleStatus::Approved, &user.id);
        }
    }

    // Audit Log
    let _ = state.repo.create_audit_log(AuditLog {
        id: Uuid::new_v4(),
        actor_id: Some(user.id),
        action: format!("REVIEW_DRIVER_{}", payload.status.to_uppercase()),
        target_entity: "driver_profile".to_string(),
        target_id: Some(driver_id),
        details: json!({
            "status": payload.status,
            "notes": payload.notes
        }),
        ip_address: None,
        created_at: Utc::now(),
    });

    Ok(Json(json!({
        "message": format!("Conductor actualizado a {}", payload.status),
        "profile": updated
    })))
}

pub async fn list_audit_logs(
    user: AuthenticatedUser,
    State(state): State<AppState>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    if !user.is_admin() {
        return Err((
            StatusCode::FORBIDDEN,
            Json(json!({ "error": "Solo administradores pueden consultar el registro de auditoría" })),
        ));
    }

    let logs = state
        .repo
        .list_audit_logs()
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "error": e.to_string() }))))?;

    Ok(Json(json!(logs)))
}

