use chrono::Utc;
use uuid::Uuid;
use zuvio_api::config::AppConfig;
use zuvio_api::domain::ev_rules::{validate_electric_vehicle, EVValidationError};
use zuvio_api::domain::models::*;
use zuvio_api::domain::pricing::calculate_fare;
use zuvio_api::domain::state_machine::{validate_transition, StateMachineError};
use zuvio_api::integrations::{EmailPayload, EmailProvider, MockEmailProvider};
use zuvio_api::repository::{InMemoryRepository, RepoError, Repository};

#[test]
fn test_ev_validation_rules() {
    let valid = validate_electric_vehicle(PropulsionType::BEV, 2023, 60.0, 4, "LW-72-91");
    assert!(valid.is_ok(), "Un vehículo BEV moderno debe ser válido");

    let old_ev = validate_electric_vehicle(PropulsionType::BEV, 2016, 40.0, 4, "GG-12-34");
    assert_eq!(old_ev, Err(EVValidationError::VehicleTooOld(2016)));

    let low_battery = validate_electric_vehicle(PropulsionType::BEV, 2022, 22.0, 4, "LW-72-91");
    assert_eq!(
        low_battery,
        Err(EVValidationError::InsufficientBatteryCapacity(22.0))
    );

    let bad_plate = validate_electric_vehicle(PropulsionType::BEV, 2023, 50.0, 4, "INVALIDO123");
    assert!(bad_plate.is_err());
}

#[test]
fn test_pricing_calculation_santiago_clp() {
    let config = AppConfig::from_env();

    let fare = calculate_fare(5.4, 16, &config);
    assert_eq!(fare.currency, "CLP");
    assert!(fare.total_fare_clp >= config.minimum_fare_clp);
    assert!(fare.co2_saved_kg > 0.0, "Debe registrar ahorro de CO2 en kg");

    let short_fare = calculate_fare(0.8, 3, &config);
    assert_eq!(
        short_fare.total_fare_clp, config.minimum_fare_clp,
        "Viajes muy cortos deben aplicar la tarifa mínima de ${} CLP",
        config.minimum_fare_clp
    );
    assert!(short_fare.is_minimum_applied);
}

#[test]
fn test_state_machine_valid_transitions() {
    let passenger_id = Uuid::new_v4();
    let driver_profile_id = Uuid::new_v4();

    let ride = Ride {
        id: Uuid::new_v4(),
        passenger_id,
        driver_id: Some(driver_profile_id),
        vehicle_id: Some(Uuid::new_v4()),
        status: RideStatus::Requested,
        origin_address: "Av. Providencia 1234".to_string(),
        origin_commune: "Providencia".to_string(),
        origin_lat: -33.4285,
        origin_lng: -70.6189,
        destination_address: "Av. Apoquindo 4500".to_string(),
        destination_commune: "Las Condes".to_string(),
        destination_lat: -33.4150,
        destination_lng: -70.5790,
        distance_km: 5.4,
        duration_minutes: 16,
        estimated_fare_clp: 5200,
        final_fare_clp: None,
        co2_saved_kg: 0.86,
        cancellation_reason: None,
        cancelled_by: None,
        created_at: Utc::now(),
        updated_at: Utc::now(),
        accepted_at: None,
        driver_arrived_at: None,
        started_at: None,
        completed_at: None,
        cancelled_at: None,
    };

    assert!(validate_transition(
        RideStatus::Requested,
        RideStatus::Matching,
        Role::Passenger,
        passenger_id,
        &ride
    )
    .is_ok());

    assert!(validate_transition(
        RideStatus::Matching,
        RideStatus::Accepted,
        Role::Driver,
        driver_profile_id,
        &ride
    )
    .is_ok());

    assert!(validate_transition(
        RideStatus::Accepted,
        RideStatus::DriverEnRoute,
        Role::Driver,
        driver_profile_id,
        &ride
    )
    .is_ok());

    assert!(validate_transition(
        RideStatus::DriverEnRoute,
        RideStatus::Arrived,
        Role::Driver,
        driver_profile_id,
        &ride
    )
    .is_ok());

    assert!(validate_transition(
        RideStatus::Arrived,
        RideStatus::InProgress,
        Role::Driver,
        driver_profile_id,
        &ride
    )
    .is_ok());

    assert!(validate_transition(
        RideStatus::InProgress,
        RideStatus::Completed,
        Role::Driver,
        driver_profile_id,
        &ride
    )
    .is_ok());
}

#[test]
fn test_state_machine_invalid_skips_and_unauthorized() {
    let passenger_id = Uuid::new_v4();
    let other_user_id = Uuid::new_v4();
    let driver_id = Uuid::new_v4();

    let ride = Ride {
        id: Uuid::new_v4(),
        passenger_id,
        driver_id: Some(driver_id),
        vehicle_id: Some(Uuid::new_v4()),
        status: RideStatus::Requested,
        origin_address: "Metro Manuel Montt".to_string(),
        origin_commune: "Providencia".to_string(),
        origin_lat: -33.4285,
        origin_lng: -70.6189,
        destination_address: "Costanera Center".to_string(),
        destination_commune: "Providencia".to_string(),
        destination_lat: -33.4178,
        destination_lng: -70.6065,
        distance_km: 2.5,
        duration_minutes: 8,
        estimated_fare_clp: 3500,
        final_fare_clp: None,
        co2_saved_kg: 0.4,
        cancellation_reason: None,
        cancelled_by: None,
        created_at: Utc::now(),
        updated_at: Utc::now(),
        accepted_at: None,
        driver_arrived_at: None,
        started_at: None,
        completed_at: None,
        cancelled_at: None,
    };

    let invalid_skip = validate_transition(
        RideStatus::Requested,
        RideStatus::Completed,
        Role::Driver,
        driver_id,
        &ride,
    );
    assert_eq!(
        invalid_skip,
        Err(StateMachineError::InvalidTransition {
            from: RideStatus::Requested,
            to: RideStatus::Completed
        })
    );

    let unauthorized_cancel = validate_transition(
        RideStatus::Requested,
        RideStatus::Cancelled,
        Role::Passenger,
        other_user_id,
        &ride,
    );
    assert_eq!(
        unauthorized_cancel,
        Err(StateMachineError::NotRidePassengerOrAdmin)
    );
}

#[test]
fn test_prevent_double_assignment() {
    let repo = InMemoryRepository::new_with_demo_data();

    let driver1_profile_id = Uuid::parse_str("10000000-0000-0000-0000-000000000001").unwrap();
    let vehicle1_id = Uuid::parse_str("20000000-0000-0000-0000-000000000001").unwrap();

    let driver2_profile_id = Uuid::parse_str("10000000-0000-0000-0000-000000000002").unwrap();
    let vehicle2_id = Uuid::parse_str("20000000-0000-0000-0000-000000000002").unwrap();

    let passenger_id = Uuid::parse_str("00000000-0000-0000-0000-000000000002").unwrap();

    let ride = Ride {
        id: Uuid::new_v4(),
        passenger_id,
        driver_id: None,
        vehicle_id: None,
        status: RideStatus::Matching,
        origin_address: "Plaza Baquedano".to_string(),
        origin_commune: "Santiago Centro".to_string(),
        origin_lat: -33.4372,
        origin_lng: -70.6347,
        destination_address: "Parque Araucano".to_string(),
        destination_commune: "Las Condes".to_string(),
        destination_lat: -33.4022,
        destination_lng: -70.5732,
        distance_km: 7.2,
        duration_minutes: 20,
        estimated_fare_clp: 6800,
        final_fare_clp: None,
        co2_saved_kg: 1.15,
        cancellation_reason: None,
        cancelled_by: None,
        created_at: Utc::now(),
        updated_at: Utc::now(),
        accepted_at: None,
        driver_arrived_at: None,
        started_at: None,
        completed_at: None,
        cancelled_at: None,
    };

    let created_ride = repo.create_ride(ride).expect("Debe crear viaje");

    let accept1 = repo.accept_ride_atomic(&created_ride.id, &driver1_profile_id, &vehicle1_id);
    assert!(accept1.is_ok(), "Conductor 1 debe poder aceptar el viaje");
    let accepted_ride = accept1.unwrap();
    assert_eq!(accepted_ride.status, RideStatus::Accepted);
    assert_eq!(accepted_ride.driver_id, Some(driver1_profile_id));

    let accept2 = repo.accept_ride_atomic(&created_ride.id, &driver2_profile_id, &vehicle2_id);
    match accept2 {
        Err(RepoError::DoubleAssignmentConflict) => {}
        other => panic!("Se esperaba DoubleAssignmentConflict, se obtuvo: {:?}", other),
    }

    let ride2 = Ride {
        id: Uuid::new_v4(),
        passenger_id,
        driver_id: None,
        vehicle_id: None,
        status: RideStatus::Matching,
        origin_address: "Tobalaba".to_string(),
        origin_commune: "Providencia".to_string(),
        origin_lat: -33.418,
        origin_lng: -70.601,
        destination_address: "Los Leones".to_string(),
        destination_commune: "Providencia".to_string(),
        destination_lat: -33.422,
        destination_lng: -70.608,
        distance_km: 1.5,
        duration_minutes: 6,
        estimated_fare_clp: 3000,
        final_fare_clp: None,
        co2_saved_kg: 0.24,
        cancellation_reason: None,
        cancelled_by: None,
        created_at: Utc::now(),
        updated_at: Utc::now(),
        accepted_at: None,
        driver_arrived_at: None,
        started_at: None,
        completed_at: None,
        cancelled_at: None,
    };
    let created_ride2 = repo.create_ride(ride2).expect("Debe crear segundo viaje");
    let accept_committed = repo.accept_ride_atomic(&created_ride2.id, &driver1_profile_id, &vehicle1_id);
    match accept_committed {
        Err(RepoError::DriverAlreadyCommitted) => {}
        other => panic!("Se esperaba DriverAlreadyCommitted, se obtuvo: {:?}", other),
    }
}

#[test]
fn test_driver_coverage_radius_and_nearby_drivers() {
    let repo = InMemoryRepository::new_with_demo_data();
    let driver1_id = Uuid::parse_str("10000000-0000-0000-0000-000000000001").unwrap();

    // 1. Update coverage radius to 12.0 km
    let updated = repo.set_driver_coverage_radius(&driver1_id, 12.0).expect("Debe actualizar radio");
    assert_eq!(updated.coverage_radius_km, 12.0);

    // 2. Query nearby drivers near Metro Manuel Montt (-33.4285, -70.6189)
    let nearby = repo.get_nearby_drivers(-33.4285, -70.6189, 15.0).expect("Debe buscar choferes");
    assert!(!nearby.is_empty(), "Debe encontrar al menos 1 chofer eléctrico cercano");
    assert_eq!(nearby[0].driver_id, driver1_id);
    assert_eq!(nearby[0].vehicle_brand, "Tesla");
    assert!(nearby[0].distance_km < 12.0);

    // 3. If driver restricts coverage to very small (e.g. 0.1 km / 1.0 km), they drop out if further
    let updated_small = repo.set_driver_coverage_radius(&driver1_id, 1.0).unwrap();
    assert_eq!(updated_small.coverage_radius_km, 1.0);

    // From Plaza Baquedano (-33.4372, -70.6347), distance to Rodrigo is ~2.3 km > 1.0 km
    let far_query = repo.get_nearby_drivers(-33.4372, -70.6347, 15.0).unwrap();
    let found_driver1 = far_query.iter().any(|d| d.driver_id == driver1_id);
    assert!(!found_driver1, "El chofer no debe ser llamado si el punto de partida excede su radio de cobertura");
}

#[tokio::test]
async fn test_mock_email_provider() {
    let provider = MockEmailProvider::new();
    let payload = EmailPayload {
        to_email: "pasajero@zuvio.cl".to_string(),
        to_name: "Valentina Silva".to_string(),
        subject: "Confirmación de viaje eléctrico Zuvio".to_string(),
        html_content: "<p>Tu Tesla eléctrico está en camino</p>".to_string(),
        text_content: Some("Tu Tesla eléctrico está en camino".to_string()),
    };

    let res = provider.send_email(payload.clone()).await;
    assert!(res.is_ok());

    let sent = provider.sent_emails.read().clone();
    assert_eq!(sent.len(), 1);
    assert_eq!(sent[0].to_email, "pasajero@zuvio.cl");
    assert_eq!(sent[0].subject, "Confirmación de viaje eléctrico Zuvio");
}
