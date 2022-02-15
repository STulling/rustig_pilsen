use std::{sync::{mpsc, Arc}};

use crate::logging::log;

use super::math::calc_rms;
use super::math::calc_fft;
use rustfft::{FftPlanner};

pub fn run(rx: mpsc::Receiver<Arc<Vec<f32>>>) {
    let mut fft = FftPlanner::<f32>::new(); 
    while let Ok(data) = rx.recv() {
        log::debug(format!("FFT: {:?}", calc_fft(&mut fft, &data)));
        log::debug(format!("RMS: {:?}", calc_rms(&data)));
    }
}