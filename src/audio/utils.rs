use cpal::traits::{DeviceTrait, HostTrait};

#[cfg(target_os = "linux")]
pub fn get_device(name: &str) -> cpal::Device {
    let host = cpal::default_host();
    let devices = host.devices().unwrap();
    for device in devices {
        if device.name().unwrap() == name {
            return device;
        }
    }
    let device = host.default_input_device().unwrap();
    return device;
}

pub fn get_format(device: &cpal::Device) -> Result<cpal::SampleFormat, anyhow::Error> {
    let format = device.default_input_config()?.sample_format();
    return Ok(format);
}