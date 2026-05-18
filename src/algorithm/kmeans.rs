pub trait Centroid {
    fn centroid(cluster: &[Self]) -> Self
    where
        Self: Sized;
}

pub trait Distance {
    fn distance(a: &Self, b: &Self) -> f64;
}

pub trait Same {
    fn same(a: &Self, b: &Self) -> bool;
}

/// Implementation of naive k-means algorithm.
///
/// Ripped directly ripped from its [Wikipedia article](https://en.wikipedia.org/wiki/K-means_clustering#Standard_algorithm_(naive_k-means)).
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
/// let clusters = kmeans(k, points);
/// assert_eq!(clusters.len(), 2);
/// ```
pub fn kmeans<P>(k: usize, points: &[P]) -> Vec<Vec<P>>
where
    P: Centroid + Distance + Same + Clone,
{
    assert!(k > 0, "Number of cluster must be greater than zero");
    assert!(
        points.len() >= k,
        "Number of points should not less than number of clusters"
    );

    // Initialize centroids
    let mut centroids: Vec<P> = points.iter().take(k).cloned().collect();
    let mut converged = false;
    let mut clusters: Vec<Vec<P>> = Vec::with_capacity(k);

    while !converged {
        // Create empty clusters
        clusters = vec![vec![]; k];

        // Assign each point to the nearest centroid
        for point in points {
            let mut closest_index = 0;
            let mut min_distance = P::distance(point, &centroids[0]);
            for j in 1..k {
                let d = P::distance(point, &centroids[j]);
                if d < min_distance {
                    min_distance = d;
                    closest_index = j;
                }
            }
            clusters[closest_index].push(point.clone());
        }

        // Recalculate centroids as the mean of each cluster
        let new_centroids: Vec<P> = clusters.iter().map(|c| P::centroid(c)).collect();

        // Check for convergence
        if new_centroids
            .iter()
            .zip(&centroids)
            .all(|x| P::same(x.0, x.1))
        {
            converged = true;
        } else {
            centroids = new_centroids;
        }
    }

    clusters
}
