use rust_algorithms::patterns::weighted_graphs::{
    bellman_ford_shortest_paths, cheapest_flight_within_k_stops, critical_connections,
    dijkstra_shortest_paths, floyd_warshall_all_pairs, kruskal_minimum_spanning_tree_weight,
    minimum_effort_path, network_delay_time, prim_minimum_spanning_tree_weight, BellmanFordError,
};

#[test]
fn dijkstra_returns_shortest_paths_from_start() {
    let edges = vec![(0, 1, 4), (0, 2, 1), (2, 1, 2), (1, 3, 1), (2, 3, 5)];

    assert_eq!(
        dijkstra_shortest_paths(4, &edges, 0),
        vec![Some(0), Some(3), Some(1), Some(4)]
    );
}

#[test]
fn dijkstra_marks_unreachable_nodes_as_none() {
    let edges = vec![(0, 1, 7), (2, 3, 1)];

    assert_eq!(
        dijkstra_shortest_paths(4, &edges, 0),
        vec![Some(0), Some(7), None, None]
    );
}

#[test]
fn dijkstra_returns_empty_when_start_is_out_of_range() {
    let edges = vec![(0, 1, 7)];

    assert!(dijkstra_shortest_paths(2, &edges, 9).is_empty());
}

#[test]
fn network_delay_time_returns_max_shortest_path() {
    let times = vec![(2, 1, 1), (2, 3, 1), (3, 4, 1)];

    assert_eq!(network_delay_time(times, 4, 2), 2);
}

#[test]
fn network_delay_time_returns_minus_one_when_node_is_unreachable() {
    let times = vec![(1, 2, 1)];

    assert_eq!(network_delay_time(times, 3, 1), -1);
}

#[test]
fn network_delay_time_rejects_invalid_start() {
    assert_eq!(network_delay_time(Vec::new(), 3, 0), -1);
}

#[test]
fn bellman_ford_handles_negative_edges_without_negative_cycle() {
    let edges = vec![
        (0, 1, 6),
        (0, 2, 7),
        (1, 3, 5),
        (1, 2, 8),
        (1, 4, -4),
        (2, 3, -3),
        (2, 4, 9),
        (3, 1, -2),
        (4, 3, 7),
    ];

    assert_eq!(
        bellman_ford_shortest_paths(5, &edges, 0),
        Ok(vec![Some(0), Some(2), Some(7), Some(4), Some(-2)])
    );
}

#[test]
fn bellman_ford_detects_reachable_negative_cycle() {
    let edges = vec![(0, 1, 1), (1, 2, -1), (2, 1, -1)];

    assert_eq!(
        bellman_ford_shortest_paths(3, &edges, 0),
        Err(BellmanFordError::NegativeCycle)
    );
}

#[test]
fn bellman_ford_marks_unreachable_nodes_as_none() {
    let edges = vec![(0, 1, 3), (2, 3, -5)];

    assert_eq!(
        bellman_ford_shortest_paths(4, &edges, 0),
        Ok(vec![Some(0), Some(3), None, None])
    );
}

#[test]
fn floyd_warshall_returns_all_pairs_shortest_paths() {
    let edges = vec![(0, 1, 3), (0, 2, 8), (1, 2, 2), (2, 3, 1), (3, 0, 4)];

    assert_eq!(
        floyd_warshall_all_pairs(4, &edges),
        vec![
            vec![Some(0), Some(3), Some(5), Some(6)],
            vec![Some(7), Some(0), Some(2), Some(3)],
            vec![Some(5), Some(8), Some(0), Some(1)],
            vec![Some(4), Some(7), Some(9), Some(0)],
        ]
    );
}

#[test]
fn floyd_warshall_keeps_unreachable_pairs_as_none() {
    let edges = vec![(0, 1, 5), (2, 3, 7)];

    assert_eq!(
        floyd_warshall_all_pairs(4, &edges),
        vec![
            vec![Some(0), Some(5), None, None],
            vec![None, Some(0), None, None],
            vec![None, None, Some(0), Some(7)],
            vec![None, None, None, Some(0)],
        ]
    );
}

#[test]
fn prim_returns_minimum_spanning_tree_weight() {
    let edges = vec![(0, 1, 10), (0, 2, 6), (0, 3, 5), (1, 3, 15), (2, 3, 4)];

    assert_eq!(prim_minimum_spanning_tree_weight(4, &edges), Some(19));
}

#[test]
fn kruskal_returns_minimum_spanning_tree_weight() {
    let edges = vec![(0, 1, 10), (0, 2, 6), (0, 3, 5), (1, 3, 15), (2, 3, 4)];

    assert_eq!(kruskal_minimum_spanning_tree_weight(4, &edges), Some(19));
}

#[test]
fn minimum_spanning_tree_returns_none_for_disconnected_graph() {
    let edges = vec![(0, 1, 1), (2, 3, 1)];

    assert_eq!(prim_minimum_spanning_tree_weight(4, &edges), None);
    assert_eq!(kruskal_minimum_spanning_tree_weight(4, &edges), None);
}

#[test]
fn cheapest_flight_respects_stop_limit() {
    let flights = vec![(0, 1, 100), (1, 2, 100), (2, 3, 100), (0, 3, 500)];

    assert_eq!(
        cheapest_flight_within_k_stops(4, &flights, 0, 3, 1),
        Some(500)
    );
    assert_eq!(
        cheapest_flight_within_k_stops(4, &flights, 0, 3, 2),
        Some(300)
    );
}

#[test]
fn cheapest_flight_returns_none_when_destination_is_unreachable() {
    let flights = vec![(0, 1, 50), (1, 2, 50)];

    assert_eq!(cheapest_flight_within_k_stops(4, &flights, 0, 3, 2), None);
}

#[test]
fn minimum_effort_path_minimizes_largest_step() {
    let heights = vec![vec![1, 2, 2], vec![3, 8, 2], vec![5, 3, 5]];

    assert_eq!(minimum_effort_path(heights), 2);
}

#[test]
fn minimum_effort_path_returns_zero_for_single_cell() {
    assert_eq!(minimum_effort_path(vec![vec![7]]), 0);
}

#[test]
fn critical_connections_returns_network_bridges() {
    let connections = vec![(0, 1), (1, 2), (2, 0), (1, 3)];

    assert_eq!(critical_connections(4, &connections), vec![(1, 3)]);
}

#[test]
fn critical_connections_returns_empty_when_every_edge_is_in_a_cycle() {
    let connections = vec![(0, 1), (1, 2), (2, 0)];

    assert!(critical_connections(3, &connections).is_empty());
}
