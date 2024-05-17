use std::ffi::OsStr;
use std::path::PathBuf;
use std::process::{Command, ExitStatus};

const FFMPEG: &str = "ffmpeg";

pub struct Ffmpeg {
    verbosity: String,
}

impl Ffmpeg {
    pub fn new(verbosity: &str) -> Self {
        Ffmpeg {
            verbosity: verbosity.to_string(),
        }
    }

    pub fn extract_frames(&self, input_file: &str, frames_dir: PathBuf) -> Result<ExitStatus, std::io::Error> {
        let output_pattern = format!("{}/frame_%04d.png", frames_dir.display());
        Command::new(FFMPEG)
            .args(&[OsStr::new("-v"), OsStr::new(&self.verbosity), OsStr::new("-i"), OsStr::new(input_file), OsStr::new(&output_pattern)])
            .stdout(std::process::Stdio::null())
            .status()
    }

    pub fn extract_audio(&self, input_file: &str, audio_path: PathBuf) -> Result<ExitStatus, std::io::Error> {
        Command::new(FFMPEG)
            .args(&[OsStr::new("-v"), OsStr::new(&self.verbosity), OsStr::new("-i"), OsStr::new(input_file), OsStr::new("-b:a"), OsStr::new("192K"), OsStr::new("-ac"), OsStr::new("2"), OsStr::new("-ar"), OsStr::new("44.1k"), OsStr::new(&audio_path.display().to_string())])
            .stdout(std::process::Stdio::null())
            .status()
    }

    pub fn reassemble_video(&self, frames_dir: PathBuf, audio_path: PathBuf, output_path: PathBuf, framerate: f32) -> Result<ExitStatus, std::io::Error> {
        let input_pattern = format!("{}/frame_%04d.png", frames_dir.display());
        Command::new(FFMPEG)
            .args(&[OsStr::new("-v"), OsStr::new(&self.verbosity), OsStr::new("-framerate"), OsStr::new(&framerate.to_string()), OsStr::new("-i"), OsStr::new(&input_pattern), OsStr::new("-i"), OsStr::new(&audio_path.display().to_string()), OsStr::new("-c:v"), OsStr::new("libx264"), OsStr::new("-c:a"), OsStr::new("aac"), OsStr::new("-s:v"), OsStr::new("3840x2160"), OsStr::new("-pix_fmt"), OsStr::new("yuv420p"), OsStr::new(&output_path.display().to_string())])
            .stdout(std::process::Stdio::null())
            .status()
    }
}