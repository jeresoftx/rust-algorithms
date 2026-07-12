use rust_algorithms::patterns::geometry::{
    convex_hull, cross_product, orientation, Orientation, Point,
};

fn sorted_points(mut points: Vec<Point>) -> Vec<Point> {
    points.sort_unstable();
    points
}

#[test]
fn cross_product_detects_turn_direction() {
    let origin = Point::new(0, 0);

    assert_eq!(cross_product(origin, Point::new(2, 0), Point::new(1, 3)), 6);
    assert_eq!(
        cross_product(origin, Point::new(1, 3), Point::new(2, 0)),
        -6
    );
    assert_eq!(cross_product(origin, Point::new(2, 2), Point::new(4, 4)), 0);
}

#[test]
fn orientation_classifies_three_points() {
    assert_eq!(
        orientation(Point::new(0, 0), Point::new(2, 0), Point::new(1, 3)),
        Orientation::CounterClockwise
    );
    assert_eq!(
        orientation(Point::new(0, 0), Point::new(1, 3), Point::new(2, 0)),
        Orientation::Clockwise
    );
    assert_eq!(
        orientation(Point::new(0, 0), Point::new(2, 2), Point::new(4, 4)),
        Orientation::Collinear
    );
}

#[test]
fn convex_hull_returns_outer_points() {
    let points = vec![
        Point::new(1, 1),
        Point::new(2, 2),
        Point::new(2, 0),
        Point::new(2, 4),
        Point::new(3, 3),
        Point::new(4, 2),
    ];

    assert_eq!(
        sorted_points(convex_hull(points)),
        vec![
            Point::new(1, 1),
            Point::new(2, 0),
            Point::new(2, 4),
            Point::new(3, 3),
            Point::new(4, 2),
        ]
    );
}

#[test]
fn convex_hull_keeps_collinear_boundary_points() {
    let points = vec![
        Point::new(0, 0),
        Point::new(1, 0),
        Point::new(2, 0),
        Point::new(2, 1),
        Point::new(2, 2),
        Point::new(1, 2),
        Point::new(0, 2),
        Point::new(0, 1),
        Point::new(1, 1),
    ];

    assert_eq!(
        sorted_points(convex_hull(points)),
        vec![
            Point::new(0, 0),
            Point::new(0, 1),
            Point::new(0, 2),
            Point::new(1, 0),
            Point::new(1, 2),
            Point::new(2, 0),
            Point::new(2, 1),
            Point::new(2, 2),
        ]
    );
}

#[test]
fn convex_hull_handles_duplicates_and_small_inputs() {
    assert_eq!(
        convex_hull(vec![Point::new(1, 1), Point::new(1, 1)]),
        vec![Point::new(1, 1)]
    );
    assert_eq!(convex_hull(Vec::new()), Vec::<Point>::new());
}
