use axum::{
    http::{header, HeaderValue, Method},
    routing::{get, post},
    Router,
};
use std::net::SocketAddr;
use std::sync::Arc;
use tower_http::cors::{AllowOrigin, CorsLayer};
use tower_http::trace::TraceLayer;
use tracing::info;
use zuvio_api::{
    auth::AppState,
    config::AppConfig,
    integrations::{
        BrevoEmailProvider, DemoSantiagoMapProvider, EmailProvider, MockEmailProvider,
        MockPaymentProvider,
    },
    repository::InMemoryRepository,
    routes,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();

    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "info,zuvio_api=debug,tower_http=debug".into()),
        )
        .init();

    let config = AppConfig::from_env();
    info!(
        service = "Zuvio Ride-Hailing API",
        version = "0.1.0",
        market = %config.market_currency,
        environment = %config.environment,
        demo_mode = config.demo_mode,
        "Iniciando plataforma de movilidad eléctrica Zuvio..."
    );

    let repo = Arc::new(InMemoryRepository::new_with_demo_data());

    let email_provider: Arc<dyn EmailProvider> = if let Some(ref api_key) = config.brevo_api_key {
        info!("Proveedor de correo Brevo configurado para correos transaccionales.");
        Arc::new(BrevoEmailProvider::new(
            api_key.clone(),
            config.brevo_sender_email.clone(),
            config.brevo_sender_name.clone(),
        ))
    } else {
        info!("Brevo API key no configurada. Activando MockEmailProvider para desarrollo y pruebas.");
        Arc::new(MockEmailProvider::new())
    };

    let map_provider = Arc::new(DemoSantiagoMapProvider::new());
    let payment_provider = Arc::new(MockPaymentProvider::new());

    let state = AppState {
        repo,
        email_provider,
        map_provider,
        payment_provider,
        config: config.clone(),
    };

    // CORS definido por CORS_ALLOWED_ORIGINS (lista separada por comas). "*" permite cualquier origen (solo demo).
    let allow_origin = if config.cors_allowed_origins.iter().any(|o| o == "*") {
        tracing::warn!("CORS abierto a cualquier origen (\"*\"). Usar solo en demo.");
        AllowOrigin::any()
    } else {
        let origins: Vec<HeaderValue> = config
            .cors_allowed_origins
            .iter()
            .filter_map(|o| o.parse::<HeaderValue>().ok())
            .collect();
        info!(allowed_origins = origins.len(), "CORS restringido a orígenes configurados.");
        AllowOrigin::list(origins)
    };

    let cors = CorsLayer::new()
        .allow_origin(allow_origin)
        .allow_methods([
            Method::GET,
            Method::POST,
            Method::PUT,
            Method::PATCH,
            Method::DELETE,
            Method::OPTIONS,
        ])
        .allow_headers([
            header::AUTHORIZATION,
            header::CONTENT_TYPE,
            header::ACCEPT,
            header::HeaderName::from_static("x-user-role"),
            header::HeaderName::from_static("x-user-id"),
        ]);

    let api_routes = Router::new()
        .route("/health", get(routes::health::health_check))
        .route("/config/market", get(routes::config_routes::get_market_config))
        .route("/auth/demo-session", post(routes::auth_routes::demo_session))
        .route("/profile", get(routes::auth_routes::get_current_profile))
        .route("/rides/estimate", post(routes::rides_routes::estimate_ride))
        .route("/rides", post(routes::rides_routes::create_ride))
        .route("/rides", get(routes::rides_routes::list_rides))
        .route("/rides/active", get(routes::rides_routes::get_active_ride))
        .route("/rides/:id", get(routes::rides_routes::get_ride_by_id))
        .route("/rides/:id/accept", post(routes::rides_routes::accept_ride))
        .route("/rides/:id/status", post(routes::rides_routes::update_ride_status))
        .route("/rides/:id/cancel", post(routes::rides_routes::cancel_ride))
        .route("/rides/:id/rate", post(routes::rides_routes::rate_ride))
        // Drivers & Nearby
        .route("/drivers/me", get(routes::drivers_routes::get_driver_profile))
        .route("/drivers/register-ev", post(routes::drivers_routes::register_electric_vehicle))
        .route("/drivers/availability", post(routes::drivers_routes::toggle_availability))
        .route("/drivers/coverage-radius", post(routes::drivers_routes::update_coverage_radius))
        .route("/drivers/nearby", get(routes::drivers_routes::get_nearby_drivers))
        .route("/drivers/available-rides", get(routes::drivers_routes::get_available_rides))
        // Admin
        .route("/admin/pending-drivers", get(routes::admin_routes::list_pending_drivers))
        .route("/admin/drivers/:id/review", post(routes::admin_routes::review_driver))
        .route("/admin/audit-logs", get(routes::admin_routes::list_audit_logs))
        // Support
        .route("/support/tickets", post(routes::support_routes::create_ticket))
        .route("/support/tickets", get(routes::support_routes::list_tickets))
        .route("/admin/tickets/:id/resolve", post(routes::support_routes::resolve_ticket))
        .with_state(state);

    let app = Router::new()
        .nest("/api/v1", api_routes)
        .layer(cors)
        .layer(TraceLayer::new_for_http());

    let addr = SocketAddr::from(([0, 0, 0, 0], config.port));
    info!("Servidor Zuvio escuchando en http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
