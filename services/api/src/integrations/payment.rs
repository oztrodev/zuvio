use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use tracing::info;
use uuid::Uuid;

#[derive(Debug, Error)]
pub enum PaymentError {
    #[error("Error de proveedor de pago: {0}")]
    ProviderError(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentRequest {
    pub ride_id: Uuid,
    pub amount_clp: i32,
    pub currency: String,
    pub passenger_id: Uuid,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentResult {
    pub transaction_id: String,
    pub is_success: bool,
    pub status: String,
    pub amount_clp: i32,
    pub note: String,
}

#[async_trait]
pub trait PaymentProvider: Send + Sync {
    async fn authorize_ride(&self, request: PaymentRequest) -> Result<PaymentResult, PaymentError>;
    async fn capture_ride(&self, ride_id: &Uuid, amount_clp: i32) -> Result<PaymentResult, PaymentError>;
}

// ----------------------------------------------------------------------------
// Sandbox / Mock Payment Adapter (Explicitly NO real charges)
// ----------------------------------------------------------------------------
pub struct MockPaymentProvider;

impl MockPaymentProvider {
    pub fn new() -> Self {
        Self
    }
}

impl Default for MockPaymentProvider {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl PaymentProvider for MockPaymentProvider {
    async fn authorize_ride(&self, request: PaymentRequest) -> Result<PaymentResult, PaymentError> {
        info!(
            target: "zuvio_payment",
            ride_id = %request.ride_id,
            amount = request.amount_clp,
            currency = %request.currency,
            "[MOCK PAYMENT] Autorización de pago simulada exitosa. Sin cobro bancario real."
        );

        Ok(PaymentResult {
            transaction_id: format!("mock_tx_{}", Uuid::new_v4().simple()),
            is_success: true,
            status: "authorized_mock".to_string(),
            amount_clp: request.amount_clp,
            note: "Operación simulada en entorno de desarrollo Zuvio. Integración real con pasarela pendiente de selección.".to_string(),
        })
    }

    async fn capture_ride(&self, ride_id: &Uuid, amount_clp: i32) -> Result<PaymentResult, PaymentError> {
        info!(
            target: "zuvio_payment",
            ride_id = %ride_id,
            amount = amount_clp,
            "[MOCK PAYMENT] Captura de tarifa de viaje simulada. Sin cobro bancario real."
        );

        Ok(PaymentResult {
            transaction_id: format!("mock_tx_{}", Uuid::new_v4().simple()),
            is_success: true,
            status: "captured_mock".to_string(),
            amount_clp,
            note: "Tarifa registrada en modo demo. Ningún cargo financiero ejecutado.".to_string(),
        })
    }
}

