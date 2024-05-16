use std::path::Path;

pub struct Config {
    pub input_file: String,
    pub base_path: std::path::PathBuf,
    pub framerate: f32,
    pub output_path: std::path::PathBuf,
    pub frames_dir: std::path::PathBuf,
    pub audio_path: std::path::PathBuf,
}

impl Config {
    pub fn new() -> Self {
        let input_file: String = std::env::args().nth(1).expect("Please provide an input file");
        let framerate: f32 = std::env::args().nth(2).expect("Please provide a framerate").parse().expect("Failed to parse framerate");
        let input_path: &Path = Path::new(&input_file);
        let base_path: std::path::PathBuf = input_path.parent().unwrap().join(input_path.file_stem().unwrap().to_str().unwrap());

        let output_path: std::path::PathBuf = if let Some(output_path) = std::env::args().nth(3) {
            Path::new(output_path.as_str()).to_path_buf()
        } else {
            base_path.join(format!("{}_upscaled.mp4", base_path.file_name().unwrap().to_str().unwrap()))
        };

        let frames_dir: std::path::PathBuf = base_path.join(format!("{}_frames", base_path.file_name().unwrap().to_str().unwrap()));
        let audio_path: std::path::PathBuf = base_path.join(format!("{}_audio.aac", base_path.file_name().unwrap().to_str().unwrap()));

        if !frames_dir.exists() {
            std::fs::create_dir_all(&frames_dir).expect("Failed to create frames directory");
        }

        Config {
            input_file,
            base_path,
            framerate,
            output_path,
            frames_dir,
            audio_path,
        }
    }
}