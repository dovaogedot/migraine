use std::ops::Sub;

use image_lib::Rgb;

use crate::algorithm::kmeans::{Centroid, Distance, Same};

impl Centroid for Rgb<f64> {
    fn centroid(cluster: &[Self]) -> Self {
        let (r, g, b) = cluster
            .iter()
            .map(|x| x.0)
            .fold((0.0, 0.0, 0.0), |(r_sum, g_sum, b_sum), [r, g, b]| {
                (r_sum + r, g_sum + g, b_sum + b)
            });
        let total = cluster.len() as f64;
        [r / total, g / total, b / total].into()
    }
}

impl Distance for Rgb<f64> {
    fn distance(this: &Self, that: &Self) -> f64 {
        let r = this[0] - that[0];
        let g = this[1] - that[1];
        let b = this[2] - that[2];
        ((r * r) * 0.3 + (g * g) * 0.59 + (b * b) * 0.11).sqrt()
    }
}

impl Distance for Rgb<f32> {
    fn distance(this: &Self, that: &Self) -> f64 {
        let r = (this[0] - that[0]) as f64;
        let g = (this[1] - that[1]) as f64;
        let b = (this[2] - that[2]) as f64;
        ((r * r) * 0.3 + (g * g) * 0.59 + (b * b) * 0.11).sqrt()
    }
}

impl Same for Rgb<f64> {
    fn same_clusters(this: &[Self], that: &[Self]) -> bool {
        if this.len() != that.len() {
            return false;
        }

        this.iter().zip(that).all(|(a, b)| {
            a[0].sub(b[0]).abs() < f64::EPSILON
                && a[1].sub(b[1]).abs() < f64::EPSILON
                && a[2].sub(b[2]).abs() < f64::EPSILON
        })
    }
}

pub fn to_rgb_string(color: &Rgb<f64>) -> String {
    format!("({:3}, {:3}, {:3})", color[0], color[1], color[2],)
}

pub fn to_hex_string(color: &Rgb<f64>) -> String {
    format!(
        "#{:02x}{:02x}{:02x}",
        color[0].round() as u32,
        color[1].round() as u32,
        color[2].round() as u32,
    )
}

pub fn to_string(color: &Rgb<f64>) -> String {
    format!("{} {}", to_hex_string(color), to_rgb_string(color))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rgb_string() {
        let color: Rgb<f64> = [255.0, 127.0, 64.0].into();
        let rgb_string = to_rgb_string(&color);
        assert_eq!(rgb_string, "(255, 127,  64)");
    }

    #[test]
    fn hex_string() {
        let color: Rgb<f64> = [255.0, 127.0, 64.0].into();
        let hex_string = to_hex_string(&color);
        assert_eq!(hex_string, "#ff7f40");
    }

    #[test]
    fn distance() {
        let yellow: Rgb<f64> = [250.0, 250.0, 10.0].into();
        let red: Rgb<f64> = [250.0, 10.0, 10.0].into();
        let blue: Rgb<f64> = [10.0, 10.0, 250.0].into();

        let dist_to_red = Distance::distance(&yellow, &red);
        let dist_to_blue = Distance::distance(&yellow, &blue);

        assert!(dist_to_red < dist_to_blue)
    }

    #[test]
    fn distance_same_color_0() {
        let a: Rgb<f64> = [220.0, 200.0, 180.0].into();
        let b: Rgb<f64> = [220.0, 200.0, 180.0].into();

        let dist_ab = Distance::distance(&a, &b);

        assert!(dist_ab < f64::EPSILON)
    }
}
