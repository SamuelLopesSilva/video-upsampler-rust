mod config;
mod ffmpeg;
mod image_processing;
mod logger;

use config::Config;
use ffmpeg::Ffmpeg;
use image_processing::ImageProcessor;
use logger::Logger;

fn main() {
    

    let config: Config = Config::new();
    let logger: Logger = Logger::new();
    logger.set_log_level("info");
    let ffmpeg: Ffmpeg = Ffmpeg::new("error");  // or "warning", "info", "debug", etc.
    let image_processor: ImageProcessor = ImageProcessor::new((3840, 2160));

    logger.info("Starting video upscaling...");
    
    logger.info("Extracting frames from input video...");
    ffmpeg.extract_frames(&config.input_file, &config.frames_dir);
    
    logger.info("Extracting audio from input video...");
    ffmpeg.extract_audio(&config.input_file, &config.audio_path);

    logger.info("Upscaling frames...");
    image_processor.upscale_frames(&config.frames_dir);

    logger.info("Reassembling video...");

    ffmpeg.reassemble_video(&config.frames_dir, &config.audio_path, &config.output_path, config.framerate);
    logger.info("Video upscaled successfully!");
}