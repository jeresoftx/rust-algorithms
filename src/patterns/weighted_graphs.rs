use std::cmp::Reverse;
use std::collections::BinaryHeap;

#[derive(Debug, PartialEq, Eq)]
pub enum BellmanFordError {
    NegativeCycle,
}

pub fn dijkstra_shortest_paths(
    node_count: usize,
    edges: &[(usize, usize, i32)],
    start: usize,
) -> Vec<Option<i32>> {
    if start >= node_count {
        return Vec::new();
    }

    let graph = adjacency_list(node_count, edges);
    let mut distances = vec![i32::MAX; node_count];
    let mut heap = BinaryHeap::new();

    distances[start] = 0;
    heap.push(Reverse((0, start)));

    while let Some(Reverse((distance, node))) = heap.pop() {
        if distance > distances[node] {
            continue;
        }

        for &(neighbor, weight) in &graph[node] {
            let next_distance = distance + weight;

            if next_distance < distances[neighbor] {
                distances[neighbor] = next_distance;
                heap.push(Reverse((next_distance, neighbor)));
            }
        }
    }

    distances
        .into_iter()
        .map(|distance| (distance != i32::MAX).then_some(distance))
        .collect()
}

pub fn network_delay_time(times: Vec<(usize, usize, i32)>, node_count: usize, start: usize) -> i32 {
    if start == 0 || start > node_count {
        return -1;
    }

    let zero_indexed_edges: Vec<(usize, usize, i32)> = times
        .into_iter()
        .filter(|&(from, to, _)| from > 0 && from <= node_count && to > 0 && to <= node_count)
        .map(|(from, to, weight)| (from - 1, to - 1, weight))
        .collect();

    let distances = dijkstra_shortest_paths(node_count, &zero_indexed_edges, start - 1);
    let mut delay = 0;

    for distance in distances {
        let Some(distance) = distance else {
            return -1;
        };

        delay = delay.max(distance);
    }

    delay
}

pub fn bellman_ford_shortest_paths(
    node_count: usize,
    edges: &[(usize, usize, i32)],
    start: usize,
) -> Result<Vec<Option<i32>>, BellmanFordError> {
    if start >= node_count {
        return Ok(Vec::new());
    }

    let mut distances = vec![i32::MAX; node_count];
    distances[start] = 0;

    for _ in 0..node_count.saturating_sub(1) {
        let mut changed = false;

        for &(from, to, weight) in edges {
            if from >= node_count || to >= node_count || distances[from] == i32::MAX {
                continue;
            }

            let next_distance = distances[from] + weight;

            if next_distance < distances[to] {
                distances[to] = next_distance;
                changed = true;
            }
        }

        if !changed {
            break;
        }
    }

    for &(from, to, weight) in edges {
        if from >= node_count || to >= node_count || distances[from] == i32::MAX {
            continue;
        }

        if distances[from] + weight < distances[to] {
            return Err(BellmanFordError::NegativeCycle);
        }
    }

    Ok(distances
        .into_iter()
        .map(|distance| (distance != i32::MAX).then_some(distance))
        .collect())
}

fn adjacency_list(node_count: usize, edges: &[(usize, usize, i32)]) -> Vec<Vec<(usize, i32)>> {
    let mut graph = vec![Vec::new(); node_count];

    for &(from, to, weight) in edges {
        if from < node_count && to < node_count {
            graph[from].push((to, weight));
        }
    }

    graph
}
