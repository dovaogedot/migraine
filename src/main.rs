use std::path::Path;

use clap::Parser;
use image::{DynamicImage, RgbImage};

use crate::args::MigraineArgs;

mod algorithm;
mod args;
mod downsample;
mod error;
mod migraine;
mod color;
mod visualizer;


fn main() -> std::io::Result<()> {
    let args = MigraineArgs::parse();

    let image = open_image(&args.path)?.to_rgb32f();

    let result = migraine::restore(&image, args.scale, args.width, args.height, args.colors, args.max_colors)?;

    println!("Processing took {}ms", result.time_spent.as_millis());
    println!("Approximated palette:\n{}", result.palette);

    let new_path_str = format!("{}_downsampled.bmp", args.path.to_string_lossy());
    let new_path = Path::new(&new_path_str);
    save_image(&result.image, &new_path)?;
    Ok(())
}

fn open_image(path: &Path) -> std::io::Result<DynamicImage> {
    image::ImageReader::open(path)?
        .with_guessed_format()?
        .decode()
        .map_err(|e| std::io::Error::other(format!("Could not decode image at path '{:?}'. {}", path, e)))
}

fn save_image(image: &RgbImage, path: &Path) -> std::io::Result<()> {
    image
        .save_with_format(path, image::ImageFormat::Bmp)
        .map_err(std::io::Error::other)
}

#[cfg(test)]
mod scale {
    use std::ops::{Add, Div};

    use super::*;

    struct ScaleTest {
        path: &'static str,
        scale: f64,
    }

    #[test]
    fn angle() {
        test_scale(ScaleTest {
            path: "./samples/angel_200x200_5.4_4.webp",
            scale: (1080_f64 / 200.0).add(1080.0 / 200.0).div(2.0),
        });
    }

    #[test]
    fn sailor() {
        test_scale(ScaleTest {
            path: "./samples/sailor_160x144_4_4.png",
            scale: (640_f64 / 160.0).add(576.0 / 144.0).div(2.0),
        });
    }

    #[test]
    fn skull() {
        test_scale(ScaleTest {
            path: "./samples/skull_167x174_6.67_4.png",
            scale: (1114_f64 / 167.0).add(1160.0 / 174.0).div(2.0),
        });
    }

    #[test]
    fn sunset() {
        test_scale(ScaleTest {
            path: "./samples/sunset_252x142_7.61_x.jpg",
            scale: (1920_f64 / 252.0).add(1080.0 / 142.0).div(2.0),
        });
    }

    fn test_scale(case: ScaleTest) {
        let image = open_image(Path::new(case.path)).unwrap().to_rgb32f();
        let result = migraine::guess_pixel_size(&image);
        let expected = case.scale;
        println!("expected: {expected}, got: {result}");
        assert!((result - expected).abs() < 0.05)
    }
}
