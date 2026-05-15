use crate::image::Image;

pub trait ScaleGuesser {
    /// Returns best guesses for pixel scale ordered from best match to worst
    fn guess(&self, img: &impl Image) -> Vec<f64>;
}