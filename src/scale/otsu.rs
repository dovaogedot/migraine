use crate::{image::Image, scale::guesser::ScaleGuesser};

fn otsu_threshold(distances: &[f64]) -> f64 {
    let bins = 256;
    let mut hist = [0.0; 256];
    distances
        .iter()
        .for_each(|d| hist[(*d as usize).min(bins - 1)] += 1.0);
    let total = distances.len() as f64;

    let prob = hist.map(|h| h / total);

    let mut best_threshold = 0;
    let mut best_variance = 0.0;

    let mut w0 = 0.0;
    let mut sum0 = 0.0;
    let total_mean: f64 = prob.iter().enumerate().map(|(i, p)| *p * i as f64).sum();

    for t in 0..bins {
        w0 += prob[t];
        sum0 += prob[t] * t as f64;
        let w1 = 1.0 - w0;
        if w0 > 0.0 && w1 > 0.0 {
            let mean0 = sum0 / w0;
            let mean1 = (total_mean - sum0) / w1;
            let variance = w0 * w1 * (mean0 - mean1) * (mean0 - mean1);
            if variance > best_variance {
                best_variance = variance;
                best_threshold = t;
            }
        }
    }

    best_threshold as f64
}

pub fn detect_scale_factor<I: Image>(img: &I) -> f64 {
    let row_step = (img.height() / 100).max(1);
    let height = img.height();
    let width = img.width();

    #[rustfmt::skip]
    let distances: Vec<f64> = 
        (0..height).step_by(row_step as usize).flat_map(|y| {
            (1..width).map(move |x| {
                img.sample(x, y).distance(&img.sample(x - 1, y))
            })
        })
        .collect();

    let threshold = otsu_threshold(&distances);

    let mut density = vec![0.0; width as usize];
    for y in 0..height {
        for x in 1..width {
            if img.sample(x, y).distance(&img.sample(x - 1, y)) > threshold {
                density[x as usize] += 1.0;
            }
        }
    }

    let baseline = density.iter().sum::<f64>() / density.len() as f64;

    let interpolate = |pos: f64| -> f64 {
        let lo: usize = (pos as usize).max(0).min(density.len() - 2);
        let frac = pos - lo as f64;
        density[lo] * (1.0 - frac) + density[lo + 1] * frac
    };

    let score_scale = |s: f64| -> f64 {
        let positions: Vec<f64> = (1..((width as f64 / s) as u32)).map(|x| x as f64 * s).collect();
        if positions.is_empty() {
            f64::NEG_INFINITY
        } else {
            positions.iter().map(|p| interpolate(*p) - baseline).sum::<f64>() / positions.len() as f64
        }
    };

    let best_coarse = (15..251).map(|x| x as f64 / 10.0).max_by(|&a, &b| score_scale(a).total_cmp(&score_scale(b))).unwrap_or(0.0);

    (-20..21).map(|x| best_coarse + x as f64 / 100.0)
    .filter(|&x| x > 1.0)
    .max_by(|&a, &b| score_scale(a).total_cmp(&score_scale(b))).unwrap_or(0.0)
}

pub struct OtsuGuesser {}

impl ScaleGuesser for OtsuGuesser {
    fn guess(&self, img: &impl Image) -> Vec<f64> {
        let best = detect_scale_factor(img);
        vec![best]
    }
}