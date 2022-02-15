extern crate cpal;
extern crate lazy_static;

use std::{sync::{mpsc, Arc}, thread};

use cpal::traits::{DeviceTrait, HostTrait};

mod info;
mod logging;
mod processing;
use processing::process;
mod audio;
use logging::log;
use info::audio_info;
use audio::{feedback, utils};

const BLOCK_SIZE: u32 = 1024;

// This function only gets compiled if the target OS is linux
#[cfg(target_os = "linux")]
fn init_devices(_host: &cpal::Host) -> (cpal::Device, cpal::Device) {
    return (utils::get_device("hw:CARD=Device,DEV=0"), utils::get_device("plughw:CARD=Device,DEV=0"));
}

// And this function only gets compiled if the target OS is *not* linux
#[cfg(target_os = "windows")]
fn init_devices(host: &cpal::Host) -> (cpal::Device, cpal::Device){
    let input_device = host.default_input_device().unwrap();
    let output_device = host.default_output_device().unwrap();
    return (input_device, output_device);
}

fn create_pipes<'a, T>(input_device: cpal::Device, output_device: cpal::Device) 
where T: cpal::Sample + Send + 'static + std::marker::Sync {
    let (tx, rx) = mpsc::channel::<Arc<Vec<f32>>>();
    log::info("Playing Feedback".to_string());
    thread::spawn(move || {
        let success = feedback::run::<T>(&input_device, &output_device, 100.0, tx);
        log::error("Stopped playing Feedback".to_string());
        if success.is_err() {
            log::error(format!("Error: {:?}", success.err().unwrap()));
        }
    });
    process::run(rx);

}

fn main() {
    audio_info::print_info();
    let host = cpal::default_host();
    let (input_device, output_device) = init_devices(&host);
    log::warn(format!("Using Devices: \n  [IN] {}\n    {:?}\n  [OUT] {}\n    {:?}", 
        input_device.name().unwrap(), 
        input_device.default_input_config().unwrap(),
        output_device.name().unwrap(),
        output_device.default_output_config().unwrap()));

    let format = utils::get_format(&input_device).unwrap();
    log::warn(format!("Using Format: {:?}", format));

    match format {
        cpal::SampleFormat::F32 => create_pipes::<f32>(input_device, output_device),
        cpal::SampleFormat::I16 => create_pipes::<i16>(input_device, output_device),
        cpal::SampleFormat::U16 => create_pipes::<u16>(input_device, output_device),
    };
}
