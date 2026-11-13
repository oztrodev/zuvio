use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Role {
    Passenger,
    Driver,
    Admin,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub full_name: String,
    pub phone: Option<String>,
    pub role: Role,
    pub avatar_url: Option<String>,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DriverStatus {
    PendingReview,
    Approved,
    Rejected,
    Suspended,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DriverProfile {
    pub id: Uuid,
    pub user_id: Uuid,
    pub status: DriverStatus,
    pub license_number: String,
    pub license_class: String, // e.g. "A2", "A3" in Chile
    pub background_check_status: String,
    pub rating_avg: f64,
    pub total_trips: i32,
    pub is_online: bool,
    pub coverage_radius_km: f64, // Configurable coverage radius (e.g. 1 to 30 km)
    pub current_lat: Option<f64>,
    pub current_lng: Option<f64>,
    pub last_seen_at: Option<DateTime<Utc>>,
    pub reviewed_at: Option<DateTime<Utc>>,
    pub reviewed_by: Option<Uuid>,
    pub review_notes: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NearbyDriver {
    pub driver_id: Uuid,
    pub user_id: Uuid,
    pub full_name: String,
    pub rating_avg: f64,
    pub total_trips: i32,
    pub latitude: f64,
    pub longitude: f64,
    pub coverage_radius_km: f64,
    pub distance_km: f64,
    pub estimated_arrival_minutes: i32,
    pub vehicle_brand: String,
    pub vehicle_model: String,
    pub license_plate: String,
    pub battery_kwh: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum PropulsionType {
    BEV, // Battery Electric Vehicle - 100% Electric Only
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum VehicleStatus {
    PendingReview,
    Approved,
    Rejected,
    Inactive,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Vehicle {
    pub id: Uuid,
    pub driver_id: Uuid,
    pub brand: String,
    pub model: String,
    pub year: i32,
    pub license_plate: String, // Chilean format: ABCD-12 or AB-12-34
    pub propulsion: PropulsionType,
    pub battery_capacity_kwh: f64,
    pub passenger_capacity: i32,
    pub status: VehicleStatus,
    pub is_active: bool,
    pub verified_at: Option<DateTime<Utc>>,
    pub verified_by: Option<Uuid>,
    pub verification_notes: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DriverDocument {
    pub id: Uuid,
    pub driver_id: Uuid,
    pub doc_type: String, // driver_license, vehicle_registration, ev_homologation, background_check
    pub storage_path: String,
    pub original_filename: String,
    pub status: String, // pending, verified, rejected
    pub download_url: Option<String>, // short-lived signed URL
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RideStatus {
    Requested,
    Matching,
    Accepted,
    DriverEnRoute,
    Arrived,
    InProgress,
    Completed,
    Cancelled,
    Expired,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ride {
    pub id: Uuid,
    pub passenger_id: Uuid,
    pub driver_id: Option<Uuid>,
    pub vehicle_id: Option<Uuid>,
    pub status: RideStatus,

    // Location
    pub origin_address: String,
    pub origin_commune: String,
    pub origin_lat: f64,
    pub origin_lng: f64,
    pub destination_address: String,
    pub destination_commune: String,
    pub destination_lat: f64,
    pub destination_lng: f64,

    // Estimation & Pricing (CLP)
    pub distance_km: f64,
    pub duration_minutes: i32,
    pub estimated_fare_clp: i32,
    pub final_fare_clp: Option<i32>,
    pub co2_saved_kg: f64,

    // Cancellation
    pub cancellation_reason: Option<String>,
    pub cancelled_by: Option<Uuid>,

    // Timestamps
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub accepted_at: Option<DateTime<Utc>>,
    pub driver_arrived_at: Option<DateTime<Utc>>,
    pub started_at: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
    pub cancelled_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RideEvent {
    pub id: Uuid,
    pub ride_id: Uuid,
    pub previous_status: Option<RideStatus>,
    pub new_status: RideStatus,
    pub triggered_by: Option<Uuid>,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RideRating {
    pub id: Uuid,
    pub ride_id: Uuid,
    pub rater_id: Uuid,
    pub rated_user_id: Uuid,
    pub score: i32, // 1 to 5
    pub comment: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TicketStatus {
    Open,
    InProgress,
    Resolved,
    Closed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SupportTicket {
    pub id: Uuid,
    pub user_id: Uuid,
    pub ride_id: Option<Uuid>,
    pub subject: String,
    pub message: String,
    pub status: TicketStatus,
    pub resolution_notes: Option<String>,
    pub resolved_by: Option<Uuid>,
    pub resolved_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketingConsent {
    pub user_id: Uuid,
    pub service_notifications: bool,
    pub marketing_emails: bool,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditLog {
    pub id: Uuid,
    pub actor_id: Option<Uuid>,
    pub action: String,
    pub target_entity: String,
    pub target_id: Option<Uuid>,
    pub details: serde_json::Value,
    pub ip_address: Option<String>,
    pub created_at: DateTime<Utc>,
}
