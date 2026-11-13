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
pub struct CreateTicketRequest {
    pub ride_id: Option<Uuid>,
    pub subject: String,
    pub message: String,
}

#[derive(Debug, Deserialize)]
pub struct ResolveTicketRequest {
    pub resolution_notes: String,
}

pub async fn create_ticket(
    user: AuthenticatedUser,
    State(state): State<AppState>,
    Json(payload): Json<CreateTicketRequest>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    if payload.subject.trim().is_empty() || payload.message.trim().is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(json!({ "error": "El asunto y mensaje son obligatorios" })),
        ));
    }

    let now = Utc::now();
    let ticket = SupportTicket {
        id: Uuid::new_v4(),
        user_id: user.id,
        ride_id: payload.ride_id,
        subject: payload.subject,
        message: payload.message,
        status: TicketStatus::Open,
        resolution_notes: None,
        resolved_by: None,
        resolved_at: None,
        created_at: now,
        updated_at: now,
    };

    let created = state
        .repo
        .create_support_ticket(ticket)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "error": e.to_string() }))))?;

    Ok(Json(json!({
        "message": "Ticket de soporte recibido. Nuestro equipo te responderá a la brevedad.",
        "ticket": created
    })))
}

pub async fn list_tickets(
    user: AuthenticatedUser,
    State(state): State<AppState>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    let tickets = if user.is_admin() {
        state.repo.list_all_support_tickets()
    } else {
        state.repo.list_support_tickets_for_user(&user.id)
    }
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "error": e.to_string() }))))?;

    Ok(Json(json!(tickets)))
}

pub async fn resolve_ticket(
    user: AuthenticatedUser,
    Path(ticket_id): Path<Uuid>,
    State(state): State<AppState>,
    Json(payload): Json<ResolveTicketRequest>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    if !user.is_admin() {
        return Err((
            StatusCode::FORBIDDEN,
            Json(json!({ "error": "Solo administradores pueden resolver tickets de soporte" })),
        ));
    }

    let resolved = state
        .repo
        .resolve_support_ticket(&ticket_id, &payload.resolution_notes, &user.id)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "error": e.to_string() }))))?;

    Ok(Json(json!({
        "message": "Ticket resuelto con éxito",
        "ticket": resolved
    })))
}

