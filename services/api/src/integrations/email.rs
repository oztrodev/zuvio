use async_trait::async_trait;
use parking_lot::RwLock;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use thiserror::Error;
use tracing::{error, info, warn};

#[derive(Debug, Error)]
pub enum EmailError {
    #[error("Error al enviar correo vía Brevo: {0}")]
    ProviderError(String),

    #[error("Error de serialización: {0}")]
    SerializationError(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmailPayload {
    pub to_email: String,
    pub to_name: String,
    pub subject: String,
    pub html_content: String,
    pub text_content: Option<String>,
}

#[async_trait]
pub trait EmailProvider: Send + Sync {
    async fn send_email(&self, payload: EmailPayload) -> Result<(), EmailError>;
}

// ----------------------------------------------------------------------------
// Brevo (Sendinblue) Transactional REST v3 Client
// ----------------------------------------------------------------------------
pub struct BrevoEmailProvider {
    client: reqwest::Client,
    api_key: String,
    sender_email: String,
    sender_name: String,
}

impl BrevoEmailProvider {
    pub fn new(api_key: String, sender_email: String, sender_name: String) -> Self {
        Self {
            client: reqwest::Client::builder()
                .timeout(std::time::Duration::from_secs(10))
                .build()
                .unwrap_or_default(),
            api_key,
            sender_email,
            sender_name,
        }
    }
}

#[async_trait]
impl EmailProvider for BrevoEmailProvider {
    async fn send_email(&self, payload: EmailPayload) -> Result<(), EmailError> {
        let url = "https://api.brevo.com/v3/smtp/email";

        let body = serde_json::json!({
            "sender": {
                "name": self.sender_name,
                "email": self.sender_email
            },
            "to": [
                {
                    "email": payload.to_email,
                    "name": payload.to_name
                }
            ],
            "subject": payload.subject,
            "htmlContent": payload.html_content,
            "textContent": payload.text_content
        });

        let mut attempts = 0;
        let max_retries = 2;

        loop {
            attempts += 1;
            let res = self
                .client
                .post(url)
                .header("api-key", &self.api_key)
                .header("Content-Type", "application/json")
                .json(&body)
                .send()
                .await;

            match res {
                Ok(response) if response.status().is_success() => {
                    info!(
                        target: "zuvio_email",
                        to = %payload.to_email,
                        subject = %payload.subject,
                        "Correo transaccional enviado exitosamente vía Brevo"
                    );
                    return Ok(());
                }
                Ok(response) => {
                    let status = response.status();
                    let text = response.text().await.unwrap_or_default();
                    warn!(
                        target: "zuvio_email",
                        status = %status,
                        attempt = attempts,
                        "Intento fallido de envío de correo Brevo: {}",
                        text
                    );
                }
                Err(e) => {
                    error!(
                        target: "zuvio_email",
                        attempt = attempts,
                        "Error de conexión con Brevo: {}",
                        e
                    );
                }
            }

            if attempts >= max_retries {
                return Err(EmailError::ProviderError(format!(
                    "Fallaron los {} intentos de envío vía Brevo",
                    max_retries
                )));
            }

            tokio::time::sleep(std::time::Duration::from_millis(500)).await;
        }
    }
}

// ----------------------------------------------------------------------------
// Mock Email Provider (Zero-Dependency & Demo Mode)
// ----------------------------------------------------------------------------
pub struct MockEmailProvider {
    pub sent_emails: Arc<RwLock<Vec<EmailPayload>>>,
}

impl MockEmailProvider {
    pub fn new() -> Self {
        Self {
            sent_emails: Arc::new(RwLock::new(Vec::new())),
        }
    }
}

impl Default for MockEmailProvider {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl EmailProvider for MockEmailProvider {
    async fn send_email(&self, payload: EmailPayload) -> Result<(), EmailError> {
        info!(
            target: "zuvio_mock_email",
            to = %payload.to_email,
            name = %payload.to_name,
            subject = %payload.subject,
            "[MODO DEMO] Correo transaccional simulado (no enviado a red real)"
        );
        self.sent_emails.write().push(payload);
        Ok(())
    }
}

