mod config;
mod ffmpeg;
mod image_processing;
mod logger;

use config::Config;
use ffmpeg::Ffmpeg;
use image_processing::ImageProcessor;
use logger::Logger;
use std::time::Instant;
use std::path::Path;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let config: Config = Config::new();
    let logger: Logger = Logger::new();
    logger.set_log_level("info");
    let ffmpeg: Ffmpeg = Ffmpeg::new("error");  // or "warning", "info", "debug", etc.
    let image_processor: ImageProcessor = ImageProcessor::new(
        config.final_resolution, 
        Path::new(&config.frames_dir), 
        Path::new(&config.upscaled_frames_dir)
    );

    if !Path::new(&config.input_file).exists() {
        return Err(format!("Input file '{}' does not exist", config.input_file).into());
    }

    let start_time = Instant::now();
    logger.info("Starting video upscaling...");

    extract_frames(&ffmpeg, &config, &logger)?;
    extract_audio(&ffmpeg, &config, &logger)?;
    upscale_frames(&image_processor, &logger)?;
    reassemble_video(&ffmpeg, &config, &logger)?;

    logger.info("Video upscaled successfully!");

    let end_time = Instant::now();
    let execution_time = end_time.duration_since(start_time);
    logger.info(&format!("Total execution time: {:.2?}", execution_time));

    Ok(())
}

fn extract_frames(ffmpeg: &Ffmpeg, config: &Config, logger: &Logger) -> Result<(), Box<dyn Error>> {
    let start_time = Instant::now();
    logger.info("Extracting frames from input video...");
    ffmpeg.extract_frames(&config.input_file, config.frames_dir.clone())?;
    let execution_time = Instant::now().duration_since(start_time);
    logger.info(&format!("Extracting frames execution time: {:.2?}", execution_time));
    Ok(())
}

fn extract_audio(ffmpeg: &Ffmpeg, config: &Config, logger: &Logger) -> Result<(), Box<dyn Error>> {
    let start_time = Instant::now();
    logger.info("Extracting audio from input video...");
    ffmpeg.extract_audio(&config.input_file, config.audio_path.clone())?;
    let execution_time = Instant::now().duration_since(start_time);
    logger.info(&format!("Extracting audio execution time: {:.2?}", execution_time));
    Ok(())
}

fn upscale_frames(image_processor: &ImageProcessor, logger: &Logger) -> Result<(), Box<dyn Error>> {
    let start_time = Instant::now();
    logger.info("Upscaling frames...");
    image_processor.upscale_frames()?;
    let execution_time = Instant::now().duration_since(start_time);
    logger.info(&format!("Upscaling frames execution time: {:.2?}", execution_time));
    Ok(())
}

fn reassemble_video(ffmpeg: &Ffmpeg, config: &Config, logger: &Logger) -> Result<(), Box<dyn Error>> {
    let start_time = Instant::now();
    logger.info("Reassembling video...");
    ffmpeg.reassemble_video(
        config.upscaled_frames_dir.clone(), 
        config.audio_path.clone(), 
        config.output_path.clone(), 
        config.framerate
    )?;
    let execution_time = Instant::now().duration_since(start_time);
    logger.info(&format!("Reassembling video execution time: {:.2?}", execution_time));
    Ok(())
}