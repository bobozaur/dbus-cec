use std::error::Error;

use cec_rs::{CecDeviceType, CecDeviceTypeVec, CecLogicalAddress};
use tracing::level_filters::LevelFilter;
use tracing_subscriber::EnvFilter;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn Error>> {
    let filter = EnvFilter::builder()
        .with_default_directive(LevelFilter::INFO.into())
        .from_env_lossy();

    tracing_subscriber::fmt().with_env_filter(filter).init();

    dbus_cec::run(
        "Raspberry Pi".to_owned(),
        CecDeviceTypeVec::new(CecDeviceType::PlaybackDevice),
        CecLogicalAddress::Tv,
    )
    .await
}
