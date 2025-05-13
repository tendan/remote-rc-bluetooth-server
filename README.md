# Rover remotely controlled via Bluetooth LE on Linux device

This project includes implementation for remote control via Bluetooth LE written in Rust.

Bluetooth LE byte signals send via characteristic are able to:
- move forward
- stop the rover
- rotate wheels

It uses [bluer](https://github.com/bluez/bluer) as a Bluetooth API and [rppal](https://github.com/golemparts/rppal) for GPIO peripheral communication.

Raspberry Pi 4B is used as a tested device.

It's cargo build and run application. It requires dbus and bluez5 to be present on Linux machine.