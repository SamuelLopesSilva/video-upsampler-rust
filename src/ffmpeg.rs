use std::path::Path;
use std::process::Command;

pub struct Ffmpeg {
    verbosity: String,
}

impl Ffmpeg {
    pub fn new(verbosity: &str) -> Self {
        Ffmpeg {
            verbosity: verbosity.to_string(),
        }
    }

    pub fn extract_frames(&self, input_file: &str, frames_dir: &Path) {
        Command::new("ffmpeg")
            .args(&["-v", &self.verbosity, "-i", input_file, &format!("{}/frame_%04d.png", frames_dir.display())])
            .stdout(std::process::Stdio::null())
            .status()
            .expect("Failed to extract frames");
    }

    pub fn extract_audio(&self, input_file: &str, audio_path: &Path) {
        Command::new("ffmpeg")
            .args(&["-v", &self.verbosity, "-i", input_file, "-b:a", "192K", "-ac", "2", "-ar", "44.1k", &audio_path.display().to_string().as_str()])
            .stdout(std::process::Stdio::null())
            .status()
            .expect("Failed to extract audio");
    }

    pub fn reassemble_video(&self, frames_dir: &Path, audio_path: &Path, output_path: &Path, framerate: f32) {
        Command::new("ffmpeg")
            .args(&["-v", &self.verbosity, "-framerate", &framerate.to_string(), "-i", &format!("{}/frame_%04d.png", frames_dir.display()), "-i", &audio_path.display().to_string().as_str(), "-c:v", "libx264", "-c:a", "aac", "-s:v", "3840x2160", "-pix_fmt", "yuv420p", output_path.display().to_string().as_str()])
            .stdout(std::process::Stdio::null())
            .status()
            .expect("Failed to reassemble video");
    }
}
