use rust_algorithms::patterns::weighted_graphs::{
    bellman_ford_shortest_paths, dijkstra_shortest_paths, network_delay_time, BellmanFordError,
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
