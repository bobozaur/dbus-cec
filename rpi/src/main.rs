use std::error::Error;

use dbus_cec::{CecDeviceType, CecDeviceTypeVec, CecLogicalAddress};
use tracing_subscriber::EnvFilter;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn Error>> {
    let filter = EnvFilter::builder()
        .with_default_directive("info".parse()?)
        .from_env_lossy();

    tracing_subscriber::fmt().with_env_filter(filter).init();

    dbus_cec::run(
        "Raspberry Pi".to_owned(),
        CecDeviceTypeVec::new(CecDeviceType::PlaybackDevice),
        CecLogicalAddress::Tv,
    )
    .await
}
