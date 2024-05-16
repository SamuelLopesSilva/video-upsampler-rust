use std::path::Path;
use image::{DynamicImage, GenericImage, GenericImageView, Rgba};

pub struct ImageProcessor {
    final_resolution: (u32, u32),
}

impl ImageProcessor {
    pub fn new(final_resolution: (u32, u32)) -> Self {
        ImageProcessor { final_resolution }
    }

    pub fn upscale_frames(&self, frames_dir: &Path) {
        for i in 0.. {
            let frame_path: std::path::PathBuf = frames_dir.join(format!("frame_{:04}.png", i));
            if let Ok(image) = image::open(&frame_path) {
                let upscaled_image: DynamicImage = self.bilinear_interpolation(&image);
                upscaled_image.save(&frame_path).expect("Failed to save upscaled frame");
            } else {
                break;
            }
        }
    }

    /// Performs bilinear interpolation on an image to resize it.
    ///
    /// # Arguments
    ///
    /// * `image` - A DynamicImage that holds the image to be resized.
    ///
    /// # Returns
    ///
    /// * A new DynamicImage that is the resized version of the input image.
    ///
    /// # Bilinear Interpolation
    ///
    /// Bilinear interpolation is a technique for calculating values between two points in a grid.
    /// It's used here to calculate the color value of a pixel in the resized image based on the corresponding position in the original image.
    ///
    /// The calculation involves the following steps:
    ///
    /// 1. Calculate the position in the original image that corresponds to a pixel in the resized image.
    /// 2. Get the colors of the four pixels in the original image that surround this position.
    /// 3. Calculate the weights for these four colors based on the distance from the position to each pixel.
    /// 4. Calculate the color of the pixel in the resized image by multiplying each original color by its weight and adding the results.
    ///
    /// This process is repeated for every pixel in the resized image.
    fn bilinear_interpolation(&self, image: &DynamicImage) -> DynamicImage {
        // Get the dimensions of the original and resized images.
        let (width, height) = image.dimensions();
        let (new_width, new_height) = self.final_resolution;
        let mut new_image: DynamicImage = DynamicImage::new_rgba8(new_width, new_height);

        // Calculate the ratio of old dimensions to new dimensions.
        let width_ratio = width as f32 / new_width as f32;
        let height_ratio = height as f32 / new_height as f32;

        // Iterate over every pixel in the resized image.
        for y in 0..new_height {
            for x in 0..new_width {
                // Calculate the corresponding position in the original image.
                let gx = x as f32 * width_ratio;
                let gy = y as f32 * height_ratio;

                // Get the coordinates of the four pixels that surround this position.
                let gxi = gx.floor() as u32;
                let gyi = gy.floor() as u32;

                // Get the colors of these four pixels.
                let c00: image::Rgba<u8> = image.get_pixel(gxi, gyi).into();
                let c10: image::Rgba<u8> = image.get_pixel((gxi + 1).min(width - 1), gyi).into();
                let c01: image::Rgba<u8> = image.get_pixel(gxi, (gyi + 1).min(height - 1)).into();
                let c11: image::Rgba<u8> = image.get_pixel((gxi + 1).min(width - 1), (gyi + 1).min(height - 1)).into();

                // Calculate the weights for these four colors.
                let weights = [(1.0 - gx + gxi as f32) * (1.0 - gy + gyi as f32),
                            (gx - gxi as f32) * (1.0 - gy + gyi as f32),
                            (1.0 - gx + gxi as f32) * (gy - gyi as f32),
                            (gx - gxi as f32) * (gy - gyi as f32)];

                // Calculate the color of the pixel in the resized image.
                let mut pixel = [0u8; 4];
                for i in 0..4 {
                    pixel[i] = (c00.0[i] as f32 * weights[0]
                                + c10.0[i] as f32 * weights[1]
                                + c01.0[i] as f32 * weights[2]
                                + c11.0[i] as f32 * weights[3]) as u8;
                }

                // Set the color of the pixel in the resized image.
                new_image.put_pixel(x, y, Rgba(pixel));
            }
        }

        // Return the resized image.
        new_image
    }
}