use core::str;
use std::process::{Command, Stdio};

pub fn connect_to_device() {
    let bluetooth_devices = Command::new("bluetoothctl")
        .arg("devices")
        .stdout(Stdio::piped())
        .spawn()
        .expect("Could not retrieve bluetooth devices");

    let devices_output = bluetooth_devices.stdout.expect("Could not read output");

    let selected = Command::new("fzf")
        .stdin(Stdio::from(devices_output))
        .stdout(Stdio::piped())
        .spawn()
        .expect("Unable to start fzf");

    let output = selected
        .wait_with_output()
        .expect("No response received from fzf");

    println!("Got output {}", str::from_utf8(&output.stdout).unwrap());
}

pub fn disconnect_to_device() {
    let bluetooth_devices = Command::new("bluetoothctl")
        .arg("devices")
        .arg("Connected")
        .stdout(Stdio::piped())
        .spawn()
        .expect("Could not retrieve connected bluetooth devices");

    let devices_output = bluetooth_devices.stdout.expect("Could not read output");

    let selected = Command::new("fzf")
        .stdin(Stdio::from(devices_output))
        .stdout(Stdio::piped())
        .spawn()
        .expect("Unable to start fzf");

    let output = selected
        .wait_with_output()
        .expect("No response received from fzf");

    println!("Got output {}", str::from_utf8(&output.stdout).unwrap());
}
