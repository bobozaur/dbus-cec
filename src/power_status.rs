use serde::{Deserialize, Serialize};
use zbus::zvariant::Type;

/// Power status of the HDMI-CEC device analog to [`cec_rs::CecPowerStatus`].
/// Used to represent the power status serialized as a string.
#[derive(Debug, Deserialize, Serialize, Type)]
#[zvariant(signature = "s")]
pub enum CecPowerStatus {
    On,
    Standby,
    InTransitionStandbyToOn,
    InTransitionOnToStandby,
    Unknown,
}

impl From<cec_rs::CecPowerStatus> for CecPowerStatus {
    fn from(value: cec_rs::CecPowerStatus) -> Self {
        match value {
            cec_rs::CecPowerStatus::On => CecPowerStatus::On,
            cec_rs::CecPowerStatus::Standby => CecPowerStatus::Standby,
            cec_rs::CecPowerStatus::InTransitionStandbyToOn => {
                CecPowerStatus::InTransitionStandbyToOn
            }
            cec_rs::CecPowerStatus::InTransitionOnToStandby => {
                CecPowerStatus::InTransitionOnToStandby
            }
            cec_rs::CecPowerStatus::Unknown => CecPowerStatus::Unknown,
        }
    }
}
