use crate::auth::{AppState, AuthenticatedUser};
use crate::domain::models::*;
use crate::domain::pricing::calculate_fare;
use crate::integrations::EmailPayload;
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
pub struct EstimateRideRequest {
    pub origin_address: String,
    pub origin_commune: String,
    pub origin_lat: f64,
    pub origin_lng: f64,
    pub destination_address: String,
    pub destination_commune: String,
    pub destination_lat: f64,
    pub destination_lng: f64,
}

#[derive(Debug, Deserialize)]
pub struct CreateRideRequest {
    pub origin_address: String,
    pub origin_commune: String,
    pub origin_lat: f64,
    pub origin_lng: f64,
    pub destination_address: String,
    pub destination_commune: String,
    pub destination_lat: f64,
    pub destination_lng: f64,
}

#[derive(Debug, Deserialize)]
pub struct UpdateRideStatusRequest {
    pub status: String,
}

#[derive(Debug, Deserialize)]
pub struct CancelRideRequest {
    pub reason: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct RateRideRequest {
    pub score: i32,
    pub comment: Option<String>,
}

pub async fn estimate_ride(
    State(state): State<AppState>,
    Json(payload): Json<EstimateRideRequest>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    let route = state
        .map_provider
        .calculate_route(
            payload.origin_lat,
            payload.origin_lng,
            payload.destination_lat,
            payload.destination_lng,
        )
        .await;

    let fare = calculate_fare(route.distance_km, route.duration_minutes, &state.config);

    Ok(Json(json!({
        "distance_km": fare.distance_km,
        "duration_minutes": fare.duration_minutes,
        "base_fare_clp": fare.base_fare_clp,
        "distance_fare_clp": fare.distance_fare_clp,
        "time_fare_clp": fare.time_fare_clp,
        "total_fare_clp": fare.total_fare_clp,
        "currency": fare.currency,
        "co2_saved_kg": fare.co2_saved_kg,
        "is_minimum_applied": fare.is_minimum_applied,
        "vehicle_type": "100% Eléctrico (BEV)",
        "is_estimate": true,
        "provider_note": route.provider_note
    })))
}

pub async fn create_ride(
    user: AuthenticatedUser,
    State(state): State<AppState>,
    Json(payload): Json<CreateRideRequest>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    if !user.is_passenger() && !user.is_admin() {
        return Err((
            StatusCode::FORBIDDEN,
            Json(json!({ "error": "Solo pasajeros registrados pueden solicitar viajes" })),
        ));
    }

    if let Ok(Some(active)) = state.repo.find_active_ride_for_user(&user.id, user.role) {
        return Err((
            StatusCode::CONFLICT,
            Json(json!({
                "error": "Ya tienes un viaje activo en curso",
                "active_ride_id": active.id,
                "status": active.status
            })),
        ));
    }

    let route = state
        .map_provider
        .calculate_route(
            payload.origin_lat,
            payload.origin_lng,
            payload.destination_lat,
            payload.destination_lng,
        )
        .await;

    let fare = calculate_fare(route.distance_km, route.duration_minutes, &state.config);
    let now = Utc::now();

    let ride = Ride {
        id: Uuid::new_v4(),
        passenger_id: user.id,
        driver_id: None,
        vehicle_id: None,
        status: RideStatus::Matching,
        origin_address: payload.origin_address,
        origin_commune: payload.origin_commune,
        origin_lat: payload.origin_lat,
        origin_lng: payload.origin_lng,
        destination_address: payload.destination_address,
        destination_commune: payload.destination_commune,
        destination_lat: payload.destination_lat,
        destination_lng: payload.destination_lng,
        distance_km: fare.distance_km,
        duration_minutes: fare.duration_minutes,
        estimated_fare_clp: fare.total_fare_clp,
        final_fare_clp: None,
        co2_saved_kg: fare.co2_saved_kg,
        cancellation_reason: None,
        cancelled_by: None,
        created_at: now,
        updated_at: now,
        accepted_at: None,
        driver_arrived_at: None,
        started_at: None,
        completed_at: None,
        cancelled_at: None,
    };

    let created = state
        .repo
        .create_ride(ride)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "error": e.to_string() }))))?;

    let email_body = format!(
        "Hola {},\n\nTu viaje Zuvio 100% eléctrico ha sido solicitado con éxito.\nOrigen: {}\nDestino: {}\nTarifa estimada: ${} CLP.\n\nEstamos conectándote con el conductor eléctrico más cercano en Santiago.",
        user.full_name, created.origin_address, created.destination_address, created.estimated_fare_clp
    );

    let _ = state
        .email_provider
        .send_email(EmailPayload {
            to_email: user.email,
            to_name: user.full_name,
            subject: "Zuvio — Tu viaje 100% eléctrico está siendo asignado".to_string(),
            html_content: format!("<p>{}</p>", email_body.replace('\n', "<br>")),
            text_content: Some(email_body),
        })
        .await;

    Ok(Json(json!(created)))
}

pub async fn list_rides(
    user: AuthenticatedUser,
    State(state): State<AppState>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    let rides = state
        .repo
        .list_rides_for_user(&user.id, user.role)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "error": e.to_string() }))))?;

    Ok(Json(json!(rides)))
}

pub async fn get_active_ride(
    user: AuthenticatedUser,
    State(state): State<AppState>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    let ride = state
        .repo
        .find_active_ride_for_user(&user.id, user.role)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "error": e.to_string() }))))?;

    if let Some(r) = ride {
        let driver_user = if let Some(dp_id) = r.driver_id {
            if let Ok(Some(dp)) = state.repo.find_driver_profile_by_id(&dp_id) {
                state.repo.find_user_by_id(&dp.user_id).ok().flatten()
            } else {
                None
            }
        } else {
            None
        };

        let vehicle = if let Some(v_id) = r.vehicle_id {
            state.repo.find_vehicle_by_id(&v_id).ok().flatten()
        } else {
            None
        };

        Ok(Json(json!({
            "ride": r,
            "driver": driver_user,
            "vehicle": vehicle
        })))
    } else {
        Ok(Json(json!({ "ride": null })))
    }
}

pub async fn get_ride_by_id(
    Path(ride_id): Path<Uuid>,
    State(state): State<AppState>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    let ride = state
        .repo
        .find_ride_by_id(&ride_id)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "error": e.to_string() }))))?
        .ok_or_else(|| {
            (
                StatusCode::NOT_FOUND,
                Json(json!({ "error": "Viaje no encontrado" })),
            )
        })?;

    Ok(Json(json!(ride)))
}

pub async fn accept_ride(
    user: AuthenticatedUser,
    Path(ride_id): Path<Uuid>,
    State(state): State<AppState>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    if !user.is_driver() && !user.is_admin() {
        return Err((
            StatusCode::FORBIDDEN,
            Json(json!({ "error": "Solo conductores habilitados pueden aceptar solicitudes" })),
        ));
    }

    let driver_profile = state
        .repo
        .find_driver_profile_by_user_id(&user.id)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "error": e.to_string() }))))?
        .ok_or_else(|| {
            (
                StatusCode::BAD_REQUEST,
                Json(json!({ "error": "No existe perfil de conductor para este usuario" })),
            )
        })?;

    let vehicle = state
        .repo
        .find_vehicle_by_driver_id(&driver_profile.id)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "error": e.to_string() }))))?
        .ok_or_else(|| {
            (
                StatusCode::BAD_REQUEST,
                Json(json!({ "error": "El conductor no tiene un vehículo eléctrico registrado" })),
            )
        })?;

    let accepted_ride = state
        .repo
        .accept_ride_atomic(&ride_id, &driver_profile.id, &vehicle.id)
        .map_err(|e| match e {
            crate::repository::RepoError::DoubleAssignmentConflict => (
                StatusCode::CONFLICT,
                Json(json!({ "error": "La solicitud ya fue tomada por otro conductor o ya no está disponible" })),
            ),
            crate::repository::RepoError::DriverAlreadyCommitted => (
                StatusCode::CONFLICT,
                Json(json!({ "error": "Ya tienes un viaje en curso activo" })),
            ),
            crate::repository::RepoError::DriverNotEligible => (
                StatusCode::FORBIDDEN,
                Json(json!({ "error": "Tu perfil o vehículo eléctrico no se encuentra aprobado" })),
            ),
            _ => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": e.to_string() })),
            ),
        })?;

    Ok(Json(json!({
        "message": "Viaje aceptado exitosamente",
        "ride": accepted_ride
    })))
}

pub async fn update_ride_status(
    user: AuthenticatedUser,
    Path(ride_id): Path<Uuid>,
    State(state): State<AppState>,
    Json(payload): Json<UpdateRideStatusRequest>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    let next_status = match payload.status.to_lowercase().as_str() {
        "driver_en_route" => RideStatus::DriverEnRoute,
        "arrived" => RideStatus::Arrived,
        "in_progress" => RideStatus::InProgress,
        "completed" => RideStatus::Completed,
        "cancelled" => RideStatus::Cancelled,
        _ => {
            return Err((
                StatusCode::BAD_REQUEST,
                Json(json!({ "error": "Estado de destino no válido" })),
            ))
        }
    };

    let dp_id = if user.is_driver() {
        state
            .repo
            .find_driver_profile_by_user_id(&user.id)
            .ok()
            .flatten()
            .map(|dp| dp.id)
            .unwrap_or(user.id)
    } else {
        user.id
    };

    let updated_ride = state
        .repo
        .update_ride_status(&ride_id, next_status, &dp_id, user.role, None)
        .map_err(|e| (StatusCode::BAD_REQUEST, Json(json!({ "error": e.to_string() }))))?;

    if next_status == RideStatus::Completed {
        let _ = state
            .payment_provider
            .capture_ride(&ride_id, updated_ride.estimated_fare_clp)
            .await;
    }

    Ok(Json(json!({
        "message": "Estado de viaje actualizado",
        "ride": updated_ride
    })))
}

pub async fn cancel_ride(
    user: AuthenticatedUser,
    Path(ride_id): Path<Uuid>,
    State(state): State<AppState>,
    Json(payload): Json<CancelRideRequest>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    let reason = payload.reason.unwrap_or_else(|| "Cancelado por el usuario".to_string());

    let updated_ride = state
        .repo
        .update_ride_status(
            &ride_id,
            RideStatus::Cancelled,
            &user.id,
            user.role,
            Some(reason),
        )
        .map_err(|e| (StatusCode::BAD_REQUEST, Json(json!({ "error": e.to_string() }))))?;

    Ok(Json(json!({
        "message": "Viaje cancelado",
        "ride": updated_ride
    })))
}

pub async fn rate_ride(
    user: AuthenticatedUser,
    Path(ride_id): Path<Uuid>,
    State(state): State<AppState>,
    Json(payload): Json<RateRideRequest>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    if payload.score < 1 || payload.score > 5 {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(json!({ "error": "La calificación debe ser entre 1 y 5 estrellas" })),
        ));
    }

    let ride = state
        .repo
        .find_ride_by_id(&ride_id)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "error": e.to_string() }))))?
        .ok_or_else(|| {
            (
                StatusCode::NOT_FOUND,
                Json(json!({ "error": "Viaje no encontrado" })),
            )
        })?;

    if ride.status != RideStatus::Completed {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(json!({ "error": "Solo se pueden calificar viajes completados" })),
        ));
    }

    let rated_user_id = if user.id == ride.passenger_id {
        if let Some(dp_id) = ride.driver_id {
            state
                .repo
                .find_driver_profile_by_id(&dp_id)
                .ok()
                .flatten()
                .map(|dp| dp.user_id)
                .unwrap_or(user.id)
        } else {
            user.id
        }
    } else {
        ride.passenger_id
    };

    let rating = RideRating {
        id: Uuid::new_v4(),
        ride_id,
        rater_id: user.id,
        rated_user_id,
        score: payload.score,
        comment: payload.comment,
        created_at: Utc::now(),
    };

    let saved = state
        .repo
        .create_ride_rating(rating)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "error": e.to_string() }))))?;

    Ok(Json(json!({
        "message": "Calificación registrada con éxito",
        "rating": saved
    })))
}
