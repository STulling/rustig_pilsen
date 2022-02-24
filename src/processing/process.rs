use std::{sync::{mpsc, Arc}, process::{Command, Stdio}, io::Write};

use crate::logging::log;

use super::math::calc_rms;
use super::math::calc_fft;
use rustfft::{FftPlanner};

pub fn run(rx: mpsc::Receiver<Arc<Vec<f32>>>) -> Result<(), anyhow::Error> {
    if cfg!(target_os="linux") {
    Command::new("sudo")
        .arg("pkill -f video")
        .spawn()?;
    } else {
        Command::new("taskkill")
            .arg("/IM")
            .arg("video.exe")
            .spawn()?;
    }
    let mut child = if cfg!(target_os = "linux"){
        Command::new("sudo")
        .arg("../Biermuur3/video/video")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()?
    } else {
        Command::new("../Biermuur3/video/video.exe")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()?
    };

    let mut fft = FftPlanner::<f32>::new();
    let child_stdin = child.stdin.as_mut().unwrap();

    while let Ok(data) = rx.recv() {
        let fft_res = calc_fft(&mut fft, &data);
        let rms_res = calc_rms(&data);
        child_stdin.write_all(format!("{}, {};", rms_res, fft_res).as_bytes())?;
    }

    Ok(())
}