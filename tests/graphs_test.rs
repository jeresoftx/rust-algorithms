use rust_algorithms::patterns::graphs::{
    max_area_of_island, number_of_islands, oranges_rotting, pacific_atlantic, walls_and_gates,
};

#[test]
fn number_of_islands_counts_disconnected_land_groups() {
    let grid = vec![
        vec!['1', '1', '0', '0', '0'],
        vec!['1', '1', '0', '0', '0'],
        vec!['0', '0', '1', '0', '0'],
        vec!['0', '0', '0', '1', '1'],
    ];

    assert_eq!(number_of_islands(grid), 3);
}

#[test]
fn number_of_islands_returns_zero_for_empty_grid() {
    assert_eq!(number_of_islands(Vec::new()), 0);
}

#[test]
fn max_area_of_island_returns_largest_connected_area() {
    let grid = vec![
        vec![0, 0, 1, 0, 0, 0, 1, 1],
        vec![0, 0, 1, 1, 0, 0, 1, 0],
        vec![0, 0, 0, 0, 0, 1, 1, 0],
        vec![1, 1, 0, 0, 0, 0, 0, 0],
    ];

    assert_eq!(max_area_of_island(grid), 5);
}

#[test]
fn max_area_of_island_returns_zero_when_there_is_no_land() {
    assert_eq!(max_area_of_island(vec![vec![0, 0], vec![0, 0]]), 0);
}

#[test]
fn pacific_atlantic_returns_cells_reaching_both_oceans() {
    let heights = vec![
        vec![1, 2, 2, 3, 5],
        vec![3, 2, 3, 4, 4],
        vec![2, 4, 5, 3, 1],
        vec![6, 7, 1, 4, 5],
        vec![5, 1, 1, 2, 4],
    ];

    let mut result = pacific_atlantic(heights);
    result.sort_unstable();

    assert_eq!(
        result,
        vec![(0, 4), (1, 3), (1, 4), (2, 2), (3, 0), (3, 1), (4, 0)]
    );
}

#[test]
fn pacific_atlantic_handles_single_cell() {
    assert_eq!(pacific_atlantic(vec![vec![7]]), vec![(0, 0)]);
}

#[test]
fn oranges_rotting_returns_minutes_until_all_fresh_oranges_rot() {
    let grid = vec![vec![2, 1, 1], vec![1, 1, 0], vec![0, 1, 1]];

    assert_eq!(oranges_rotting(grid), 4);
}

#[test]
fn oranges_rotting_returns_minus_one_when_fresh_orange_is_unreachable() {
    let grid = vec![vec![2, 1, 1], vec![0, 1, 1], vec![1, 0, 1]];

    assert_eq!(oranges_rotting(grid), -1);
}

#[test]
fn walls_and_gates_fills_each_room_with_nearest_gate_distance() {
    let inf = i32::MAX;
    let mut rooms = vec![
        vec![inf, -1, 0, inf],
        vec![inf, inf, inf, -1],
        vec![inf, -1, inf, -1],
        vec![0, -1, inf, inf],
    ];

    walls_and_gates(&mut rooms);

    assert_eq!(
        rooms,
        vec![
            vec![3, -1, 0, 1],
            vec![2, 2, 1, -1],
            vec![1, -1, 2, -1],
            vec![0, -1, 3, 4],
        ]
    );
}

#[test]
fn walls_and_gates_leaves_empty_input_unchanged() {
    let mut rooms = Vec::new();

    walls_and_gates(&mut rooms);

    assert!(rooms.is_empty());
}
