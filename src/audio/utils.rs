use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use crate::logging::log;

pub fn get_default_device() -> cpal::Device {
    let host = cpal::default_host();
    let device = host.default_output_device().unwrap();
    return device;
}