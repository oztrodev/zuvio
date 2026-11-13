use crate::auth::{AppState, AuthenticatedUser};
use crate::domain::ev_rules::validate_electric_vehicle;
use crate::domain::models::*;
use axum::{
    extract::{Query, State},
    http::StatusCode,
    Json,
};
use chrono::Utc;
use serde::Deserialize;
use serde_json::{json, Value};
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct RegisterEVRequest {
    pub brand: String,
    pub model: String,
    pub year: i32,
    pub license_plate: String,
    pub propulsion: PropulsionType,
    pub battery_capacity_kwh: f64,
    pub passenger_capacity: i32,
}

#[derive(Debug, Deserialize)]
pub struct ToggleAvailabilityRequest {
    pub is_online: bool,
}

#[derive(Debug, Deserialize)]
pub struct UpdateCoverageRadiusRequest {
    pub radius_km: f64,
}

#[derive(Debug, Deserialize)]
pub struct NearbyDriversQuery {
    pub lat: f64,
    pub lng: f64,
    pub radius_km: Option<f64>,
}

pub async fn get_driver_profile(
    user: AuthenticatedUser,
    State(state): State<AppState>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    let dp = state
        .repo
        .find_driver_profile_by_user_id(&user.id)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "error": e.to_string() }))))?;

    if let Some(dp) = dp {
        let vehicle = state.repo.find_vehicle_by_driver_id(&dp.id).ok().flatten();
        Ok(Json(json!({
            "profile": dp,
            "vehicle": vehicle
        })))
    } else {
        Ok(Json(json!({ "profile": null, "vehicle": null })))
    }
}

pub async fn register_electric_vehicle(
    user: AuthenticatedUser,
    State(state): State<AppState>,
    Json(payload): Json<RegisterEVRequest>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    let dp = state
        .repo
        .find_driver_profile_by_user_id(&user.id)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "error": e.to_string() }))))?
        .ok_or_else(|| {
            (
                StatusCode::BAD_REQUEST,
                Json(json!({ "error": "Debe registrar un perfil de conductor antes de asociar su vehículo" })),
            )
        })?;

    if let Err(err) = validate_electric_vehicle(
        payload.propulsion,
        payload.year,
        payload.battery_capacity_kwh,
        payload.passenger_capacity,
        &payload.license_plate,
    ) {
        return Err((
            StatusCode::UNPROCESSABLE_ENTITY,
            Json(json!({
                "error": "Validación de auto eléctrico fallida",
                "detail": err.to_string()
            })),
        ));
    }

    let now = Utc::now();
    let vehicle = Vehicle {
        id: Uuid::new_v4(),
        driver_id: dp.id,
        brand: payload.brand,
        model: payload.model,
        year: payload.year,
        license_plate: payload.license_plate.to_uppercase(),
        propulsion: payload.propulsion,
        battery_capacity_kwh: payload.battery_capacity_kwh,
        passenger_capacity: payload.passenger_capacity,
        status: VehicleStatus::PendingReview,
        is_active: false,
        verified_at: None,
        verified_by: None,
        verification_notes: None,
        created_at: now,
        updated_at: now,
    };

    let created = state
        .repo
        .create_vehicle(vehicle)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "error": e.to_string() }))))?;

    let _ = state.repo.create_audit_log(AuditLog {
        id: Uuid::new_v4(),
        actor_id: Some(user.id),
        action: "SUBMIT_EV_REGISTRATION".to_string(),
        target_entity: "vehicle".to_string(),
        target_id: Some(created.id),
        details: json!({
            "brand": created.brand,
            "model": created.model,
            "plate": created.license_plate,
            "propulsion": "BEV"
        }),
        ip_address: None,
        created_at: now,
    });

    Ok(Json(json!({
        "message": "Vehículo 100% eléctrico registrado con éxito. Pendiente de revisión manual por el equipo de Operaciones.",
        "vehicle": created
    })))
}

pub async fn toggle_availability(
    user: AuthenticatedUser,
    State(state): State<AppState>,
    Json(payload): Json<ToggleAvailabilityRequest>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    let dp = state
        .repo
        .find_driver_profile_by_user_id(&user.id)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "error": e.to_string() }))))?
        .ok_or_else(|| {
            (
                StatusCode::BAD_REQUEST,
                Json(json!({ "error": "No existe perfil de conductor" })),
            )
        })?;

    let updated = state
        .repo
        .set_driver_online(&dp.id, payload.is_online)
        .map_err(|e| match e {
            crate::repository::RepoError::DriverNotEligible => (
                StatusCode::FORBIDDEN,
                Json(json!({
                    "error": "No puedes conectarte hasta que tu documentación y vehículo eléctrico sean aprobados por Operaciones"
                })),
            ),
            _ => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": e.to_string() })),
            ),
        })?;

    Ok(Json(json!({
        "message": if payload.is_online { "Conectado. Disponible para recibir viajes" } else { "Desconectado" },
        "is_online": updated.is_online
    })))
}

pub async fn update_coverage_radius(
    user: AuthenticatedUser,
    State(state): State<AppState>,
    Json(payload): Json<UpdateCoverageRadiusRequest>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    if !user.is_driver() && !user.is_admin() {
        return Err((
            StatusCode::FORBIDDEN,
            Json(json!({ "error": "Solo conductores pueden modificar su radio de cobertura" })),
        ));
    }

    let dp = state
        .repo
        .find_driver_profile_by_user_id(&user.id)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "error": e.to_string() }))))?
        .ok_or_else(|| {
            (
                StatusCode::BAD_REQUEST,
                Json(json!({ "error": "No existe perfil de conductor" })),
            )
        })?;

    let updated = state
        .repo
        .set_driver_coverage_radius(&dp.id, payload.radius_km)
        .map_err(|e| (StatusCode::BAD_REQUEST, Json(json!({ "error": e.to_string() }))))?;

    Ok(Json(json!({
        "message": format!("Radio de cobertura actualizado a {:.1} km", updated.coverage_radius_km),
        "coverage_radius_km": updated.coverage_radius_km
    })))
}

pub async fn get_nearby_drivers(
    State(state): State<AppState>,
    Query(params): Query<NearbyDriversQuery>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    let max_radius = params.radius_km.unwrap_or(15.0);
    let nearby = state
        .repo
        .get_nearby_drivers(params.lat, params.lng, max_radius)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "error": e.to_string() }))))?;

    Ok(Json(json!(nearby)))
}

pub async fn get_available_rides(
    user: AuthenticatedUser,
    State(state): State<AppState>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    if !user.is_driver() && !user.is_admin() {
        return Err((
            StatusCode::FORBIDDEN,
            Json(json!({ "error": "Solo conductores pueden consultar viajes disponibles" })),
        ));
    }

    let all = state
        .repo
        .list_all_rides()
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "error": e.to_string() }))))?;

    let available: Vec<Ride> = all
        .into_iter()
        .filter(|r| r.status == RideStatus::Matching && r.driver_id.is_none())
        .collect();

    Ok(Json(json!(available)))
}
