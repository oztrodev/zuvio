use std::env;

#[derive(Clone, Debug)]
pub struct AppConfig {
    pub port: u16,
    pub host: String,
    pub environment: String,
    pub demo_mode: bool,
    pub cors_allowed_origins: Vec<String>,
    
    // Santiago de Chile Market Fare Rules (CLP)
    pub base_fare_clp: i32,
    pub per_km_clp: i32,
    pub per_minute_clp: i32,
    pub minimum_fare_clp: i32,
    pub cancellation_penalty_clp: i32,
    pub market_currency: String,
    pub market_timezone: String,

    // Brevo Email
    pub brevo_api_key: Option<String>,
    pub brevo_sender_email: String,
    pub brevo_sender_name: String,
}

impl AppConfig {
    pub fn from_env() -> Self {
        let port = env::var("PORT")
            .ok()
            .and_then(|p| p.parse().ok())
            .unwrap_or(4000);

        let host = env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
        let environment = env::var("ENVIRONMENT").unwrap_or_else(|_| "development".to_string());
        let demo_mode = env::var("DEMO_MODE")
            .map(|v| v.to_lowercase() == "true" || v == "1")
            .unwrap_or(true);

        let cors_allowed_origins = env::var("CORS_ALLOWED_ORIGINS")
            .unwrap_or_else(|_| "http://localhost:5173,http://127.0.0.1:5173,http://localhost:3000".to_string())
            .split(',')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect();

        let base_fare_clp = env::var("BASE_FARE_CLP")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(1400);

        let per_km_clp = env::var("PER_KM_CLP")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(650);

        let per_minute_clp = env::var("PER_MINUTE_CLP")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(150);

        let minimum_fare_clp = env::var("MINIMUM_FARE_CLP")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(3000);

        let cancellation_penalty_clp = env::var("CANCELLATION_PENALTY_CLP")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(1500);

        let brevo_api_key = env::var("BREVO_API_KEY")
            .ok()
            .filter(|k| !k.trim().is_empty());

        let brevo_sender_email = env::var("BREVO_SENDER_EMAIL")
            .unwrap_or_else(|_| "hola@zuvio.cl".to_string());

        let brevo_sender_name = env::var("BREVO_SENDER_NAME")
            .unwrap_or_else(|_| "Zuvio Chile".to_string());

        Self {
            port,
            host,
            environment,
            demo_mode,
            cors_allowed_origins,
            base_fare_clp,
            per_km_clp,
            per_minute_clp,
            minimum_fare_clp,
            cancellation_penalty_clp,
            market_currency: "CLP".to_string(),
            market_timezone: "America/Santiago".to_string(),
            brevo_api_key,
            brevo_sender_email,
            brevo_sender_name,
        }
    }
}

