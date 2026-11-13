use crate::domain::models::*;
use crate::domain::state_machine::validate_transition;
use crate::repository::traits::{RepoError, Repository};
use chrono::Utc;
use parking_lot::RwLock;
use std::collections::HashMap;
use std::sync::Arc;
use uuid::Uuid;

pub struct InMemoryRepository {
    users: Arc<RwLock<HashMap<Uuid, User>>>,
    driver_profiles: Arc<RwLock<HashMap<Uuid, DriverProfile>>>,
    vehicles: Arc<RwLock<HashMap<Uuid, Vehicle>>>,
    #[allow(dead_code)]
    driver_documents: Arc<RwLock<HashMap<Uuid, DriverDocument>>>,
    rides: Arc<RwLock<HashMap<Uuid, Ride>>>,
    ride_events: Arc<RwLock<Vec<RideEvent>>>,
    ratings: Arc<RwLock<Vec<RideRating>>>,
    support_tickets: Arc<RwLock<HashMap<Uuid, SupportTicket>>>,
    audit_logs: Arc<RwLock<Vec<AuditLog>>>,
}

impl InMemoryRepository {
    pub fn new_with_demo_data() -> Self {
        let repo = Self {
            users: Arc::new(RwLock::new(HashMap::new())),
            driver_profiles: Arc::new(RwLock::new(HashMap::new())),
            vehicles: Arc::new(RwLock::new(HashMap::new())),
            driver_documents: Arc::new(RwLock::new(HashMap::new())),
            rides: Arc::new(RwLock::new(HashMap::new())),
            ride_events: Arc::new(RwLock::new(Vec::new())),
            ratings: Arc::new(RwLock::new(Vec::new())),
            support_tickets: Arc::new(RwLock::new(HashMap::new())),
            audit_logs: Arc::new(RwLock::new(Vec::new())),
        };

        repo.seed_demo_data();
        repo
    }

    fn seed_demo_data(&self) {
        let now = Utc::now();

        let admin_id = Uuid::parse_str("00000000-0000-0000-0000-000000000001").unwrap();
        let passenger1_id = Uuid::parse_str("00000000-0000-0000-0000-000000000002").unwrap();
        let passenger2_id = Uuid::parse_str("00000000-0000-0000-0000-000000000003").unwrap();
        let driver1_user_id = Uuid::parse_str("00000000-0000-0000-0000-000000000010").unwrap();
        let driver2_user_id = Uuid::parse_str("00000000-0000-0000-0000-000000000020").unwrap();
        let driver3_user_id = Uuid::parse_str("00000000-0000-0000-0000-000000000030").unwrap();

        let users = vec![
            User {
                id: admin_id,
                email: "admin@zuvio.cl".to_string(),
                full_name: "Carlos Morales (Operaciones Zuvio)".to_string(),
                phone: Some("+56 9 8888 1111".to_string()),
                role: Role::Admin,
                avatar_url: None,
                is_active: true,
                created_at: now,
                updated_at: now,
            },
            User {
                id: passenger1_id,
                email: "pasajero@zuvio.cl".to_string(),
                full_name: "Valentina Silva".to_string(),
                phone: Some("+56 9 9123 4567".to_string()),
                role: Role::Passenger,
                avatar_url: None,
                is_active: true,
                created_at: now,
                updated_at: now,
            },
            User {
                id: passenger2_id,
                email: "rider2@zuvio.cl".to_string(),
                full_name: "Matías Gómez".to_string(),
                phone: Some("+56 9 7654 3210".to_string()),
                role: Role::Passenger,
                avatar_url: None,
                is_active: true,
                created_at: now,
                updated_at: now,
            },
            User {
                id: driver1_user_id,
                email: "conductor1@zuvio.cl".to_string(),
                full_name: "Rodrigo Araya".to_string(),
                phone: Some("+56 9 8111 2222".to_string()),
                role: Role::Driver,
                avatar_url: None,
                is_active: true,
                created_at: now,
                updated_at: now,
            },
            User {
                id: driver2_user_id,
                email: "conductor2@zuvio.cl".to_string(),
                full_name: "Camila Soto".to_string(),
                phone: Some("+56 9 8222 3333".to_string()),
                role: Role::Driver,
                avatar_url: None,
                is_active: true,
                created_at: now,
                updated_at: now,
            },
            User {
                id: driver3_user_id,
                email: "conductor.nuevo@zuvio.cl".to_string(),
                full_name: "Esteban Fuentes".to_string(),
                phone: Some("+56 9 8333 4444".to_string()),
                role: Role::Driver,
                avatar_url: None,
                is_active: true,
                created_at: now,
                updated_at: now,
            },
        ];

        for u in users {
            self.users.write().insert(u.id, u);
        }

        let driver1_profile_id = Uuid::parse_str("10000000-0000-0000-0000-000000000001").unwrap();
        let driver2_profile_id = Uuid::parse_str("10000000-0000-0000-0000-000000000002").unwrap();
        let driver3_profile_id = Uuid::parse_str("10000000-0000-0000-0000-000000000003").unwrap();

        let driver_profiles = vec![
            DriverProfile {
                id: driver1_profile_id,
                user_id: driver1_user_id,
                status: DriverStatus::Approved,
                license_number: "15.489.231-K".to_string(),
                license_class: "A2".to_string(),
                background_check_status: "approved".to_string(),
                rating_avg: 4.96,
                total_trips: 142,
                is_online: true,
                coverage_radius_km: 8.0,
                current_lat: Some(-33.424500),
                current_lng: Some(-70.612000),
                last_seen_at: Some(now),
                reviewed_at: Some(now),
                reviewed_by: Some(admin_id),
                review_notes: Some("Documentación y antecedentes verificados para Ley EAT.".to_string()),
                created_at: now,
                updated_at: now,
            },
            DriverProfile {
                id: driver2_profile_id,
                user_id: driver2_user_id,
                status: DriverStatus::Approved,
                license_number: "16.712.980-3".to_string(),
                license_class: "A2".to_string(),
                background_check_status: "approved".to_string(),
                rating_avg: 4.92,
                total_trips: 89,
                is_online: true,
                coverage_radius_km: 10.0,
                current_lat: Some(-33.411000),
                current_lng: Some(-70.589000),
                last_seen_at: Some(now),
                reviewed_at: Some(now),
                reviewed_by: Some(admin_id),
                review_notes: Some("Aprobada para flota eléctrica Zuvio.".to_string()),
                created_at: now,
                updated_at: now,
            },
            DriverProfile {
                id: driver3_profile_id,
                user_id: driver3_user_id,
                status: DriverStatus::PendingReview,
                license_number: "18.123.456-7".to_string(),
                license_class: "A2".to_string(),
                background_check_status: "pending".to_string(),
                rating_avg: 5.0,
                total_trips: 0,
                is_online: false,
                coverage_radius_km: 5.0,
                current_lat: Some(-33.448900),
                current_lng: Some(-70.669300),
                last_seen_at: Some(now),
                reviewed_at: None,
                reviewed_by: None,
                review_notes: None,
                created_at: now,
                updated_at: now,
            },
        ];

        for dp in driver_profiles {
            self.driver_profiles.write().insert(dp.id, dp);
        }

        let vehicle1_id = Uuid::parse_str("20000000-0000-0000-0000-000000000001").unwrap();
        let vehicle2_id = Uuid::parse_str("20000000-0000-0000-0000-000000000002").unwrap();
        let vehicle3_id = Uuid::parse_str("20000000-0000-0000-0000-000000000003").unwrap();

        let vehicles = vec![
            Vehicle {
                id: vehicle1_id,
                driver_id: driver1_profile_id,
                brand: "Tesla".to_string(),
                model: "Model 3 Standard Range".to_string(),
                year: 2023,
                license_plate: "LW-72-91".to_string(),
                propulsion: PropulsionType::BEV,
                battery_capacity_kwh: 60.0,
                passenger_capacity: 4,
                status: VehicleStatus::Approved,
                is_active: true,
                verified_at: Some(now),
                verified_by: Some(admin_id),
                verification_notes: Some("Homologación eléctrica 3CV verificada.".to_string()),
                created_at: now,
                updated_at: now,
            },
            Vehicle {
                id: vehicle2_id,
                driver_id: driver2_profile_id,
                brand: "BYD".to_string(),
                model: "Dolphin EV".to_string(),
                year: 2024,
                license_plate: "PP-45-12".to_string(),
                propulsion: PropulsionType::BEV,
                battery_capacity_kwh: 44.9,
                passenger_capacity: 4,
                status: VehicleStatus::Approved,
                is_active: true,
                verified_at: Some(now),
                verified_by: Some(admin_id),
                verification_notes: Some("Vehículo eléctrico urbano verificado.".to_string()),
                created_at: now,
                updated_at: now,
            },
            Vehicle {
                id: vehicle3_id,
                driver_id: driver3_profile_id,
                brand: "Hyundai".to_string(),
                model: "Ioniq 5".to_string(),
                year: 2023,
                license_plate: "TG-88-34".to_string(),
                propulsion: PropulsionType::BEV,
                battery_capacity_kwh: 72.6,
                passenger_capacity: 4,
                status: VehicleStatus::PendingReview,
                is_active: false,
                verified_at: None,
                verified_by: None,
                verification_notes: None,
                created_at: now,
                updated_at: now,
            },
        ];

        for v in vehicles {
            self.vehicles.write().insert(v.id, v);
        }

        let sample_ride_id = Uuid::parse_str("30000000-0000-0000-0000-000000000001").unwrap();
        let sample_ride = Ride {
            id: sample_ride_id,
            passenger_id: passenger1_id,
            driver_id: Some(driver1_profile_id),
            vehicle_id: Some(vehicle1_id),
            status: RideStatus::Completed,
            origin_address: "Av. Providencia 1234 (Metro Manuel Montt)".to_string(),
            origin_commune: "Providencia".to_string(),
            origin_lat: -33.4285,
            origin_lng: -70.6189,
            destination_address: "Av. Apoquindo 4500 (Metro Escuela Militar)".to_string(),
            destination_commune: "Las Condes".to_string(),
            destination_lat: -33.4150,
            destination_lng: -70.5790,
            distance_km: 5.4,
            duration_minutes: 16,
            estimated_fare_clp: 5200,
            final_fare_clp: Some(5200),
            co2_saved_kg: 0.86,
            cancellation_reason: None,
            cancelled_by: None,
            created_at: now - chrono::Duration::hours(2),
            updated_at: now - chrono::Duration::hours(1),
            accepted_at: Some(now - chrono::Duration::hours(2) + chrono::Duration::minutes(2)),
            driver_arrived_at: Some(now - chrono::Duration::hours(2) + chrono::Duration::minutes(7)),
            started_at: Some(now - chrono::Duration::hours(2) + chrono::Duration::minutes(10)),
            completed_at: Some(now - chrono::Duration::hours(1) + chrono::Duration::minutes(44)),
            cancelled_at: None,
        };

        self.rides.write().insert(sample_ride.id, sample_ride);

        self.ratings.write().push(RideRating {
            id: Uuid::new_v4(),
            ride_id: sample_ride_id,
            rater_id: passenger1_id,
            rated_user_id: driver1_user_id,
            score: 5,
            comment: Some("Excelente viaje en auto eléctrico, suave, limpio y silencioso.".to_string()),
            created_at: now,
        });

        let ticket_id = Uuid::new_v4();
        self.support_tickets.write().insert(
            ticket_id,
            SupportTicket {
                id: ticket_id,
                user_id: passenger1_id,
                ride_id: Some(sample_ride_id),
                subject: "Consulta sobre comprobante digital de viaje".to_string(),
                message: "Hola, me gustaría saber si puedo descargar la boleta electrónica de mi viaje.".to_string(),
                status: TicketStatus::Resolved,
                resolution_notes: Some("Comprobante enviado al correo registrado de acuerdo con normativa SII.".to_string()),
                resolved_by: Some(admin_id),
                resolved_at: Some(now),
                created_at: now - chrono::Duration::hours(1),
                updated_at: now,
            },
        );

        self.audit_logs.write().push(AuditLog {
            id: Uuid::new_v4(),
            actor_id: Some(admin_id),
            action: "APPROVE_DRIVER_EV".to_string(),
            target_entity: "driver_profile".to_string(),
            target_id: Some(driver1_profile_id),
            details: serde_json::json!({
                "vehicle": "Tesla Model 3",
                "propulsion": "BEV",
                "plate": "LW-72-91",
                "status": "approved"
            }),
            ip_address: Some("127.0.0.1".to_string()),
            created_at: now,
        });
    }
}

// Distance helper using Haversine formula
fn haversine_distance_km(lat1: f64, lng1: f64, lat2: f64, lng2: f64) -> f64 {
    let r_earth = 6371.0;
    let d_lat = (lat2 - lat1).to_radians();
    let d_lng = (lng2 - lng1).to_radians();
    let a = (d_lat / 2.0).sin().powi(2)
        + lat1.to_radians().cos() * lat2.to_radians().cos() * (d_lng / 2.0).sin().powi(2);
    let c = 2.0 * a.sqrt().atan2((1.0 - a).sqrt());
    ((r_earth * c) * 100.0).round() / 100.0
}

impl Repository for InMemoryRepository {
    fn find_user_by_id(&self, id: &Uuid) -> Result<Option<User>, RepoError> {
        Ok(self.users.read().get(id).cloned())
    }

    fn find_user_by_email(&self, email: &str) -> Result<Option<User>, RepoError> {
        let needle = email.trim().to_lowercase();
        Ok(self
            .users
            .read()
            .values()
            .find(|u| u.email.to_lowercase() == needle)
            .cloned())
    }

    fn list_users(&self) -> Result<Vec<User>, RepoError> {
        Ok(self.users.read().values().cloned().collect())
    }

    fn find_driver_profile_by_user_id(&self, user_id: &Uuid) -> Result<Option<DriverProfile>, RepoError> {
        Ok(self
            .driver_profiles
            .read()
            .values()
            .find(|dp| dp.user_id == *user_id)
            .cloned())
    }

    fn find_driver_profile_by_id(&self, id: &Uuid) -> Result<Option<DriverProfile>, RepoError> {
        Ok(self.driver_profiles.read().get(id).cloned())
    }

    fn update_driver_status(
        &self,
        driver_id: &Uuid,
        status: DriverStatus,
        notes: Option<String>,
        admin_id: &Uuid,
    ) -> Result<DriverProfile, RepoError> {
        let mut profiles = self.driver_profiles.write();
        let profile = profiles.get_mut(driver_id).ok_or(RepoError::NotFound)?;
        profile.status = status;
        profile.reviewed_at = Some(Utc::now());
        profile.reviewed_by = Some(*admin_id);
        profile.review_notes = notes;
        profile.updated_at = Utc::now();
        Ok(profile.clone())
    }

    fn set_driver_online(&self, driver_id: &Uuid, is_online: bool) -> Result<DriverProfile, RepoError> {
        let mut profiles = self.driver_profiles.write();
        let profile = profiles.get_mut(driver_id).ok_or(RepoError::NotFound)?;
        
        if is_online && profile.status != DriverStatus::Approved {
            return Err(RepoError::DriverNotEligible);
        }

        profile.is_online = is_online;
        profile.last_seen_at = Some(Utc::now());
        profile.updated_at = Utc::now();
        Ok(profile.clone())
    }

    fn set_driver_coverage_radius(&self, driver_id: &Uuid, radius_km: f64) -> Result<DriverProfile, RepoError> {
        if radius_km < 1.0 || radius_km > 50.0 {
            return Err(RepoError::ValidationError("El radio debe estar entre 1 y 50 km".to_string()));
        }
        let mut profiles = self.driver_profiles.write();
        let profile = profiles.get_mut(driver_id).ok_or(RepoError::NotFound)?;
        profile.coverage_radius_km = radius_km;
        profile.updated_at = Utc::now();
        Ok(profile.clone())
    }

    fn get_nearby_drivers(&self, lat: f64, lng: f64, max_radius_km: f64) -> Result<Vec<NearbyDriver>, RepoError> {
        let profiles = self.driver_profiles.read();
        let users = self.users.read();
        let vehicles = self.vehicles.read();

        let mut results = Vec::new();

        for dp in profiles.values() {
            if dp.status != DriverStatus::Approved || !dp.is_online {
                continue;
            }

            if let (Some(d_lat), Some(d_lng)) = (dp.current_lat, dp.current_lng) {
                let dist = haversine_distance_km(lat, lng, d_lat, d_lng);
                let allowed_radius = dp.coverage_radius_km.min(max_radius_km);

                if dist <= allowed_radius {
                    if let Some(user) = users.get(&dp.user_id) {
                        if let Some(v) = vehicles.values().find(|v| v.driver_id == dp.id && v.status == VehicleStatus::Approved && v.is_active) {
                            let arrival_min = ((dist / 25.0 * 60.0).round() as i32).max(2);
                            results.push(NearbyDriver {
                                driver_id: dp.id,
                                user_id: dp.user_id,
                                full_name: user.full_name.clone(),
                                rating_avg: dp.rating_avg,
                                total_trips: dp.total_trips,
                                latitude: d_lat,
                                longitude: d_lng,
                                coverage_radius_km: dp.coverage_radius_km,
                                distance_km: dist,
                                estimated_arrival_minutes: arrival_min,
                                vehicle_brand: v.brand.clone(),
                                vehicle_model: v.model.clone(),
                                license_plate: v.license_plate.clone(),
                                battery_kwh: v.battery_capacity_kwh,
                            });
                        }
                    }
                }
            }
        }

        results.sort_by(|a, b| a.distance_km.partial_cmp(&b.distance_km).unwrap());
        Ok(results)
    }

    fn list_pending_drivers(&self) -> Result<Vec<(DriverProfile, User, Option<Vehicle>)>, RepoError> {
        let profiles = self.driver_profiles.read();
        let users = self.users.read();
        let vehicles = self.vehicles.read();

        let mut results = Vec::new();
        for dp in profiles.values().filter(|dp| dp.status == DriverStatus::PendingReview) {
            if let Some(user) = users.get(&dp.user_id) {
                let vehicle = vehicles.values().find(|v| v.driver_id == dp.id).cloned();
                results.push((dp.clone(), user.clone(), vehicle));
            }
        }
        Ok(results)
    }

    fn list_active_drivers(&self) -> Result<Vec<(DriverProfile, User, Option<Vehicle>)>, RepoError> {
        let profiles = self.driver_profiles.read();
        let users = self.users.read();
        let vehicles = self.vehicles.read();

        let mut results = Vec::new();
        for dp in profiles.values().filter(|dp| dp.status == DriverStatus::Approved) {
            if let Some(user) = users.get(&dp.user_id) {
                let vehicle = vehicles.values().find(|v| v.driver_id == dp.id).cloned();
                results.push((dp.clone(), user.clone(), vehicle));
            }
        }
        Ok(results)
    }

    fn find_vehicle_by_driver_id(&self, driver_id: &Uuid) -> Result<Option<Vehicle>, RepoError> {
        Ok(self
            .vehicles
            .read()
            .values()
            .find(|v| v.driver_id == *driver_id)
            .cloned())
    }

    fn find_vehicle_by_id(&self, id: &Uuid) -> Result<Option<Vehicle>, RepoError> {
        Ok(self.vehicles.read().get(id).cloned())
    }

    fn create_vehicle(&self, vehicle: Vehicle) -> Result<Vehicle, RepoError> {
        self.vehicles.write().insert(vehicle.id, vehicle.clone());
        Ok(vehicle)
    }

    fn update_vehicle_status(&self, vehicle_id: &Uuid, status: VehicleStatus, admin_id: &Uuid) -> Result<Vehicle, RepoError> {
        let mut vehicles = self.vehicles.write();
        let v = vehicles.get_mut(vehicle_id).ok_or(RepoError::NotFound)?;
        v.status = status;
        v.is_active = status == VehicleStatus::Approved;
        v.verified_at = Some(Utc::now());
        v.verified_by = Some(*admin_id);
        v.updated_at = Utc::now();
        Ok(v.clone())
    }

    fn create_ride(&self, ride: Ride) -> Result<Ride, RepoError> {
        self.rides.write().insert(ride.id, ride.clone());
        self.ride_events.write().push(RideEvent {
            id: Uuid::new_v4(),
            ride_id: ride.id,
            previous_status: None,
            new_status: ride.status,
            triggered_by: Some(ride.passenger_id),
            notes: Some("Solicitud de viaje creada en Santiago".to_string()),
            created_at: Utc::now(),
        });
        Ok(ride)
    }

    fn find_ride_by_id(&self, id: &Uuid) -> Result<Option<Ride>, RepoError> {
        Ok(self.rides.read().get(id).cloned())
    }

    fn find_active_ride_for_user(&self, user_id: &Uuid, role: Role) -> Result<Option<Ride>, RepoError> {
        let rides = self.rides.read();
        let active_statuses = [
            RideStatus::Requested,
            RideStatus::Matching,
            RideStatus::Accepted,
            RideStatus::DriverEnRoute,
            RideStatus::Arrived,
            RideStatus::InProgress,
        ];

        match role {
            Role::Passenger => Ok(rides
                .values()
                .find(|r| r.passenger_id == *user_id && active_statuses.contains(&r.status))
                .cloned()),
            Role::Driver => {
                let dp_id = self
                    .driver_profiles
                    .read()
                    .values()
                    .find(|dp| dp.user_id == *user_id)
                    .map(|dp| dp.id);

                if let Some(dp_id) = dp_id {
                    Ok(rides
                        .values()
                        .find(|r| r.driver_id == Some(dp_id) && active_statuses.contains(&r.status))
                        .cloned())
                } else {
                    Ok(None)
                }
            }
            Role::Admin => Ok(None),
        }
    }

    fn list_rides_for_user(&self, user_id: &Uuid, role: Role) -> Result<Vec<Ride>, RepoError> {
        let rides = self.rides.read();
        let mut list: Vec<Ride> = match role {
            Role::Passenger => rides
                .values()
                .filter(|r| r.passenger_id == *user_id)
                .cloned()
                .collect(),
            Role::Driver => {
                let dp_id = self
                    .driver_profiles
                    .read()
                    .values()
                    .find(|dp| dp.user_id == *user_id)
                    .map(|dp| dp.id);

                if let Some(dp_id) = dp_id {
                    rides
                        .values()
                        .filter(|r| r.driver_id == Some(dp_id))
                        .cloned()
                        .collect()
                } else {
                    Vec::new()
                }
            }
            Role::Admin => rides.values().cloned().collect(),
        };

        list.sort_by(|a, b| b.created_at.cmp(&a.created_at));
        Ok(list)
    }

    fn list_all_rides(&self) -> Result<Vec<Ride>, RepoError> {
        let mut list: Vec<Ride> = self.rides.read().values().cloned().collect();
        list.sort_by(|a, b| b.created_at.cmp(&a.created_at));
        Ok(list)
    }

    fn accept_ride_atomic(
        &self,
        ride_id: &Uuid,
        driver_profile_id: &Uuid,
        vehicle_id: &Uuid,
    ) -> Result<Ride, RepoError> {
        let driver_profile = self
            .driver_profiles
            .read()
            .get(driver_profile_id)
            .cloned()
            .ok_or(RepoError::NotFound)?;

        if driver_profile.status != DriverStatus::Approved {
            return Err(RepoError::DriverNotEligible);
        }

        let vehicle = self
            .vehicles
            .read()
            .get(vehicle_id)
            .cloned()
            .ok_or(RepoError::NotFound)?;

        if vehicle.status != VehicleStatus::Approved || !vehicle.is_active {
            return Err(RepoError::DriverNotEligible);
        }

        let mut rides = self.rides.write();

        let active_statuses = [
            RideStatus::Accepted,
            RideStatus::DriverEnRoute,
            RideStatus::Arrived,
            RideStatus::InProgress,
        ];

        let driver_has_active_ride = rides.values().any(|r| {
            r.driver_id == Some(*driver_profile_id) && active_statuses.contains(&r.status)
        });

        if driver_has_active_ride {
            return Err(RepoError::DriverAlreadyCommitted);
        }

        let ride = rides.get_mut(ride_id).ok_or(RepoError::NotFound)?;

        if ride.status != RideStatus::Matching && ride.status != RideStatus::Requested {
            return Err(RepoError::DoubleAssignmentConflict);
        }

        if ride.driver_id.is_some() {
            return Err(RepoError::DoubleAssignmentConflict);
        }

        let now = Utc::now();
        ride.driver_id = Some(*driver_profile_id);
        ride.vehicle_id = Some(*vehicle_id);
        ride.status = RideStatus::Accepted;
        ride.accepted_at = Some(now);
        ride.updated_at = now;

        let updated_ride = ride.clone();

        self.ride_events.write().push(RideEvent {
            id: Uuid::new_v4(),
            ride_id: *ride_id,
            previous_status: Some(RideStatus::Matching),
            new_status: RideStatus::Accepted,
            triggered_by: Some(driver_profile.user_id),
            notes: Some("Viaje aceptado por conductor eléctrico.".to_string()),
            created_at: now,
        });

        Ok(updated_ride)
    }

    fn update_ride_status(
        &self,
        ride_id: &Uuid,
        new_status: RideStatus,
        actor_id: &Uuid,
        actor_role: Role,
        cancellation_reason: Option<String>,
    ) -> Result<Ride, RepoError> {
        let mut rides = self.rides.write();
        let ride = rides.get_mut(ride_id).ok_or(RepoError::NotFound)?;

        validate_transition(ride.status, new_status, actor_role, *actor_id, ride)
            .map_err(|e| RepoError::ValidationError(e.to_string()))?;

        let previous_status = ride.status;
        let now = Utc::now();
        ride.status = new_status;
        ride.updated_at = now;

        match new_status {
            RideStatus::DriverEnRoute => {}
            RideStatus::Arrived => {
                ride.driver_arrived_at = Some(now);
            }
            RideStatus::InProgress => {
                ride.started_at = Some(now);
            }
            RideStatus::Completed => {
                ride.completed_at = Some(now);
                ride.final_fare_clp = Some(ride.estimated_fare_clp);
            }
            RideStatus::Cancelled => {
                ride.cancelled_at = Some(now);
                ride.cancelled_by = Some(*actor_id);
                ride.cancellation_reason = cancellation_reason.clone();
            }
            _ => {}
        }

        let updated_ride = ride.clone();

        self.ride_events.write().push(RideEvent {
            id: Uuid::new_v4(),
            ride_id: *ride_id,
            previous_status: Some(previous_status),
            new_status,
            triggered_by: Some(*actor_id),
            notes: cancellation_reason,
            created_at: now,
        });

        Ok(updated_ride)
    }

    fn create_ride_rating(&self, rating: RideRating) -> Result<RideRating, RepoError> {
        let mut ratings = self.ratings.write();
        ratings.push(rating.clone());
        Ok(rating)
    }

    fn list_ride_ratings_for_user(&self, user_id: &Uuid) -> Result<Vec<RideRating>, RepoError> {
        Ok(self
            .ratings
            .read()
            .iter()
            .filter(|r| r.rated_user_id == *user_id || r.rater_id == *user_id)
            .cloned()
            .collect())
    }

    fn create_support_ticket(&self, ticket: SupportTicket) -> Result<SupportTicket, RepoError> {
        self.support_tickets.write().insert(ticket.id, ticket.clone());
        Ok(ticket)
    }

    fn list_support_tickets_for_user(&self, user_id: &Uuid) -> Result<Vec<SupportTicket>, RepoError> {
        let mut list: Vec<SupportTicket> = self
            .support_tickets
            .read()
            .values()
            .filter(|t| t.user_id == *user_id)
            .cloned()
            .collect();
        list.sort_by(|a, b| b.created_at.cmp(&a.created_at));
        Ok(list)
    }

    fn list_all_support_tickets(&self) -> Result<Vec<SupportTicket>, RepoError> {
        let mut list: Vec<SupportTicket> = self.support_tickets.read().values().cloned().collect();
        list.sort_by(|a, b| b.created_at.cmp(&a.created_at));
        Ok(list)
    }

    fn resolve_support_ticket(
        &self,
        ticket_id: &Uuid,
        resolution_notes: &str,
        admin_id: &Uuid,
    ) -> Result<SupportTicket, RepoError> {
        let mut tickets = self.support_tickets.write();
        let t = tickets.get_mut(ticket_id).ok_or(RepoError::NotFound)?;
        t.status = TicketStatus::Resolved;
        t.resolution_notes = Some(resolution_notes.to_string());
        t.resolved_by = Some(*admin_id);
        t.resolved_at = Some(Utc::now());
        t.updated_at = Utc::now();
        Ok(t.clone())
    }

    fn create_audit_log(&self, log: AuditLog) -> Result<(), RepoError> {
        self.audit_logs.write().push(log);
        Ok(())
    }

    fn list_audit_logs(&self) -> Result<Vec<AuditLog>, RepoError> {
        let mut list = self.audit_logs.read().clone();
        list.sort_by(|a, b| b.created_at.cmp(&a.created_at));
        Ok(list)
    }
}
