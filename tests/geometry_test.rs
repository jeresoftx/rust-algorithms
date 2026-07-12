use rust_algorithms::patterns::geometry::{
    convex_hull, cross_product, k_closest_points, max_points_on_a_line, orientation, Orientation,
    Point,
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

#[test]
fn k_closest_points_returns_nearest_points_to_origin() {
    let points = vec![
        Point::new(1, 3),
        Point::new(-2, 2),
        Point::new(5, 8),
        Point::new(0, 1),
    ];

    assert_eq!(
        sorted_points(k_closest_points(points, 2)),
        vec![Point::new(-2, 2), Point::new(0, 1)]
    );
}

#[test]
fn k_closest_points_breaks_ties_by_coordinates() {
    let points = vec![
        Point::new(1, 1),
        Point::new(-1, -1),
        Point::new(1, -1),
        Point::new(-1, 1),
    ];

    assert_eq!(
        k_closest_points(points, 3),
        vec![Point::new(-1, -1), Point::new(-1, 1), Point::new(1, -1)]
    );
}

#[test]
fn k_closest_points_handles_large_k_and_empty_input() {
    assert_eq!(
        k_closest_points(vec![Point::new(3, 4)], 3),
        vec![Point::new(3, 4)]
    );
    assert_eq!(k_closest_points(Vec::new(), 2), Vec::<Point>::new());
}

#[test]
fn max_points_on_a_line_counts_diagonal_points() {
    let points = vec![
        Point::new(1, 1),
        Point::new(2, 2),
        Point::new(3, 3),
        Point::new(1, 3),
    ];

    assert_eq!(max_points_on_a_line(points), 3);
}

#[test]
fn max_points_on_a_line_handles_vertical_and_horizontal_lines() {
    assert_eq!(
        max_points_on_a_line(vec![
            Point::new(2, 1),
            Point::new(2, 3),
            Point::new(2, 5),
            Point::new(4, 5),
        ]),
        3
    );
    assert_eq!(
        max_points_on_a_line(vec![
            Point::new(0, 7),
            Point::new(2, 7),
            Point::new(4, 7),
            Point::new(4, 9),
        ]),
        3
    );
}

#[test]
fn max_points_on_a_line_handles_duplicates() {
    assert_eq!(
        max_points_on_a_line(vec![
            Point::new(1, 1),
            Point::new(1, 1),
            Point::new(2, 2),
            Point::new(3, 3),
        ]),
        4
    );
}

#[test]
fn max_points_on_a_line_handles_small_inputs() {
    assert_eq!(max_points_on_a_line(Vec::new()), 0);
    assert_eq!(max_points_on_a_line(vec![Point::new(5, 8)]), 1);
    assert_eq!(
        max_points_on_a_line(vec![Point::new(5, 8), Point::new(9, 13)]),
        2
    );
}
