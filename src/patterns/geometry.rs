//! Computational geometry helpers.
//!
//! # Example
//!
//! ```
//! use rust_algorithms::patterns::geometry::{orientation, Orientation, Point};
//!
//! let a = Point::new(0, 0);
//! let b = Point::new(1, 0);
//! let c = Point::new(1, 1);
//! assert_eq!(orientation(a, b, c), Orientation::CounterClockwise);
//! ```

use std::collections::{BTreeMap, BTreeSet, HashMap};

/// Integer point in a two-dimensional plane.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Point {
    /// Horizontal coordinate.
    pub x: i64,
    /// Vertical coordinate.
    pub y: i64,
}

impl Point {
    /// Creates a point from integer coordinates.
    pub fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }
}

/// Orientation of an ordered triple of points.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Orientation {
    /// The turn goes clockwise.
    Clockwise,
    /// The turn goes counter-clockwise.
    CounterClockwise,
    /// All three points lie on the same line.
    Collinear,
}

/// Returns the signed cross product of vectors `origin -> first` and
/// `origin -> second`.
///
/// Time: O(1)
/// Space: O(1)
pub fn cross_product(origin: Point, first: Point, second: Point) -> i64 {
    let first_x = first.x - origin.x;
    let first_y = first.y - origin.y;
    let second_x = second.x - origin.x;
    let second_y = second.y - origin.y;

    first_x * second_y - first_y * second_x
}

/// Classifies the turn made by three points.
///
/// Time: O(1)
/// Space: O(1)
pub fn orientation(first: Point, second: Point, third: Point) -> Orientation {
    match cross_product(first, second, third).cmp(&0) {
        std::cmp::Ordering::Greater => Orientation::CounterClockwise,
        std::cmp::Ordering::Less => Orientation::Clockwise,
        std::cmp::Ordering::Equal => Orientation::Collinear,
    }
}

/// Computes the convex hull with the monotonic-chain algorithm.
///
/// Boundary collinear points are preserved.
///
/// Time: O(n log n)
/// Space: O(n)
pub fn convex_hull(points: Vec<Point>) -> Vec<Point> {
    let mut points = points;
    points.sort_unstable();
    points.dedup();

    if points.len() <= 1 {
        return points;
    }

    let mut lower = Vec::new();
    for &point in &points {
        while lower.len() >= 2
            && cross_product(lower[lower.len() - 2], lower[lower.len() - 1], point) < 0
        {
            lower.pop();
        }

        lower.push(point);
    }

    let mut upper = Vec::new();
    for &point in points.iter().rev() {
        while upper.len() >= 2
            && cross_product(upper[upper.len() - 2], upper[upper.len() - 1], point) < 0
        {
            upper.pop();
        }

        upper.push(point);
    }

    lower
        .into_iter()
        .chain(upper)
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect()
}

/// Returns the `k` points closest to the origin by squared distance.
///
/// Time: O(n log n)
/// Space: O(1) excluding the input vector.
pub fn k_closest_points(mut points: Vec<Point>, k: usize) -> Vec<Point> {
    points.sort_unstable_by_key(|point| (squared_distance_to_origin(*point), point.x, point.y));
    points.truncate(k.min(points.len()));
    points
}

fn squared_distance_to_origin(point: Point) -> i64 {
    point.x * point.x + point.y * point.y
}

/// Counts the largest number of points that lie on one line.
///
/// Time: O(n^2)
/// Space: O(n)
pub fn max_points_on_a_line(points: Vec<Point>) -> i32 {
    if points.len() <= 2 {
        return points.len() as i32;
    }

    let mut best = 0;

    for (anchor_index, &anchor) in points.iter().enumerate() {
        let mut slopes = HashMap::new();
        let mut duplicates = 1;
        let mut best_from_anchor = 0;

        for &point in points.iter().skip(anchor_index + 1) {
            if point == anchor {
                duplicates += 1;
                continue;
            }

            let slope = normalized_slope(anchor, point);
            let count = slopes.entry(slope).or_insert(0);
            *count += 1;
            best_from_anchor = best_from_anchor.max(*count);
        }

        best = best.max(best_from_anchor + duplicates);
    }

    best
}

/// Computes the skyline key points for axis-aligned buildings.
///
/// Time: O(n log n)
/// Space: O(n)
pub fn get_skyline(buildings: Vec<(i32, i32, i32)>) -> Vec<(i32, i32)> {
    let mut events = Vec::new();

    for (left, right, height) in buildings {
        if left >= right || height <= 0 {
            continue;
        }

        events.push((left, -height));
        events.push((right, height));
    }

    events.sort_unstable();

    let mut active_heights = BTreeMap::new();
    active_heights.insert(0, 1);
    let mut previous_height = 0;
    let mut skyline = Vec::new();
    let mut index = 0;

    while index < events.len() {
        let x = events[index].0;

        while index < events.len() && events[index].0 == x {
            let height = events[index].1;

            if height < 0 {
                *active_heights.entry(-height).or_insert(0) += 1;
            } else if let Some(count) = active_heights.get_mut(&height) {
                *count -= 1;
                if *count == 0 {
                    active_heights.remove(&height);
                }
            }

            index += 1;
        }

        let current_height = active_heights
            .last_key_value()
            .map_or(0, |(&height, _)| height);

        if current_height != previous_height {
            skyline.push((x, current_height));
            previous_height = current_height;
        }
    }

    skyline
}

fn normalized_slope(first: Point, second: Point) -> (i64, i64) {
    let mut dy = second.y - first.y;
    let mut dx = second.x - first.x;

    if dx == 0 {
        return (1, 0);
    }

    if dy == 0 {
        return (0, 1);
    }

    let divisor = gcd(dy.abs(), dx.abs());
    dy /= divisor;
    dx /= divisor;

    if dx < 0 {
        dy = -dy;
        dx = -dx;
    }

    (dy, dx)
}

fn gcd(mut left: i64, mut right: i64) -> i64 {
    while right != 0 {
        let remainder = left % right;
        left = right;
        right = remainder;
    }

    left.abs()
}
