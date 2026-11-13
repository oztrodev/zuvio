use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RouteCalculation {
    pub distance_km: f64,
    pub duration_minutes: i32,
    pub is_demo: bool,
    pub provider_note: String,
}

#[async_trait]
pub trait MapProvider: Send + Sync {
    async fn calculate_route(
        &self,
        origin_lat: f64,
        origin_lng: f64,
        dest_lat: f64,
        dest_lng: f64,
    ) -> RouteCalculation;
}

pub struct DemoSantiagoMapProvider;

impl DemoSantiagoMapProvider {
    pub fn new() -> Self {
        Self
    }
}

impl Default for DemoSantiagoMapProvider {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl MapProvider for DemoSantiagoMapProvider {
    async fn calculate_route(
        &self,
        origin_lat: f64,
        origin_lng: f64,
        dest_lat: f64,
        dest_lng: f64,
    ) -> RouteCalculation {
        // Haversine formula calculation for Santiago coordinates
        let r_earth = 6371.0; // km
        let d_lat = (dest_lat - origin_lat).to_radians();
        let d_lng = (dest_lng - origin_lng).to_radians();

        let lat1 = origin_lat.to_radians();
        let lat2 = dest_lat.to_radians();

        let a = (d_lat / 2.0).sin().powi(2)
            + lat1.cos() * lat2.cos() * (d_lng / 2.0).sin().powi(2);
        let c = 2.0 * a.sqrt().atan2((1.0 - a).sqrt());
        let straight_km = r_earth * c;

        // Santiago urban road circuit factor ~ 1.35x
        let distance_km = ((straight_km * 1.35).max(1.2) * 10.0).round() / 10.0;
        
        // Urban Santiago average speed ~ 22 km/h + 3 min traffic base
        let duration_minutes = ((distance_km / 22.0 * 60.0) + 3.0).round() as i32;

        RouteCalculation {
            distance_km,
            duration_minutes: duration_minutes.max(5),
            is_demo: true,
            provider_note: "Cálculo estimado basado en topología vial de Santiago (Modo Demo sin proveedor comercial conectado).".to_string(),
        }
    }
}

