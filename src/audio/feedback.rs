use std::sync::{mpsc, Arc};

use cpal::{traits::{DeviceTrait, StreamTrait}, BufferSize};
use crate::{logging::log, BLOCK_SIZE};
extern crate ringbuf;
use ringbuf::{RingBuffer, Producer};

pub fn run<T>(input_device: &cpal::Device, output_device: &cpal::Device, latency: f32, tx: mpsc::Sender<Arc<Vec<f32>>>) -> Result<(), anyhow::Error>
where
    T: cpal::Sample + Send + 'static + std::marker::Sync,
{   
    let mut config: cpal::StreamConfig = input_device.default_input_config()?.into();
    config.buffer_size = BufferSize::Fixed(BLOCK_SIZE * config.channels as u32 * std::mem::size_of::<T>() as u32);
    let latency_frames = (latency / 1_000.0) * config.sample_rate.0 as f32;
    let latency_samples = latency_frames as usize * config.channels as usize;
    let ring = RingBuffer::<T>::new(latency_samples * 2);
    let (mut producer, mut consumer) = ring.split();

    let zerofloat = 0.0 as f32;
    let zero = T::from::<f32>(&zerofloat);

    // Fill the samples with 0.0 equal to the length of the delay.
    for _ in 0..latency_samples {
        // The ring buffer has twice as much space as necessary to add latency here,
        // so this should never fail
        let _ = producer.push(zero);
    }

    let output_data_fn = move |data: &mut [T], _: &cpal::OutputCallbackInfo| {
        let mut input_fell_behind = false;
        for sample in data {
            *sample = match consumer.pop() {
                Some(s) => s,
                None => {
                    input_fell_behind = true;
                    zero
                }
            };
        }
        if input_fell_behind {
            log::error("input stream fell behind: try increasing latency".to_string());
        }
    };

    // Build streams.
    log::debug(format!(
        "Attempting to build both streams with {:?} samples and `{:?}`.",
        input_device.default_input_config()?.sample_format(),
        config
    ));
    let input_stream = input_device.build_input_stream(&config, move |data, _: &_| handle_input_data(data, tx.clone(), &mut producer), err_fn)?;
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

fn handle_input_data<T>(data: &[T], sender: mpsc::Sender<Arc<Vec<f32>>>, producer: &mut Producer<T>) where T : cpal::Sample {
    let mut output_fell_behind = false;
        // convert to f32
        let mut data_f32 = Vec::new();
        for sample in data.iter() {
            data_f32.push(sample.to_f32());
        }
        sender.send(Arc::new(data_f32)).unwrap();

        for &sample in data {
            if producer.push(sample).is_err() {
                output_fell_behind = true;
            }
        }
        if output_fell_behind {
            log::error("output stream fell behind: try increasing latency".to_string());
        }
}

fn err_fn(err: cpal::StreamError) {
    log::error(format!("Error: {}", err));
}
