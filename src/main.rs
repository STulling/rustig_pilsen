extern crate cpal;

use colored::*;

enum LogLevel {
    DEBUG,
    INFO,
    WARN,
    ERROR,
}

fn info(msg: String) {
    log(LogLevel::INFO, msg);
}

fn debug(msg: String) {
    log(LogLevel::DEBUG, msg);
}

fn warn(msg: String) {
    log(LogLevel::WARN, msg);
}

fn error(msg: String) {
    log(LogLevel::ERROR, msg);
}

fn log(level: LogLevel, msg: String) {
    let log = match level {
        LogLevel::DEBUG => format!("[DEBUG] {}", msg).truecolor(100, 100, 100),
        LogLevel::INFO => format!("[INFO] {}", msg).white(),
        LogLevel::WARN => format!("[WARN] {}", msg).yellow(),
        LogLevel::ERROR => format!("[ERROR] {}", msg).red(),
    };
    println!("{}", log);
}

use cpal::traits::{DeviceTrait, HostTrait};

fn main() -> Result<(), anyhow::Error> {
    env_logger::init();
    info(format!("Supported hosts:  {:?}", cpal::ALL_HOSTS));
    let available_hosts = cpal::available_hosts();
    info(format!("Available hosts:  {:?}", available_hosts));
    
    info("Enumerating Hosts".to_string());
    for host_id in available_hosts {
        warn(format!("{}", host_id.name()));
        let host = cpal::host_from_id(host_id)?;

        let default_in = host.default_input_device().map(|e| e.name().unwrap());
        let default_out = host.default_output_device().map(|e| e.name().unwrap());
        debug(format!("  Default Input Device:\n    {:?}", default_in));
        debug(format!("  Default Output Device:\n    {:?}", default_out));

        let devices = host.devices()?;
        info("  Enumerating Devices: ".to_string());
        for (device_index, device) in devices.enumerate() {
            info(format!("  {}. \"{}\"", device_index + 1, device.name()?));

            // Input configs
            if let Ok(conf) = device.default_input_config() {
                debug(format!("    Default input stream config:\n      {:?}", conf));
            }
            let input_configs = match device.supported_input_configs() {
                Ok(f) => f.collect(),
                Err(e) => {
                    error(format!("    Error getting supported input configs: {:?}", e));
                    Vec::new()
                }
            };
            if !input_configs.is_empty() {
                debug(format!("    All supported input stream configs:"));
                for (config_index, config) in input_configs.into_iter().enumerate() {
                    debug(format!(
                        "      {}.{}. {:?}",
                        device_index + 1,
                        config_index + 1,
                        config
                    ));
                }
            }

            // Output configs
            if let Ok(conf) = device.default_output_config() {
                debug(format!("    Default output stream config:\n      {:?}", conf));
            }
            let output_configs = match device.supported_output_configs() {
                Ok(f) => f.collect(),
                Err(e) => {
                    error(format!("    Error getting supported output configs: {:?}", e));
                    Vec::new()
                }
            };
            if !output_configs.is_empty() {
                debug(format!("    All supported output stream configs:"));
                for (config_index, config) in output_configs.into_iter().enumerate() {
                    debug(format!(
                        "      {}.{}. {:?}",
                        device_index + 1,
                        config_index + 1,
                        config
                    ));
                }
            }
        }
    }

    Ok(())
}
