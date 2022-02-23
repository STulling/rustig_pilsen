use std::sync::Arc;

use rustfft::{num_complex::Complex, FftPlanner};

pub fn calc_rms(data: &Arc<Vec<f32>>) -> f32 {
    let mut sum = 0.0;
    for sample in data.iter() {
        sum += sample.powi(2);
    }
    sum.sqrt()
}

pub fn calc_fft(fft: &mut FftPlanner<f32>, data: &Arc<Vec<f32>>) -> Vec<f32> {
    let plan = fft.plan_fft_forward(data.len());
    let mut complex_buffer = data
        .iter()
        .map(|&x| Complex { re: x, im: 0.0 })
        .collect::<Vec<Complex<f32>>>();

    plan.process(&mut complex_buffer);   

    let mut buffer: Vec<f32> = complex_buffer.iter().map(|x| x.norm() as f32).collect();

    // remove mirroring
    buffer = buffer[0..(complex_buffer.len() as f32 * 0.5) as usize].to_vec();

    buffer
}