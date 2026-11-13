use crate::domain::models::*;
use thiserror::Error;
use uuid::Uuid;

#[derive(Debug, Error)]
pub enum RepoError {
    #[error("Recurso no encontrado")]
    NotFound,

    #[error("Conflicto de concurrencia: el viaje ya fue asignado a otro conductor o no está disponible")]
    DoubleAssignmentConflict,

    #[error("El conductor no está aprobado o no cuenta con un vehículo eléctrico activo")]
    DriverNotEligible,

    #[error("El conductor ya tiene un viaje en curso activo")]
    DriverAlreadyCommitted,

    #[error("Error de validación: {0}")]
    ValidationError(String),

    #[error("Error de base de datos: {0}")]
    DatabaseError(String),
}

pub trait Repository: Send + Sync {
    fn find_user_by_id(&self, id: &Uuid) -> Result<Option<User>, RepoError>;
    fn find_user_by_email(&self, email: &str) -> Result<Option<User>, RepoError>;
    fn list_users(&self) -> Result<Vec<User>, RepoError>;

    fn find_driver_profile_by_user_id(&self, user_id: &Uuid) -> Result<Option<DriverProfile>, RepoError>;
    fn find_driver_profile_by_id(&self, id: &Uuid) -> Result<Option<DriverProfile>, RepoError>;
    fn update_driver_status(
        &self,
        driver_id: &Uuid,
        status: DriverStatus,
        notes: Option<String>,
        admin_id: &Uuid,
    ) -> Result<DriverProfile, RepoError>;
    fn set_driver_online(&self, driver_id: &Uuid, is_online: bool) -> Result<DriverProfile, RepoError>;
    fn set_driver_coverage_radius(&self, driver_id: &Uuid, radius_km: f64) -> Result<DriverProfile, RepoError>;
    fn get_nearby_drivers(&self, lat: f64, lng: f64, max_radius_km: f64) -> Result<Vec<NearbyDriver>, RepoError>;
    fn list_pending_drivers(&self) -> Result<Vec<(DriverProfile, User, Option<Vehicle>)>, RepoError>;
    fn list_active_drivers(&self) -> Result<Vec<(DriverProfile, User, Option<Vehicle>)>, RepoError>;

    fn find_vehicle_by_driver_id(&self, driver_id: &Uuid) -> Result<Option<Vehicle>, RepoError>;
    fn find_vehicle_by_id(&self, id: &Uuid) -> Result<Option<Vehicle>, RepoError>;
    fn create_vehicle(&self, vehicle: Vehicle) -> Result<Vehicle, RepoError>;
    fn update_vehicle_status(&self, vehicle_id: &Uuid, status: VehicleStatus, admin_id: &Uuid) -> Result<Vehicle, RepoError>;

    fn create_ride(&self, ride: Ride) -> Result<Ride, RepoError>;
    fn find_ride_by_id(&self, id: &Uuid) -> Result<Option<Ride>, RepoError>;
    fn find_active_ride_for_user(&self, user_id: &Uuid, role: Role) -> Result<Option<Ride>, RepoError>;
    fn list_rides_for_user(&self, user_id: &Uuid, role: Role) -> Result<Vec<Ride>, RepoError>;
    fn list_all_rides(&self) -> Result<Vec<Ride>, RepoError>;
    
    // Atomic assignment prevents race condition / double-booking
    fn accept_ride_atomic(
        &self,
        ride_id: &Uuid,
        driver_profile_id: &Uuid,
        vehicle_id: &Uuid,
    ) -> Result<Ride, RepoError>;

    fn update_ride_status(
        &self,
        ride_id: &Uuid,
        new_status: RideStatus,
        actor_id: &Uuid,
        actor_role: Role,
        cancellation_reason: Option<String>,
    ) -> Result<Ride, RepoError>;

    fn create_ride_rating(&self, rating: RideRating) -> Result<RideRating, RepoError>;
    fn list_ride_ratings_for_user(&self, user_id: &Uuid) -> Result<Vec<RideRating>, RepoError>;

    fn create_support_ticket(&self, ticket: SupportTicket) -> Result<SupportTicket, RepoError>;
    fn list_support_tickets_for_user(&self, user_id: &Uuid) -> Result<Vec<SupportTicket>, RepoError>;
    fn list_all_support_tickets(&self) -> Result<Vec<SupportTicket>, RepoError>;
    fn resolve_support_ticket(&self, ticket_id: &Uuid, resolution_notes: &str, admin_id: &Uuid) -> Result<SupportTicket, RepoError>;

    fn create_audit_log(&self, log: AuditLog) -> Result<(), RepoError>;
    fn list_audit_logs(&self) -> Result<Vec<AuditLog>, RepoError>;
}
