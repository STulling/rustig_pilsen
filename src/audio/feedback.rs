use cpal::traits::{DeviceTrait, StreamTrait};
use crate::logging::log;
extern crate ringbuf;
use ringbuf::RingBuffer;

pub fn echo(input_device: &cpal::Device, output_device: &cpal::Device) -> Result<(), anyhow::Error> {
    let latency = 50.0;
    // We'll try and use the same configuration between streams to keep it simple.
    let config: cpal::StreamConfig = input_device.default_input_config()?.into();

    // Create a delay in case the input and output devices aren't synced.
    let latency_frames = (latency / 1_000.0) * config.sample_rate.0 as f32;
    let latency_samples = latency_frames as usize * config.channels as usize;

    // The buffer to share samples
    let ring = RingBuffer::<i16>::new(latency_samples * 2);
    let (mut producer, mut consumer) = ring.split();

    // Fill the samples with 0.0 equal to the length of the delay.
    for _ in 0..latency_samples {
        // The ring buffer has twice as much space as necessary to add latency here,
        // so this should never fail
        producer.push(0).unwrap();
    }

    let input_data_fn = move |data: &[i16], _: &cpal::InputCallbackInfo| {
        let mut output_fell_behind = false;
        for &sample in data {
            if producer.push(sample).is_err() {
                output_fell_behind = true;
            }
        }
        if output_fell_behind {
            log::error("output stream fell behind: try increasing latency".to_string());
        }
    };

    let output_data_fn = move |data: &mut [i16], _: &cpal::OutputCallbackInfo| {
        let mut input_fell_behind = false;
        for sample in data {
            *sample = match consumer.pop() {
                Some(s) => s,
                None => {
                    input_fell_behind = true;
                    0
                }
            };
        }
        if input_fell_behind {
            log::error("input stream fell behind: try increasing latency".to_string());
        }
    };

    // Build streams.
    log::debug(format!(
        "Attempting to build both streams with i16 samples and `{:?}`.",
        config
    ));
    let input_stream = input_device.build_input_stream(&config, input_data_fn, err_fn)?;
    let output_stream = output_device.build_output_stream(&config, output_data_fn, err_fn)?;
    log::debug("Successfully built streams.".to_string());

    // Play the streams.
    log::debug(format!(
        "Starting the input and output streams with `{}` milliseconds of latency.",
        latency
    ));
    input_stream.play()?;
    output_stream.play()?;

    // Run for 3 seconds before closing.
    log::debug("Playing for 10 seconds... ".to_string());
    std::thread::sleep(std::time::Duration::from_secs(10));
    drop(input_stream);
    drop(output_stream);
    log::warn("Done!".to_string());
    Ok(())
}

fn err_fn(err: cpal::StreamError) {
    log::error(format!("an error occurred on stream: {}", err));
}
