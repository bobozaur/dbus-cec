use cec_rs::{CecConnection, CecLogicalAddress};
use tokio::sync::{mpsc, oneshot};
use tracing::instrument;

use crate::{interface::CecError, power_status::CecPowerStatus};

/// A request the `DBus` interface makes to the background task for interacting with the CEC
/// connection.
#[derive(Debug)]
pub enum CecRequest {
    On(oneshot::Sender<Result<(), CecError>>),
    Off(oneshot::Sender<Result<(), CecError>>),
    Status(oneshot::Sender<CecPowerStatus>),
}

/// Background task that awaits requests and issues commands using the CEC connection.
/// Used so as not to block the async `DBus` interface for too long.
#[allow(clippy::needless_pass_by_value)]
#[instrument(skip(conn))]
pub fn background_task(
    conn: CecConnection,
    target: CecLogicalAddress,
    mut req_rx: mpsc::Receiver<CecRequest>,
) {
    const RESP_SENDER_ERR: &str = "BUG: response receiver dropped";

    while let Some(req) = req_rx.blocking_recv() {
        match req {
            CecRequest::On(sender) => {
                tracing::info!("Requesting device power on...");
                let resp = conn.send_power_on_devices(target).map_err(From::from);
                sender.send(resp).expect(RESP_SENDER_ERR);
            }
            CecRequest::Off(sender) => {
                tracing::info!("Requesting device power off...");
                let resp = conn.send_standby_devices(target).map_err(From::from);
                sender.send(resp).expect(RESP_SENDER_ERR);
            }
            CecRequest::Status(sender) => {
                tracing::info!("Requesting device power status...");
                let resp = conn.get_device_power_status(target).into();
                sender.send(resp).expect(RESP_SENDER_ERR);
            }
        }
    }
}
