use cpal::traits::{DeviceTrait, HostTrait};

use crate::logging::log;

pub fn print_info() {
    log::info(format!("Supported hosts:  {:?}", cpal::ALL_HOSTS));
    let available_hosts = cpal::available_hosts();
    log::info(format!("Available hosts:  {:?}", available_hosts));
    
    log::info("Enumerating Hosts".to_string());
    for host_id in available_hosts {
        log::warn(format!("{}", host_id.name()));
        let host = cpal::host_from_id(host_id).unwrap();

        let default_in = host.default_input_device().map(|e| e.name().unwrap());
        let default_out = host.default_output_device().map(|e| e.name().unwrap());
        log::debug(format!("  Default Input Device:\n    {:?}", default_in));
        log::debug(format!("  Default Output Device:\n    {:?}", default_out));

        let devices = host.devices().unwrap();
        log::info("  Enumerating Devices: ".to_string());
        for (device_index, device) in devices.enumerate() {
            log::info(format!("  {}. \"{}\"", device_index + 1, device.name().unwrap()));

            // Input configs
            if let Ok(conf) = device.default_input_config() {
                log::debug(format!("    Default input stream config:\n      {:?}", conf));
            }
            let input_configs = match device.supported_input_configs() {
                Ok(f) => f.collect(),
                Err(e) => {
                    log::error(format!("    Error getting supported input configs: {:?}", e));
                    Vec::new()
                }
            };
            if !input_configs.is_empty() {
                log::debug(format!("    All supported input stream configs:"));
                for (config_index, config) in input_configs.into_iter().enumerate() {
                    log::debug(format!(
                        "      {}.{}. {:?}",
                        device_index + 1,
                        config_index + 1,
                        config
                    ));
                }
            }

            // Output configs
            if let Ok(conf) = device.default_output_config() {
                log::debug(format!("    Default output stream config:\n      {:?}", conf));
            }
            let output_configs = match device.supported_output_configs() {
                Ok(f) => f.collect(),
                Err(e) => {
                    log::error(format!("    Error getting supported output configs: {:?}", e));
                    Vec::new()
                }
            };
            if !output_configs.is_empty() {
                log::debug(format!("    All supported output stream configs:"));
                for (config_index, config) in output_configs.into_iter().enumerate() {
                    log::debug(format!(
                        "      {}.{}. {:?}",
                        device_index + 1,
                        config_index + 1,
                        config
                    ));
                }
            }
        }
    }
}