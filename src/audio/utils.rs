use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use crate::logging::log;

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