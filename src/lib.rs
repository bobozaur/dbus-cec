use std::sync::Arc;

use cec_rs::{CecConnection, CecConnectionCfgBuilder, CecDeviceTypeVec, CecLogicalAddress};
use serde::{Deserialize, Serialize};
use zbus::conn::Builder;
use zbus::interface;
use zbus::zvariant::Type;

pub async fn run(
    device_name: String,
    device_types: CecDeviceTypeVec,
    target_device: CecLogicalAddress,
) {
    let cec_connection = CecConnectionCfgBuilder::default()
        .device_name(device_name)
        .device_types(device_types)
        .build()
        .unwrap()
        .open()
        .unwrap();

    let cec_power_controller = CecPowerController {
        conn: Arc::new(cec_connection),
        target: target_device,
    };

    let _connection = Builder::system()
        .unwrap()
        .name(SERVICE_NAME)
        .unwrap()
        .serve_at(OBJECT_NAME, cec_power_controller)
        .unwrap()
        .build()
        .await
        .unwrap();

    loop {
        std::future::pending::<()>().await;
    }
}

#[derive(Debug, Deserialize, Serialize, Type)]
#[zvariant(signature = "s")]
enum CecPowerStatus {
    On,
    Standby,
    Unknown,
}

struct CecPowerController {
    conn: Arc<CecConnection>,
    target: CecLogicalAddress,
}

const SERVICE_NAME: &str = "com.home.HdmiCec";
const OBJECT_NAME: &str = "/com/home/HdmiCec/Tv";

#[interface(name = "com.home.HdmiCec.Power", spawn = false)]
impl CecPowerController {
    async fn power_on(&self) -> zbus::fdo::Result<()> {
        let connection = self.conn.clone();
        let device = self.target;

        tokio::task::spawn_blocking(move || connection.send_power_on_devices(device).unwrap())
            .await
            .unwrap();

        Ok(())
    }

    async fn power_off(&self) -> zbus::fdo::Result<()> {
        let connection = self.conn.clone();
        let device = self.target;

        tokio::task::spawn_blocking(move || {
            connection.send_standby_devices(device).unwrap();
        })
        .await
        .unwrap();

        Ok(())
    }

    async fn power_status(&self) -> CecPowerStatus {
        let connection = self.conn.clone();
        let device = self.target;

        tokio::task::spawn_blocking(move || match connection.get_device_power_status(device) {
            cec_rs::CecPowerStatus::On => CecPowerStatus::On,
            cec_rs::CecPowerStatus::Standby => CecPowerStatus::Standby,
            cec_rs::CecPowerStatus::InTransitionStandbyToOn
            | cec_rs::CecPowerStatus::InTransitionOnToStandby
            | cec_rs::CecPowerStatus::Unknown => CecPowerStatus::Unknown,
        })
        .await
        .unwrap()
    }
}
