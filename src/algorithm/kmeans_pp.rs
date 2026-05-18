use crate::algorithm::kmeans::{Centroid, Distance, Same, kmeans};

/// Implementation of k-means++ algorithm.
///
/// Ripped directly ripped from its [Wikipedia article](https://en.wikipedia.org/wiki/K-means%2B%2B#Improved_initialization_algorithm).
///
/// # Examples
///
/// ```
/// struct Point(u32);
///
/// impl Distance for Point {
///     fn distance(a: &Self, b: &Self) -> f64 {
///         (a.0 - b.0) as f64
///     }
/// }
///
/// impl Same for Point {
///     fn same(a: &Self, b: &Self) -> bool {
///         a.0 == b.0
///     }
/// }
///
/// impl Centroid for Point {
///     fn centroid(cluster: &[Self]) -> Self {
///         cluster.iter().sum() / cluster.len()
///     }
/// }
///
/// let k = 2;
/// let points = vec![Point(3), Point(1), Point(9), Point(11), Point(8)];
///
/// let clusters = kmeans_pp(k, points);
/// assert_eq!(clusters.len(), 2);
/// ```
pub fn kmeans_pp<P>(k: usize, points: &[P]) -> Vec<Vec<P>>
where
    P: Centroid + Distance + Same + Clone,
{
    // Initialize list of centroids with one randomly selected point
    let mut centroids: Vec<P> = Vec::with_capacity(k);
    let first_index = std::random::random::<usize>(..) % points.len();
    centroids.push(points[first_index].clone());

    // Choose remaining k - 1 centroids
    while centroids.len() < k {
        let mut distances_squared: Vec<f64> = Vec::with_capacity(points.len());

        // For each point, compute squared distance to nearest selected centroid
        for point in points {
            let mut min_distance = Distance::distance(point, &centroids[0]);
            for centroid in &centroids {
                let d = Distance::distance(point, &centroid);
                min_distance = min_distance.min(d);
            }
            distances_squared.push(min_distance * min_distance);
        }

        // Choose next centroid with probability proportional to D(x)^2
        let total: f64 = distances_squared.iter().sum();
        let threshold = std::random::random::<usize>(..) as f64 % total;
        let mut cumulative = 0.0;

        for i in 0..points.len() {
            cumulative += distances_squared[i];
            if cumulative >= threshold {
                centroids.push(points[i].clone());
                break;
            }
        }
    }

    let clusters = kmeans(k, points, centroids);
    clusters
}
