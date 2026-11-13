use crate::domain::models::{Ride, RideStatus, Role};
use thiserror::Error;
use uuid::Uuid;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum StateMachineError {
    #[error("Transición inválida desde {from:?} hacia {to:?}")]
    InvalidTransition { from: RideStatus, to: RideStatus },

    #[error("Actor con rol {role:?} no autorizado para realizar la transición hacia {to:?}")]
    UnauthorizedActorRole { role: Role, to: RideStatus },

    #[error("Solo el conductor asignado puede actualizar el estado de este viaje")]
    NotAssignedDriver,

    #[error("Solo el pasajero titular o un administrador puede cancelar este viaje")]
    NotRidePassengerOrAdmin,

    #[error("El viaje ya se encuentra finalizado o cancelado")]
    RideAlreadyTerminated,
}

pub fn validate_transition(
    current: RideStatus,
    next: RideStatus,
    actor_role: Role,
    actor_id: Uuid,
    ride: &Ride,
) -> Result<(), StateMachineError> {
    if matches!(current, RideStatus::Completed | RideStatus::Cancelled | RideStatus::Expired) {
        return Err(StateMachineError::RideAlreadyTerminated);
    }

    match (current, next) {
        // Passenger requests -> matching
        (RideStatus::Requested, RideStatus::Matching) => Ok(()),

        // Driver accepts matching ride
        (RideStatus::Matching, RideStatus::Accepted) => {
            if actor_role != Role::Driver && actor_role != Role::Admin {
                return Err(StateMachineError::UnauthorizedActorRole {
                    role: actor_role,
                    to: next,
                });
            }
            Ok(())
        }

        // System expires matching ride
        (RideStatus::Matching, RideStatus::Expired) => Ok(()),

        // Driver moves towards pickup
        (RideStatus::Accepted, RideStatus::DriverEnRoute) => {
            validate_assigned_driver(ride, actor_id, actor_role, next)
        }

        // Driver arrives at pickup
        (RideStatus::DriverEnRoute, RideStatus::Arrived) => {
            validate_assigned_driver(ride, actor_id, actor_role, next)
        }

        // Driver begins trip with passenger
        (RideStatus::Arrived, RideStatus::InProgress) => {
            validate_assigned_driver(ride, actor_id, actor_role, next)
        }

        // Driver finishes trip at destination
        (RideStatus::InProgress, RideStatus::Completed) => {
            validate_assigned_driver(ride, actor_id, actor_role, next)
        }

        // Cancellations
        (
            RideStatus::Requested
            | RideStatus::Matching
            | RideStatus::Accepted
            | RideStatus::DriverEnRoute
            | RideStatus::Arrived,
            RideStatus::Cancelled,
        ) => {
            if actor_role == Role::Admin {
                return Ok(());
            }
            if actor_id == ride.passenger_id {
                return Ok(());
            }
            if let Some(driver_profile_id) = ride.driver_id {
                if actor_id == driver_profile_id || actor_role == Role::Driver {
                    return Ok(());
                }
            }
            Err(StateMachineError::NotRidePassengerOrAdmin)
        }

        _ => Err(StateMachineError::InvalidTransition {
            from: current,
            to: next,
        }),
    }
}

fn validate_assigned_driver(
    ride: &Ride,
    actor_id: Uuid,
    actor_role: Role,
    next: RideStatus,
) -> Result<(), StateMachineError> {
    if actor_role == Role::Admin {
        return Ok(());
    }
    if actor_role != Role::Driver {
        return Err(StateMachineError::UnauthorizedActorRole {
            role: actor_role,
            to: next,
        });
    }
    if let Some(assigned_driver_id) = ride.driver_id {
        if assigned_driver_id != actor_id {
            return Err(StateMachineError::NotAssignedDriver);
        }
    } else {
        return Err(StateMachineError::NotAssignedDriver);
    }
    Ok(())
}

