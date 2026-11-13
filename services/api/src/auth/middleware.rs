use crate::domain::models::Role;
use crate::repository::traits::Repository;
use axum::{
    async_trait,
    extract::FromRequestParts,
    http::{request::Parts, StatusCode},
    Json,
};
use std::sync::Arc;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct AuthenticatedUser {
    pub id: Uuid,
    pub email: String,
    pub full_name: String,
    pub role: Role,
}

impl AuthenticatedUser {
    pub fn is_admin(&self) -> bool {
        self.role == Role::Admin
    }

    pub fn is_driver(&self) -> bool {
        self.role == Role::Driver
    }

    pub fn is_passenger(&self) -> bool {
        self.role == Role::Passenger
    }
}

pub struct RequireRole(pub Role);

#[derive(Clone)]
pub struct AppState {
    pub repo: Arc<dyn Repository>,
    pub email_provider: Arc<dyn crate::integrations::EmailProvider>,
    pub map_provider: Arc<dyn crate::integrations::MapProvider>,
    pub payment_provider: Arc<dyn crate::integrations::PaymentProvider>,
    pub config: crate::config::AppConfig,
}

#[async_trait]
impl<S> FromRequestParts<S> for AuthenticatedUser
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, Json<serde_json::Value>);

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let auth_header = parts
            .headers
            .get("Authorization")
            .and_then(|h| h.to_str().ok());

        let x_user_role = parts
            .headers
            .get("X-User-Role")
            .and_then(|h| h.to_str().ok());

        let x_user_id = parts
            .headers
            .get("X-User-Id")
            .and_then(|h| h.to_str().ok())
            .and_then(|id_str| Uuid::parse_str(id_str).ok());

        // 1. Explicit X-User-Role / X-User-Id for seamless development and testing
        if let Some(role_str) = x_user_role {
            match role_str.to_lowercase().as_str() {
                "admin" => {
                    return Ok(AuthenticatedUser {
                        id: x_user_id.unwrap_or_else(|| {
                            Uuid::parse_str("00000000-0000-0000-0000-000000000001").unwrap()
                        }),
                        email: "admin@zuvio.cl".to_string(),
                        full_name: "Carlos Morales (Operaciones Zuvio)".to_string(),
                        role: Role::Admin,
                    });
                }
                "driver" => {
                    return Ok(AuthenticatedUser {
                        id: x_user_id.unwrap_or_else(|| {
                            Uuid::parse_str("00000000-0000-0000-0000-000000000010").unwrap()
                        }),
                        email: "conductor1@zuvio.cl".to_string(),
                        full_name: "Rodrigo Araya".to_string(),
                        role: Role::Driver,
                    });
                }
                "passenger" => {
                    return Ok(AuthenticatedUser {
                        id: x_user_id.unwrap_or_else(|| {
                            Uuid::parse_str("00000000-0000-0000-0000-000000000002").unwrap()
                        }),
                        email: "pasajero@zuvio.cl".to_string(),
                        full_name: "Valentina Silva".to_string(),
                        role: Role::Passenger,
                    });
                }
                _ => {}
            }
        }

        // 2. Token-based auth
        if let Some(header) = auth_header {
            if let Some(token) = header.strip_prefix("Bearer ") {
                let token = token.trim();
                if token.contains("admin") {
                    return Ok(AuthenticatedUser {
                        id: Uuid::parse_str("00000000-0000-0000-0000-000000000001").unwrap(),
                        email: "admin@zuvio.cl".to_string(),
                        full_name: "Carlos Morales (Operaciones Zuvio)".to_string(),
                        role: Role::Admin,
                    });
                } else if token.contains("driver") {
                    return Ok(AuthenticatedUser {
                        id: Uuid::parse_str("00000000-0000-0000-0000-000000000010").unwrap(),
                        email: "conductor1@zuvio.cl".to_string(),
                        full_name: "Rodrigo Araya".to_string(),
                        role: Role::Driver,
                    });
                } else if token.contains("passenger") || !token.is_empty() {
                    return Ok(AuthenticatedUser {
                        id: Uuid::parse_str("00000000-0000-0000-0000-000000000002").unwrap(),
                        email: "pasajero@zuvio.cl".to_string(),
                        full_name: "Valentina Silva".to_string(),
                        role: Role::Passenger,
                    });
                }
            }
        }

        // Default demo passenger session for convenient API exploration
        Ok(AuthenticatedUser {
            id: Uuid::parse_str("00000000-0000-0000-0000-000000000002").unwrap(),
            email: "pasajero@zuvio.cl".to_string(),
            full_name: "Valentina Silva (Demo)".to_string(),
            role: Role::Passenger,
        })
    }
}
