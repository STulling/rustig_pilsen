extern crate cpal;
use cpal::traits::{DeviceTrait, HostTrait};

mod info;
mod logging;
mod audio;
use logging::log;
use info::audio_info;
use audio::beep::beep;
use audio::feedback;

fn main() {
    audio_info::print_info();
    let host = cpal::default_host();
    let input_device = host.default_input_device().unwrap();
    let output_device = host.default_output_device().unwrap();
    log::warn(format!("Using Devices: \n  [IN] {}\n  [OUT] {}", input_device.name().unwrap(), output_device.name().unwrap()));
    //log::info("Playing Beep".to_string());
    //beep(&output_device);
    log::info("Playing Feedback".to_string());
    let success = feedback::echo(&input_device, &output_device);
    if success.is_err() {
        log::error(format!("Error: {:?}", success.err().unwrap()));
    }
}
