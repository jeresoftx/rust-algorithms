use std::cmp::Reverse;
use std::collections::BinaryHeap;

const GRID_DIRECTIONS: [(isize, isize); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];

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

pub fn cheapest_flight_within_k_stops(
    node_count: usize,
    flights: &[(usize, usize, i32)],
    source: usize,
    destination: usize,
    max_stops: usize,
) -> Option<i32> {
    if source >= node_count || destination >= node_count {
        return None;
    }

    if source == destination {
        return Some(0);
    }

    let mut distances = vec![i32::MAX; node_count];
    distances[source] = 0;

    for _ in 0..=max_stops {
        let mut next_distances = distances.clone();

        for &(from, to, price) in flights {
            if from >= node_count || to >= node_count || distances[from] == i32::MAX {
                continue;
            }

            let candidate = distances[from].saturating_add(price);
            next_distances[to] = next_distances[to].min(candidate);
        }

        distances = next_distances;
    }

    (distances[destination] != i32::MAX).then_some(distances[destination])
}

pub fn minimum_effort_path(heights: Vec<Vec<i32>>) -> i32 {
    if heights.is_empty() || heights[0].is_empty() {
        return 0;
    }

    let rows = heights.len();
    let cols = heights[0].len();
    let mut efforts = vec![vec![i32::MAX; cols]; rows];
    let mut heap = BinaryHeap::new();

    efforts[0][0] = 0;
    heap.push(Reverse((0, 0, 0)));

    while let Some(Reverse((effort, row, col))) = heap.pop() {
        if row == rows - 1 && col == cols - 1 {
            return effort;
        }

        if effort > efforts[row][col] {
            continue;
        }

        for (next_row, next_col) in grid_neighbors(row, col, rows, cols) {
            let step_effort = (heights[row][col] - heights[next_row][next_col]).abs();
            let next_effort = effort.max(step_effort);

            if next_effort < efforts[next_row][next_col] {
                efforts[next_row][next_col] = next_effort;
                heap.push(Reverse((next_effort, next_row, next_col)));
            }
        }
    }

    0
}

pub fn critical_connections(
    node_count: usize,
    connections: &[(usize, usize)],
) -> Vec<(usize, usize)> {
    let mut graph = vec![Vec::new(); node_count];

    for &(left, right) in connections {
        if left < node_count && right < node_count {
            graph[left].push(right);
            graph[right].push(left);
        }
    }

    let mut discovery = vec![None; node_count];
    let mut low = vec![0; node_count];
    let mut time = 0;
    let mut bridges = Vec::new();

    for node in 0..node_count {
        if discovery[node].is_none() {
            collect_bridges(
                node,
                usize::MAX,
                &graph,
                &mut discovery,
                &mut low,
                &mut time,
                &mut bridges,
            );
        }
    }

    bridges.sort_unstable();
    bridges
}

pub fn min_cost_connect_points(points: &[(i32, i32)]) -> i32 {
    if points.len() <= 1 {
        return 0;
    }

    let mut in_tree = vec![false; points.len()];
    let mut min_costs = vec![i32::MAX; points.len()];
    let mut total_cost = 0;

    min_costs[0] = 0;

    for _ in 0..points.len() {
        let mut current = None;

        for node in 0..points.len() {
            if !in_tree[node] && current.is_none_or(|best| min_costs[node] < min_costs[best]) {
                current = Some(node);
            }
        }

        let node = current.unwrap();
        in_tree[node] = true;
        total_cost += min_costs[node];

        for neighbor in 0..points.len() {
            if !in_tree[neighbor] {
                let cost = manhattan_distance(points[node], points[neighbor]);
                min_costs[neighbor] = min_costs[neighbor].min(cost);
            }
        }
    }

    total_cost
}

pub fn find_critical_and_pseudo_critical_edges(
    node_count: usize,
    edges: &[(usize, usize, i32)],
) -> (Vec<usize>, Vec<usize>) {
    let mut indexed_edges: Vec<(usize, usize, i32, usize)> = edges
        .iter()
        .copied()
        .enumerate()
        .filter_map(|(index, (from, to, weight))| {
            (from < node_count && to < node_count).then_some((from, to, weight, index))
        })
        .collect();
    indexed_edges.sort_by_key(|&(_, _, weight, _)| weight);

    let Some(base_weight) = constrained_mst_weight(node_count, &indexed_edges, None, None) else {
        return (Vec::new(), Vec::new());
    };

    let mut critical = Vec::new();
    let mut pseudo_critical = Vec::new();

    for &(_, _, _, index) in &indexed_edges {
        let without_edge = constrained_mst_weight(node_count, &indexed_edges, None, Some(index));

        if without_edge.is_none_or(|weight| weight > base_weight) {
            critical.push(index);
            continue;
        }

        if constrained_mst_weight(node_count, &indexed_edges, Some(index), None)
            == Some(base_weight)
        {
            pseudo_critical.push(index);
        }
    }

    critical.sort_unstable();
    pseudo_critical.sort_unstable();
    (critical, pseudo_critical)
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

fn constrained_mst_weight(
    node_count: usize,
    edges: &[(usize, usize, i32, usize)],
    forced_edge: Option<usize>,
    banned_edge: Option<usize>,
) -> Option<i32> {
    if node_count <= 1 {
        return Some(0);
    }

    let mut union_find = UnionFind::new(node_count);
    let mut total_weight = 0;
    let mut edges_used = 0;

    if let Some(forced_index) = forced_edge {
        let &(from, to, weight, _) = edges
            .iter()
            .find(|&&(_, _, _, index)| index == forced_index)?;

        if union_find.union(from, to) {
            total_weight += weight;
            edges_used += 1;
        }
    }

    for &(from, to, weight, index) in edges {
        if Some(index) == banned_edge || Some(index) == forced_edge {
            continue;
        }

        if union_find.union(from, to) {
            total_weight += weight;
            edges_used += 1;
        }
    }

    (edges_used == node_count - 1).then_some(total_weight)
}

fn manhattan_distance(left: (i32, i32), right: (i32, i32)) -> i32 {
    (left.0 - right.0).abs() + (left.1 - right.1).abs()
}

fn grid_neighbors(row: usize, col: usize, rows: usize, cols: usize) -> Vec<(usize, usize)> {
    let mut result = Vec::with_capacity(4);

    for (row_delta, col_delta) in GRID_DIRECTIONS {
        let next_row = row as isize + row_delta;
        let next_col = col as isize + col_delta;

        if next_row >= 0 && next_row < rows as isize && next_col >= 0 && next_col < cols as isize {
            result.push((next_row as usize, next_col as usize));
        }
    }

    result
}

fn collect_bridges(
    node: usize,
    parent: usize,
    graph: &[Vec<usize>],
    discovery: &mut [Option<usize>],
    low: &mut [usize],
    time: &mut usize,
    bridges: &mut Vec<(usize, usize)>,
) {
    *time += 1;
    discovery[node] = Some(*time);
    low[node] = *time;

    for &neighbor in &graph[node] {
        if neighbor == parent {
            continue;
        }

        if discovery[neighbor].is_none() {
            collect_bridges(neighbor, node, graph, discovery, low, time, bridges);
            low[node] = low[node].min(low[neighbor]);

            if low[neighbor] > discovery[node].unwrap() {
                bridges.push((node.min(neighbor), node.max(neighbor)));
            }
        } else {
            low[node] = low[node].min(discovery[neighbor].unwrap());
        }
    }
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
