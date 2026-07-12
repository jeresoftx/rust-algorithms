use std::collections::BTreeSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Point {
    pub x: i64,
    pub y: i64,
}

impl Point {
    pub fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Orientation {
    Clockwise,
    CounterClockwise,
    Collinear,
}

pub fn cross_product(origin: Point, first: Point, second: Point) -> i64 {
    let first_x = first.x - origin.x;
    let first_y = first.y - origin.y;
    let second_x = second.x - origin.x;
    let second_y = second.y - origin.y;

    first_x * second_y - first_y * second_x
}

pub fn orientation(first: Point, second: Point, third: Point) -> Orientation {
    match cross_product(first, second, third).cmp(&0) {
        std::cmp::Ordering::Greater => Orientation::CounterClockwise,
        std::cmp::Ordering::Less => Orientation::Clockwise,
        std::cmp::Ordering::Equal => Orientation::Collinear,
    }
}

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
