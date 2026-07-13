use rust_algorithms::patterns::graphs::{
    accounts_merge, can_finish, clone_graph, count_connected_components, find_circle_num,
    find_course_order, graph_valid_tree, is_bipartite, max_area_of_island, number_of_islands,
    oranges_rotting, pacific_atlantic, redundant_connection, walls_and_gates,
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

#[test]
fn can_finish_returns_true_when_prerequisites_are_acyclic() {
    let prerequisites = vec![(1, 0), (2, 0), (3, 1), (3, 2)];

    assert!(can_finish(4, prerequisites));
}

#[test]
fn can_finish_returns_false_when_prerequisites_have_cycle() {
    let prerequisites = vec![(1, 0), (0, 1)];

    assert!(!can_finish(2, prerequisites));
}

#[test]
fn find_course_order_returns_valid_topological_order() {
    let prerequisites = vec![(1, 0), (2, 0), (3, 1), (3, 2)];

    let order = find_course_order(4, prerequisites.clone());

    assert_eq!(order.len(), 4);
    assert_topological_order(&order, &prerequisites);
}

#[test]
fn find_course_order_returns_empty_when_cycle_exists() {
    let prerequisites = vec![(1, 0), (0, 1)];

    assert!(find_course_order(2, prerequisites).is_empty());
}

#[test]
fn redundant_connection_returns_edge_that_closes_cycle() {
    let edges = vec![(1, 2), (1, 3), (2, 3)];

    assert_eq!(redundant_connection(edges), Some((2, 3)));
}

#[test]
fn redundant_connection_returns_none_for_tree_edges() {
    let edges = vec![(1, 2), (2, 3), (3, 4)];

    assert_eq!(redundant_connection(edges), None);
}

#[test]
fn count_connected_components_counts_disconnected_groups() {
    let edges = vec![(0, 1), (1, 2), (3, 4)];

    assert_eq!(count_connected_components(5, edges), 2);
}

#[test]
fn count_connected_components_counts_isolated_nodes() {
    assert_eq!(count_connected_components(4, Vec::new()), 4);
}

#[test]
fn graph_valid_tree_accepts_connected_acyclic_graph() {
    let edges = vec![(0, 1), (0, 2), (0, 3), (1, 4)];

    assert!(graph_valid_tree(5, edges));
}

#[test]
fn graph_valid_tree_rejects_cycle_or_disconnected_graph() {
    assert!(!graph_valid_tree(5, vec![(0, 1), (1, 2), (2, 0), (3, 4)]));
    assert!(!graph_valid_tree(4, vec![(0, 1), (2, 3)]));
}

#[test]
fn is_bipartite_accepts_two_colorable_graph() {
    let graph = vec![vec![1, 3], vec![0, 2], vec![1, 3], vec![0, 2]];

    assert!(is_bipartite(graph));
}

#[test]
fn is_bipartite_rejects_odd_cycle() {
    let graph = vec![vec![1, 2], vec![0, 2], vec![0, 1]];

    assert!(!is_bipartite(graph));
}

#[test]
fn find_circle_num_counts_connected_city_groups() {
    let connected = vec![vec![1, 1, 0], vec![1, 1, 0], vec![0, 0, 1]];

    assert_eq!(find_circle_num(connected), 2);
}

#[test]
fn find_circle_num_handles_all_isolated_cities() {
    let connected = vec![vec![1, 0, 0], vec![0, 1, 0], vec![0, 0, 1]];

    assert_eq!(find_circle_num(connected), 3);
}

#[test]
fn accounts_merge_groups_accounts_connected_by_email() {
    let accounts = vec![
        vec![
            String::from("John"),
            String::from("johnsmith@mail.com"),
            String::from("john_newyork@mail.com"),
        ],
        vec![
            String::from("John"),
            String::from("johnsmith@mail.com"),
            String::from("john00@mail.com"),
        ],
        vec![String::from("Mary"), String::from("mary@mail.com")],
        vec![String::from("John"), String::from("johnnybravo@mail.com")],
    ];

    let mut result = accounts_merge(accounts);
    result.sort();

    assert_eq!(
        result,
        vec![
            vec![
                String::from("John"),
                String::from("john00@mail.com"),
                String::from("john_newyork@mail.com"),
                String::from("johnsmith@mail.com"),
            ],
            vec![String::from("John"), String::from("johnnybravo@mail.com")],
            vec![String::from("Mary"), String::from("mary@mail.com")],
        ]
    );
}

#[test]
fn accounts_merge_handles_empty_input() {
    assert!(accounts_merge(Vec::new()).is_empty());
}

#[test]
fn clone_graph_rebuilds_adjacency_for_connected_component() {
    let graph = vec![vec![1, 3], vec![0, 2], vec![1, 3], vec![0, 2]];

    assert_eq!(clone_graph(graph.clone()), graph);
}

#[test]
fn clone_graph_handles_empty_graph() {
    assert!(clone_graph(Vec::new()).is_empty());
}

fn assert_topological_order(order: &[usize], prerequisites: &[(usize, usize)]) {
    let mut positions = vec![0; order.len()];

    for (index, &course) in order.iter().enumerate() {
        positions[course] = index;
    }

    for &(course, prerequisite) in prerequisites {
        assert!(positions[prerequisite] < positions[course]);
    }
}
