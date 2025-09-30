use cec_rs::{CecDeviceType, CecDeviceTypeVec, CecLogicalAddress};

#[tokio::main(flavor = "current_thread")]
async fn main() {
    dbus_cec::run(
        "Raspberry Pi".to_owned(),
        CecDeviceTypeVec::new(CecDeviceType::PlaybackDevice),
        CecLogicalAddress::Tv,
    )
    .await;
}
