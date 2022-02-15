use cpal::traits::{DeviceTrait, HostTrait};

use crate::logging::log;

#[cfg(target_os = "linux")]
pub fn get_device(name: &str) -> cpal::Device {
    let host = cpal::default_host();
    let mut devices = host.devices().unwrap();
    log::debug("Available Devices:".to_string());
    for device in devices {
        log::debug(format!("  {:?}", device.name().unwrap()));
    }
    devices = host.devices().unwrap();
    for device in devices {
        if device.name().unwrap() == name {
            return device;
        }
    }
    log::error(format!("Could not find device: {}", name));
    let device = host.default_output_device().unwrap();
    return device;
}

pub fn get_format(device: &cpal::Device) -> Result<cpal::SampleFormat, anyhow::Error> {
    let format = device.default_input_config()?.sample_format();
    return Ok(format);
}