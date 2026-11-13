use crate::config::AppConfig;

#[derive(Debug, Clone, PartialEq)]
pub struct FareEstimate {
    pub distance_km: f64,
    pub duration_minutes: i32,
    pub base_fare_clp: i32,
    pub distance_fare_clp: i32,
    pub time_fare_clp: i32,
    pub total_fare_clp: i32,
    pub minimum_fare_clp: i32,
    pub currency: String,
    pub co2_saved_kg: f64,
    pub is_minimum_applied: bool,
}

pub fn calculate_fare(
    distance_km: f64,
    duration_minutes: i32,
    config: &AppConfig,
) -> FareEstimate {
    let base = config.base_fare_clp;
    let dist_fare = (distance_km * (config.per_km_clp as f64)).round() as i32;
    let time_fare = duration_minutes * config.per_minute_clp;

    let subtotal = base + dist_fare + time_fare;
    
    // Round to nearest 100 CLP in Chilean Pesos
    let rounded_subtotal = ((subtotal + 50) / 100) * 100;
    
    let is_minimum_applied = rounded_subtotal < config.minimum_fare_clp;
    let total_fare_clp = if is_minimum_applied {
        config.minimum_fare_clp
    } else {
        rounded_subtotal
    };

    // CO2 savings: ~0.160 kg CO2 / km avoided vs combustion car
    let co2_saved_kg = (distance_km * 0.160 * 100.0).round() / 100.0;

    FareEstimate {
        distance_km,
        duration_minutes,
        base_fare_clp: base,
        distance_fare_clp: dist_fare,
        time_fare_clp: time_fare,
        total_fare_clp,
        minimum_fare_clp: config.minimum_fare_clp,
        currency: config.market_currency.clone(),
        co2_saved_kg,
        is_minimum_applied,
    }
}

