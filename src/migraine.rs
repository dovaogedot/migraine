use std::{
    ops::Mul,
    path::{Path, PathBuf},
    time::Instant,
};

use image::{Rgb, RgbImage};

use crate::{
    color::palette::Palette,
    downsample::{Downsampler, SamplePattern},
    error::MigraineError,
    io::IO,
    scale::guess_pixel_size,
};

pub struct Migraine;
impl Migraine {
    pub fn restore(
        path: PathBuf,
        scale: Option<f64>,
        width: Option<u32>,
        height: Option<u32>,
        colors: Option<u32>,
        max_colors: Option<u32>,
    ) -> Result<(), MigraineError> {
        log::debug!("Loading image...");

        let image = IO::open_image(&path)?.to_rgb32f();

        let image_width = image.width() as f64;
        let image_height = image.height() as f64;

        let (scale, width, height): (f64, f64, f64) = match (scale, width, height) {
            (None, None, None) => {
                log::warn!("Inferring pixel size programmatically...");
                let s = guess_pixel_size(&image);
                let w = image_width / s;
                let h = image_height / s;
                (s, w, h)
            }
            (None, None, Some(h)) => {
                let h = h as f64;
                let s = image_height / h;
                let w = image_width / s;
                (s, w, h)
            }
            (None, Some(w), None) => {
                let w = w as f64;
                let s = image_width / w;
                let h = image_height / s;
                (s, w, h)
            }
            (None, Some(w), Some(h)) => {
                let w = w as f64;
                let h = h as f64;
                let s = (image_width / w + image_height / h) / 2.0;
                (s, w, h)
            }
            (Some(s), None, None) => {
                let w = image_width / s;
                let h = image_height / s;
                (s, w, h)
            }
            (_, _, _) => return Err(MigraineError::SuppliedBothDimensionsAndScale),
        };

        let target_width = width.round() as u32;
        let target_height = height.round() as u32;

        log::info!("Scale: {scale:?}");
        log::info!("Width: {target_width:?}");
        log::info!("Height: {target_height:?}");

        let downsampler = Downsampler::default();
        let sample_pattern = SamplePattern::default();

        let start = Instant::now();

        log::debug!("Downsampling...");

        let downsampled = downsampler.downsample(&image, target_width, target_height, sample_pattern);

        let palette = Palette::new(
            downsampled
                .pixels()
                .map(|p| Rgb::<f64>::from([p.0[0] as f64, p.0[1] as f64, p.0[2] as f64]))
                .collect(),
        );

        log::debug!("Uncompressed palette size: {}", palette.colors().len());

        // let data: Vec<[f32; 3]> = downsampled.pixels().map(|p| p.0).collect();
        // send_to_visualizer(&data);

        let reduced_palette = match colors {
            None => {
                log::debug!("Compressing palette to best size...");
                palette.reduced_auto(max_colors)
            }
            Some(n) => {
                log::debug!("Compressing palette to {n} colors...");
                palette.reduced(n)
            }
        };

        log::info!("Colors: {}", reduced_palette.colors().len());

        log::debug!("Remapping pixel colors...");

        let posterized_pixels: Vec<u8> = palette
            .colors()
            .iter()
            .flat_map(|p| {
                let color = reduced_palette.closest_to(&p);
                [
                    color.0[0].mul(255.0).round() as u8,
                    color.0[1].mul(255.0).round() as u8,
                    color.0[2].mul(255.0).round() as u8,
                ]
            })
            .collect();

        log::trace!("Contructing new image...");
        let restored = RgbImage::from_raw(downsampled.width(), downsampled.height(), posterized_pixels).unwrap();

        let total = start.elapsed();

        log::info!("Processing took {}ms", total.as_millis());

        let new_path_str = format!("{}_downsampled.bmp", path.to_string_lossy());
        let new_path = Path::new(&new_path_str);

        log::debug!("Saving image to file...");

        IO::save_image(&restored, &new_path)?;

        println!("\x1b[1m{target_width}\x1b[0mx\x1b[1m{target_height} {reduced_palette}");

        Ok(())
    }
}
