
docker run --privileged --rm tonistiigi/binfmt --install all
cross build --release --target aarch64-unknown-linux-gnu
cargo deb --target aarch64-unknown-linux-gnu --no-build --no-strip