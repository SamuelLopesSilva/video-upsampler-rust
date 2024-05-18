use image::{ImageBuffer, Rgb};
use indicatif::{ProgressBar, ProgressStyle};
use rayon::prelude::*;
use std::fs;
use std::io::{self};
use std::path::Path;

pub struct ImageProcessor<'a> {
    final_resolution: (u32, u32),
    frames_dir: &'a Path,
    upscaled_frames_dir: &'a Path,
    thread_pool: rayon::ThreadPool,
}

impl<'a> ImageProcessor<'a> {
    pub fn new(
        final_resolution: (u32, u32),
        frames_dir: &'a Path,
        upscaled_frames_dir: &'a Path,
    ) -> Self {
        let thread_pool = rayon::ThreadPoolBuilder::new()
            // .num_threads(4)
            .build()
            .unwrap();
        ImageProcessor {
            final_resolution,
            frames_dir,
            upscaled_frames_dir,
            thread_pool,
        }
    }

    pub fn upscale_frames(&self) -> Result<(), Box<dyn std::error::Error>> {
        let entries = self.read_directory()?;
        let total_files = entries.len();
        let pb = ProgressBar::new(total_files as u64);
        pb.set_style(
            ProgressStyle::default_bar()
                .template(
                    "[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg} ETA: {eta}",
                )
                .expect("Failed to set template")
                .progress_chars("##-"),
        );

        let errors: Vec<String> = self.thread_pool.install(|| {
            entries
                .par_iter()
                .filter_map(|entry| self.upscale_and_save_image(entry, &pb))
                .collect::<Vec<_>>()
        });

        if !errors.is_empty() {
            let error_message = errors.join("\n");
            Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                error_message,
            )))
        } else {
            pb.finish();
            Ok(())
        }
    }

    fn read_directory(&self) -> Result<Vec<fs::DirEntry>, io::Error> {
        fs::read_dir(self.frames_dir)?.collect()
    }

    fn upscale_and_save_image(&self, entry: &fs::DirEntry, pb: &ProgressBar) -> Option<String> {
        let frame_path = entry.path();
        if frame_path.is_file() && frame_path.extension().unwrap_or_default() == "png" {
            if let Ok(image) = self.load_image(&frame_path) {
                let upscaled_image = self.bilinear_interpolation(&image);
                let upscaled_path = self
                    .upscaled_frames_dir
                    .join(frame_path.file_name().unwrap());
                if upscaled_image.save(upscaled_path.clone()).is_err() {
                    Some(format!(
                        "Failed to save upscaled frame: {}",
                        upscaled_path.display()
                    ))
                } else {
                    pb.inc(1);
                    None
                }
            } else {
                Some(format!("Failed to load image: {}", frame_path.display()))
            }
        } else {
            None
        }
    }

    fn load_image(&self, path: &Path) -> Result<ImageBuffer<Rgb<u8>, Vec<u8>>, image::ImageError> {
        let file = fs::File::open(path)?;
        let reader = std::io::BufReader::new(file);
        let dynamic_image = image::load(reader, image::ImageFormat::Png)?;
        let image = dynamic_image.to_rgb8();
        Ok(image)
    }

    fn bilinear_interpolation(
        &self,
        image: &ImageBuffer<Rgb<u8>, Vec<u8>>,
    ) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
        let (width, height) = image.dimensions();
        let (new_width, new_height) = self.final_resolution;
        let mut new_image: ImageBuffer<Rgb<u8>, _> = ImageBuffer::new(new_width, new_height);

        let width_ratio = width as f32 / new_width as f32;
        let height_ratio = height as f32 / new_height as f32;

        for x in 0..new_width {
            for y in 0..new_height {
                let gx = x as f32 * width_ratio;
                let gy = y as f32 * height_ratio;

                let gxi = gx.floor() as u32;
                let gyi = gy.floor() as u32;

                let gxi1 = (gxi + 1).min(width - 1);
                let gyi1 = (gyi + 1).min(height - 1);

                let c00 = image.get_pixel(gxi, gyi);
                let c10 = image.get_pixel(gxi1, gyi);
                let c01 = image.get_pixel(gxi, gyi1);
                let c11 = image.get_pixel(gxi1, gyi1);

                let wx = gx - gxi as f32;
                let wy = gy - gyi as f32;

                let r = (c00.0[0] as f32 * (1.0 - wx - wy + wx * wy)
                    + c10.0[0] as f32 * (wx - wx * wy)
                    + c01.0[0] as f32 * (wy - wx * wy)
                    + c11.0[0] as f32 * wx * wy) as u8;
                let g = (c00.0[1] as f32 * (1.0 - wx - wy + wx * wy)
                    + c10.0[1] as f32 * (wx - wx * wy)
                    + c01.0[1] as f32 * (wy - wx * wy)
                    + c11.0[1] as f32 * wx * wy) as u8;
                let b = (c00.0[2] as f32 * (1.0 - wx - wy + wx * wy)
                    + c10.0[2] as f32 * (wx - wx * wy)
                    + c01.0[2] as f32 * (wy - wx * wy)
                    + c11.0[2] as f32 * wx * wy) as u8;

                *new_image.get_pixel_mut(x, y) = Rgb([r, g, b]);
            }
        }

        new_image
    }
}
