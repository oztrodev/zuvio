use axum::{extract::State, Json};
use serde_json::{json, Value};
use crate::auth::AppState;

pub async fn get_market_config(State(state): State<AppState>) -> Json<Value> {
    Json(json!({
        "market": "Santiago de Chile",
        "currency": state.config.market_currency,
        "timezone": state.config.market_timezone,
        "pricing": {
            "base_fare_clp": state.config.base_fare_clp,
            "per_km_clp": state.config.per_km_clp,
            "per_minute_clp": state.config.per_minute_clp,
            "minimum_fare_clp": state.config.minimum_fare_clp,
            "cancellation_penalty_clp": state.config.cancellation_penalty_clp,
            "status": "Configuración demo sujeta a confirmación del propietario"
        },
        "service_communes": [
            "Providencia",
            "Las Condes",
            "Vitacura",
            "Santiago Centro",
            "Ñuñoa",
            "Lo Barnechea",
            "La Reina",
            "Macul",
            "Peñalolén",
            "San Miguel"
        ],
        "ev_policy": {
            "allowed_propulsion": ["BEV"],
            "minimum_battery_kwh": 30.0,
            "minimum_vehicle_year": 2018,
            "zero_emissions_focus": "Flota 100% eléctrica urbana"
        },
        "safety_guideline": "Tolerancia Cero: Prohibido conducir bajo la influencia del alcohol, drogas o cannabis. Prohibido fumar al interior del vehículo."
    }))
}

