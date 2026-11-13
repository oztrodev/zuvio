use crate::domain::models::PropulsionType;
use thiserror::Error;

#[derive(Debug, Error, PartialEq)]
pub enum EVValidationError {
    #[error("Zuvio opera exclusivamente con autos 100% eléctricos (BEV). El tipo de propulsión ingresado no es válido.")]
    NotBatteryElectricVehicle,

    #[error("El año del vehículo ({0}) no cumple la antigüedad mínima requerida para Zuvio y la Ley EAT (2018 o posterior).")]
    VehicleTooOld(i32),

    #[error("La capacidad de batería ({0:.1} kWh) es insuficiente para operaciones de movilidad urbana segura en Santiago (mínimo 30 kWh).")]
    InsufficientBatteryCapacity(f64),

    #[error("La patente '{0}' no cumple con el formato oficial chileno (ej. ABCD-12 o AB-12-34).")]
    InvalidChileanLicensePlate(String),

    #[error("La capacidad de pasajeros ({0}) debe ser de al menos 3 asientos.")]
    InsufficientPassengerCapacity(i32),
}

pub fn validate_electric_vehicle(
    propulsion: PropulsionType,
    year: i32,
    battery_kwh: f64,
    passenger_capacity: i32,
    license_plate: &str,
) -> Result<(), EVValidationError> {
    // 1. Strict BEV check
    if propulsion != PropulsionType::BEV {
        return Err(EVValidationError::NotBatteryElectricVehicle);
    }

    // 2. Year check (Ley EAT and modern EV criteria)
    if year < 2018 {
        return Err(EVValidationError::VehicleTooOld(year));
    }

    // 3. Minimum battery capacity for Gran Santiago metropolitan routes
    if battery_kwh < 30.0 {
        return Err(EVValidationError::InsufficientBatteryCapacity(battery_kwh));
    }

    // 4. Passenger capacity
    if passenger_capacity < 3 {
        return Err(EVValidationError::InsufficientPassengerCapacity(passenger_capacity));
    }

    // 5. License plate format (e.g. "LW-72-91" or "LW7291" or "BBCL12")
    let cleaned = license_plate.replace(['-', ' ', '.'], "").to_uppercase();
    if cleaned.len() != 6 {
        return Err(EVValidationError::InvalidChileanLicensePlate(license_plate.to_string()));
    }

    Ok(())
}
