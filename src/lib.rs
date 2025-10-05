mod interface;
mod power_status;
mod request;

use std::error::Error;

use cec_rs::CecConnectionCfgBuilder;
pub use cec_rs::{CecDeviceType, CecDeviceTypeVec, CecLogicalAddress};
use tokio::{sync::mpsc, task};
use tracing::instrument;
use zbus::conn::Builder;

use crate::interface::{CecIface, SERVICE_NAME};

/// Runs the daemon by:
/// - starting a CEC connection and registering a background task for communication
/// - starting an async `DBus` connection to the system bus and registering the service
/// - pipes calls of `DBus` methods to the background tasks so they are acted upon
///
/// # Errors
/// Will error out if the CEC or `DBus` connections could not be established.
#[instrument]
pub async fn run(
    device_name: String,
    device_types: CecDeviceTypeVec,
    target: CecLogicalAddress,
) -> Result<(), Box<dyn Error>> {
    tracing::info!("Opening CEC connection...");
    let cec_connection = CecConnectionCfgBuilder::default()
        .device_name(device_name)
        .device_types(device_types)
        .build()?
        .open()
        .map_err(|e| format!("{e:?}"))?;

    // Setup bounded channel between DBus interface and background task.
    // We don't want buffering so a size of 1 is fine.
    let (tx, rx) = mpsc::channel(1);

    tracing::info!("Starting background task...");
    let task_handle =
        task::spawn_blocking(move || request::background_task(cec_connection, target, rx));

    // DBus object name
    let object_name = format!("/{}/{target:?}", SERVICE_NAME.replace('.', "/"));

    // Can't see this documented anywhere but I assume
    // that dropping the DBus connection would be bad?
    tracing::info!("Opening DBus connection to serve object {object_name}...");
    let _dbus_connection = Builder::system()?
        .name(SERVICE_NAME)?
        .serve_at(object_name, CecIface(tx))?
        .build()
        .await?;

    // Await background task since [`zbus`] works in the background.
    tracing::info!("DBus interface ready!");
    task_handle.await?;

    Ok(())
}
