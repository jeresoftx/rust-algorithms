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

pub fn floyd_warshall_all_pairs(
    node_count: usize,
    edges: &[(usize, usize, i32)],
) -> Vec<Vec<Option<i32>>> {
    let mut distances = vec![vec![i32::MAX; node_count]; node_count];

    for (node, row) in distances.iter_mut().enumerate() {
        row[node] = 0;
    }

    for &(from, to, weight) in edges {
        if from < node_count && to < node_count {
            distances[from][to] = distances[from][to].min(weight);
        }
    }

    for middle in 0..node_count {
        for from in 0..node_count {
            for to in 0..node_count {
                if distances[from][middle] == i32::MAX || distances[middle][to] == i32::MAX {
                    continue;
                }

                let candidate = distances[from][middle] + distances[middle][to];
                distances[from][to] = distances[from][to].min(candidate);
            }
        }
    }

    distances
        .into_iter()
        .map(|row| {
            row.into_iter()
                .map(|distance| (distance != i32::MAX).then_some(distance))
                .collect()
        })
        .collect()
}

pub fn prim_minimum_spanning_tree_weight(
    node_count: usize,
    edges: &[(usize, usize, i32)],
) -> Option<i32> {
    if node_count == 0 {
        return Some(0);
    }

    let graph = undirected_adjacency_list(node_count, edges);
    let mut visited = vec![false; node_count];
    let mut heap = BinaryHeap::new();
    let mut total_weight = 0;
    let mut visited_count = 0;

    heap.push(Reverse((0, 0)));

    while let Some(Reverse((weight, node))) = heap.pop() {
        if visited[node] {
            continue;
        }

        visited[node] = true;
        visited_count += 1;
        total_weight += weight;

        for &(neighbor, edge_weight) in &graph[node] {
            if !visited[neighbor] {
                heap.push(Reverse((edge_weight, neighbor)));
            }
        }
    }

    (visited_count == node_count).then_some(total_weight)
}

pub fn kruskal_minimum_spanning_tree_weight(
    node_count: usize,
    edges: &[(usize, usize, i32)],
) -> Option<i32> {
    if node_count == 0 {
        return Some(0);
    }

    let mut sorted_edges: Vec<(usize, usize, i32)> = edges
        .iter()
        .copied()
        .filter(|&(from, to, _)| from < node_count && to < node_count)
        .collect();
    sorted_edges.sort_by_key(|&(_, _, weight)| weight);

    let mut union_find = UnionFind::new(node_count);
    let mut total_weight = 0;
    let mut edges_used = 0;

    for (from, to, weight) in sorted_edges {
        if union_find.union(from, to) {
            total_weight += weight;
            edges_used += 1;
        }
    }

    (edges_used == node_count - 1).then_some(total_weight)
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

fn undirected_adjacency_list(
    node_count: usize,
    edges: &[(usize, usize, i32)],
) -> Vec<Vec<(usize, i32)>> {
    let mut graph = vec![Vec::new(); node_count];

    for &(from, to, weight) in edges {
        if from < node_count && to < node_count {
            graph[from].push((to, weight));
            graph[to].push((from, weight));
        }
    }

    graph
}

struct UnionFind {
    parent: Vec<usize>,
    rank: Vec<usize>,
}

impl UnionFind {
    fn new(size: usize) -> Self {
        Self {
            parent: (0..size).collect(),
            rank: vec![0; size],
        }
    }

    fn find(&mut self, value: usize) -> usize {
        if self.parent[value] != value {
            self.parent[value] = self.find(self.parent[value]);
        }

        self.parent[value]
    }

    fn union(&mut self, left: usize, right: usize) -> bool {
        let left_root = self.find(left);
        let right_root = self.find(right);

        if left_root == right_root {
            return false;
        }

        if self.rank[left_root] < self.rank[right_root] {
            self.parent[left_root] = right_root;
        } else if self.rank[left_root] > self.rank[right_root] {
            self.parent[right_root] = left_root;
        } else {
            self.parent[right_root] = left_root;
            self.rank[left_root] += 1;
        }

        true
    }
}
