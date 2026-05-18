use crate::types::Image;

/// Tries its best to guess the relationship: (provided image pixel size) / (original art pixel size)
pub trait ScaleGuesser {
    /// Returns best guesses for pixel scale ordered from best match to worst
    fn guess(&self, img: &impl Image) -> Vec<f64>;
}
