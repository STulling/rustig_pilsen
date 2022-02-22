use std::{sync::{mpsc, Arc}, process::{Command, Stdio}, io::Write};

use super::math::calc_rms;
use super::math::calc_fft;
use rustfft::{FftPlanner};

pub fn run(rx: mpsc::Receiver<Arc<Vec<f32>>>) -> Result<(), anyhow::Error> {
    let mut fft = FftPlanner::<f32>::new(); 
    let mut child = Command::new("sudo")
        .arg("../Biermuur3/video/video")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()?;

    let child_stdin = child.stdin.as_mut().unwrap();
    let mut fftRes: f32 = 0.0;
    let mut rmsRes: f32 = 0.0;

    while let Ok(data) = rx.recv() {
        fftRes = calc_fft(&mut fft, &data) as f32;
        rmsRes = calc_rms(&data);
        child_stdin.write_all(format!("{}, {};", rmsRes, fftRes).as_bytes())?;
    }

    Ok(())
}