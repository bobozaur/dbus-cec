//! Module containing the D-Bus interface and D-Bus related code,
//! arranged for visibility and locality.

use std::borrow::Cow;

use cec_rs::CecConnectionResultError;
use tokio::sync::{mpsc, oneshot};
use tracing::instrument;
use zbus::{DBusError, interface};

use crate::{power_status::CecPowerStatus, request::CecRequest};

/// D-Bus interface that sends requrests to the background task and awaits their response.
#[derive(Debug)]
pub struct CecIface(pub mpsc::Sender<CecRequest>);

pub const SERVICE_NAME: &str = "com.home.HdmiCec";

/// Error returned by the D-Bus interface [`CecIface`].
#[derive(DBusError, Debug)]
#[zbus(prefix = "com.home.HdmiCec")]
pub enum CecError {
    /// Error returned by the `Power` interface.
    // [`zbus`] docs only mentions [`String`], but anything
    // that implements [`std::borrow::Borrow<str>`] does the trick :).
    Power(Cow<'static, str>),
}

#[interface(name = "com.home.HdmiCec.Power")]
impl CecIface {
    const REQ_SENDER_ERR: &str = "BUG: request received dropped";
    const RESP_RECEIVER_ERR: &str = "BUG: response sender dropped";

    /// Sends a power on request to the CEC device and awaits its response.
    #[instrument(skip(self), err, ret)]
    async fn power_on(&self) -> Result<(), CecError> {
        tracing::debug!("D-Bus interface PowerOn called");
        let (tx, rx) = oneshot::channel();
        self.0
            .send(CecRequest::On(tx))
            .await
            .expect(Self::REQ_SENDER_ERR);
        rx.await.expect(Self::RESP_RECEIVER_ERR)
    }

    /// Sends a power off request to the CEC device and awaits its response.
    #[instrument(skip(self), err, ret)]
    async fn power_off(&self) -> Result<(), CecError> {
        tracing::debug!("D-Bus interface PowerOff called");
        let (tx, rx) = oneshot::channel();
        self.0
            .send(CecRequest::Off(tx))
            .await
            .expect(Self::REQ_SENDER_ERR);
        rx.await.expect(Self::RESP_RECEIVER_ERR)
    }

    /// Sends a power status request to the CEC device and awaits its response.
    #[instrument(skip(self), ret)]
    async fn power_status(&self) -> CecPowerStatus {
        tracing::debug!("D-Bus interface PoweStatus called");
        let (tx, rx) = oneshot::channel();
        self.0
            .send(CecRequest::Status(tx))
            .await
            .expect(Self::REQ_SENDER_ERR);
        rx.await.expect(Self::RESP_RECEIVER_ERR)
    }
}

impl From<CecConnectionResultError> for CecError {
    fn from(_: CecConnectionResultError) -> Self {
        Self::Power(Cow::Borrowed("CEC communication error"))
    }
}
