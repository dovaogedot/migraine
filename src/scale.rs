use std::ops::Div;

use image::Rgb32FImage;

use crate::algorithm::{
    autocorrelation::{Period, YIN},
    kmeans::Distance,
};

/// Guesses pixel size by transforming the image into a function of differences
/// between two adjacent pixels and then applying YIN autocorrelation function
/// to find its perid.
pub fn guess_pixel_size(image: &Rgb32FImage) -> f64 {
    let img_width = image.width();
    let img_height = image.height();
    let rows_to_sample = img_height.min(256);
    let dy = (img_height / rows_to_sample) as usize;

    // Map pixel in each row to its color distance with the one next to it
    let distances: Vec<f64> = (0..img_width)
        .map(|x| {
            (0..img_height)
                .step_by(dy)
                .map(|y| {
                    let left = image.get_pixel(x, y);
                    let right = if x + 1 == img_width {
                        image.get_pixel(0, y)
                    } else {
                        image.get_pixel(x + 1, y)
                    };

                    left.distance(right)
                })
                .sum::<f64>()
                .div(rows_to_sample as f64)
        })
        .collect();

    YIN::period(&distances)
}

#[cfg(test)]
mod scale {
    use std::{
        ops::{Add, Div},
        path::Path,
    };

    use crate::io::IO;

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
        let image = IO::open_image(Path::new(case.path)).unwrap().to_rgb32f();
        let result = guess_pixel_size(&image);
        let expected = case.scale;
        println!("expected: {expected}, got: {result}");
        assert!((result - expected).abs() < 0.05)
    }
}
