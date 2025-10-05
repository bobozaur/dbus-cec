# Raspberry Pi <-> TV

Raspberry Pi systemd D-Bus service for TV power control through HDMI-CEC.

## Features

- Exposes a D-Bus object (`/com.home.HdmiCec/Tv`) on the library defined D-Bus service which implements the library defined D-Bus interface.
- Runs as a systemd service.

## Installation

The project is packaged as a Debian (`.deb`) package. The package installs the `dbus-cec` binary, the D-Bus configuration file, and a systemd service.

The package depends on `libcec6` and `libp8-platform2`, which should be installed on the system.

## Usage

Once installed and running, you can interact with the service using D-Bus tools like `busctl`.

**Introspect the service:**

```bash
busctl introspect com.home.HdmiCec /com/home/HdmiCec/Tv
```

**Check TV Power Status:**

```bash
busctl call com.home.HdmiCec /com/home/HdmiCec/Tv com.home.HdmiCec PowerStatus
```

**Turn TV On:**

```bash
busctl call com.home.HdmiCec /com/home/HdmiCec/Tv com.home.HdmiCec PowerOn
```

**Turn TV Off:**

```bash
busctl call com.home.HdmiCec /com/home/HdmiCec/Tv com.home.HdmiCec PowerOff
```

## Local Cross-compilation

For local development, you can cross-compile the project for a Raspberry Pi (`aarch64-unknown-linux-gnu`) using `cross` and `cargo-deb`.

First, ensure you have `cross` and `cargo-deb` installed:

```bash
cargo install cross cargo-deb
```

Then, run the following commands to build the release and create the Debian package:

```bash
docker run --privileged --rm tonistiigi/binfmt --install all
cross build --release --target aarch64-unknown-linux-gnu -p rpi
cargo deb --target aarch64-unknown-linux-gnu --no-build -p rpi
```

## License

This project is licensed under the MIT License. See the [LICENSE](../LICENSE) file for details.
