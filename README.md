# dbus-cec

Library for creating daemons to control HDMI-CEC devices over D-Bus.

The daemon exposes a D-Bus service to control the power state of a connected HDMI-CEC device.

## Features

- Exposes a D-Bus service (`com.home.HdmiCec`) and a D-Bus interface (`com.home.HdmiCec.Power`) for controlling device power.
- The created D-Bus object will follow the naming convention `/com/home/HdmiCec/{CEC-DEVICE}`.
- Provides methods to turn the device on, off, and check the power status.

## Implementations

An implementation for controlling a TV from a Raspberry Pi can be found at [rpi-tv-cec](./rpi-tv-cec).

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
