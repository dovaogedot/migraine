use std::ops::Div;

pub enum Extremum {
    Minimum,
    Maximum,
}

pub trait Period {
    fn scores(x: &[f64]) -> Vec<f64>;
    fn extremum() -> Extremum;

    fn is_extremum(x: &[f64], t: usize) -> bool {
        let left = if t == 0 { x.len() - 1 } else { t - 1 };
        let right = if t == x.len() - 1 { 0 } else { t + 1 };
        match Self::extremum() {
            Extremum::Maximum => x[t] > x[left] && x[t] > x[right],
            Extremum::Minimum => x[t] < x[left] && x[t] < x[right],
        }
    }

    /// Collect all local extremums with parabolic interpolation
    fn extremums(x: &[f64]) -> Vec<f64> {
        (1..x.len())
            .filter(|&t| Self::is_extremum(x, t))
            .map(|t| {
                let left = if t == 0 { x.len() - 1 } else { t - 1 };
                let right = if t == x.len() - 1 { 0 } else { t + 1 };
                let (y0, y1, y2) = (x[left], x[t], x[right]);
                let delta = 0.5 * (y0 - y2) / (y0 - 2.0 * y1 + y2);
                t as f64 + delta
            })
            .collect()
    }

    /// Finds period of a function
    fn period(x: &[f64]) -> f64 {
        let scores: Vec<f64> = Self::scores(x);
        let extremums = Self::extremums(&scores);
        // k-th extremum ≈ k·T  →  T ≈ mean(extremum[k] / (k+1))
        let t_estimates = extremums.iter().enumerate().map(|(k, &pos)| pos / (k + 1) as f64);
        t_estimates.clone().sum::<f64>() / extremums.len() as f64
    }
}

pub struct ACF;
impl ACF {
    /// Maps each value to its product with the lagged one. Results in peaks where
    /// lag approaches period.
    ///
    /// # Parameters
    ///
    ///
    fn acf(x: &[f64], t: usize, tau: usize, w: usize) -> f64 {
        let len = x.len();
        (t + 1..=t + w).map(|j| x[j % len] * x[(j + tau) % len]).sum::<f64>()
    }
}

impl Period for ACF {
    fn scores(x: &[f64]) -> Vec<f64> {
        (0..=x.len() / 2).map(|lag| Self::acf(x, 0, lag, x.len())).collect()
    }

    fn extremum() -> Extremum {
        Extremum::Maximum
    }
}

pub struct AMDF;
impl AMDF {
    /// Maps each value to its absolute difference with the lagged one. Results in
    /// valleys where lag approaches period.
    fn amdf(x: &[f64], m: usize) -> f64 {
        let len = x.len();

        (0..len)
            .map(|n| (x[n] - x[(n + m) % len]).abs())
            .sum::<f64>()
            .div(len as f64)
    }
}

impl Period for AMDF {
    fn scores(x: &[f64]) -> Vec<f64> {
        (0..=x.len() / 2).map(|lag| Self::amdf(x, lag)).collect()
    }

    fn extremum() -> Extremum {
        Extremum::Minimum
    }
}

pub struct YIN;
impl YIN {
    /// Maps each value to its squared difference with the lagged one. Results in
    /// valleys where lag approaches period.
    fn yin(x: &[f64], tau: usize, w: usize) -> f64 {
        let len = x.len();
        (1..=w).map(|j| (x[j % len] - x[(j + tau) % len]).powi(2)).sum::<f64>()
    }

    /// Cumulative mean normalized difference function.
    ///
    /// Like the normal one, but divides each value by its average over previous
    /// values.
    fn yin_cmndf(differences: &[f64]) -> Vec<f64> {
        let mut result = vec![1.0];
        let mut running_sum = 0.0;
        for (i, &d) in differences.iter().enumerate().skip(1) {
            running_sum += d;
            result.push(d * i as f64 / running_sum);
        }
        result
    }
}

impl Period for YIN {
    fn scores(x: &[f64]) -> Vec<f64> {
        let raw: Vec<f64> = (0..=x.len() / 2).map(|lag| Self::yin(x, lag, x.len())).collect();
        Self::yin_cmndf(&raw)
    }

    fn extremum() -> Extremum {
        Extremum::Minimum
    }
}
