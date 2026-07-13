use std::collections::{BTreeSet, HashMap, HashSet, VecDeque};

const DIRECTIONS: [(isize, isize); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];

/// Number of Islands
///
/// Pattern: DFS in a matrix.
/// Idea: every unvisited land cell starts one connected component.
///
/// Time: O(m * n)
/// Space: O(m * n)
pub fn number_of_islands(grid: Vec<Vec<char>>) -> i32 {
    if grid.is_empty() || grid[0].is_empty() {
        return 0;
    }

    let rows = grid.len();
    let cols = grid[0].len();
    let mut visited = vec![vec![false; cols]; rows];
    let mut islands = 0;

    for row in 0..rows {
        for col in 0..cols {
            if grid[row][col] == '1' && !visited[row][col] {
                islands += 1;
                mark_island(&grid, &mut visited, row, col);
            }
        }
    }

    islands
}

fn mark_island(grid: &[Vec<char>], visited: &mut [Vec<bool>], row: usize, col: usize) {
    if visited[row][col] || grid[row][col] != '1' {
        return;
    }

    visited[row][col] = true;

    for (next_row, next_col) in neighbors(row, col, grid.len(), grid[0].len()) {
        mark_island(grid, visited, next_row, next_col);
    }
}

/// Max Area of Island
///
/// Pattern: DFS in a matrix.
/// Idea: each DFS returns the size of the component it consumes.
///
/// Time: O(m * n)
/// Space: O(m * n)
pub fn max_area_of_island(grid: Vec<Vec<i32>>) -> i32 {
    if grid.is_empty() || grid[0].is_empty() {
        return 0;
    }

    let rows = grid.len();
    let cols = grid[0].len();
    let mut visited = vec![vec![false; cols]; rows];
    let mut best = 0;

    for row in 0..rows {
        for col in 0..cols {
            if grid[row][col] == 1 && !visited[row][col] {
                best = best.max(island_area(&grid, &mut visited, row, col));
            }
        }
    }

    best
}

fn island_area(grid: &[Vec<i32>], visited: &mut [Vec<bool>], row: usize, col: usize) -> i32 {
    if visited[row][col] || grid[row][col] == 0 {
        return 0;
    }

    visited[row][col] = true;
    let mut area = 1;

    for (next_row, next_col) in neighbors(row, col, grid.len(), grid[0].len()) {
        area += island_area(grid, visited, next_row, next_col);
    }

    area
}

/// Pacific Atlantic Water Flow
///
/// Pattern: reverse graph search from borders.
/// Idea: start from each ocean and move to neighbors with equal or greater height.
///
/// Time: O(m * n)
/// Space: O(m * n)
pub fn pacific_atlantic(heights: Vec<Vec<i32>>) -> Vec<(usize, usize)> {
    if heights.is_empty() || heights[0].is_empty() {
        return Vec::new();
    }

    let rows = heights.len();
    let cols = heights[0].len();
    let mut pacific_starts = Vec::with_capacity(rows + cols);
    let mut atlantic_starts = Vec::with_capacity(rows + cols);

    for row in 0..rows {
        pacific_starts.push((row, 0));
        atlantic_starts.push((row, cols - 1));
    }

    for col in 0..cols {
        pacific_starts.push((0, col));
        atlantic_starts.push((rows - 1, col));
    }

    let pacific = ocean_reachable(&heights, pacific_starts);
    let atlantic = ocean_reachable(&heights, atlantic_starts);
    let mut result = Vec::new();

    for row in 0..rows {
        for col in 0..cols {
            if pacific[row][col] && atlantic[row][col] {
                result.push((row, col));
            }
        }
    }

    result
}

fn ocean_reachable(heights: &[Vec<i32>], starts: Vec<(usize, usize)>) -> Vec<Vec<bool>> {
    let rows = heights.len();
    let cols = heights[0].len();
    let mut reachable = vec![vec![false; cols]; rows];
    let mut queue = VecDeque::new();

    for (row, col) in starts {
        if !reachable[row][col] {
            reachable[row][col] = true;
            queue.push_back((row, col));
        }
    }

    while let Some((row, col)) = queue.pop_front() {
        for (next_row, next_col) in neighbors(row, col, rows, cols) {
            if !reachable[next_row][next_col] && heights[next_row][next_col] >= heights[row][col] {
                reachable[next_row][next_col] = true;
                queue.push_back((next_row, next_col));
            }
        }
    }

    reachable
}

/// Rotting Oranges
///
/// Pattern: multi-source BFS.
/// Idea: all rotten oranges spread at the same time, one BFS layer per minute.
///
/// Time: O(m * n)
/// Space: O(m * n)
pub fn oranges_rotting(mut grid: Vec<Vec<i32>>) -> i32 {
    if grid.is_empty() || grid[0].is_empty() {
        return 0;
    }

    let rows = grid.len();
    let cols = grid[0].len();
    let mut fresh = 0;
    let mut queue = VecDeque::new();

    for row in 0..rows {
        for col in 0..cols {
            match grid[row][col] {
                1 => fresh += 1,
                2 => queue.push_back((row, col, 0)),
                _ => {}
            }
        }
    }

    let mut minutes = 0;

    while let Some((row, col, minute)) = queue.pop_front() {
        minutes = minutes.max(minute);

        for (next_row, next_col) in neighbors(row, col, rows, cols) {
            if grid[next_row][next_col] == 1 {
                grid[next_row][next_col] = 2;
                fresh -= 1;
                queue.push_back((next_row, next_col, minute + 1));
            }
        }
    }

    if fresh == 0 {
        minutes
    } else {
        -1
    }
}

/// Walls and Gates
///
/// Pattern: multi-source BFS.
/// Idea: expand from every gate so the first visit gives the nearest distance.
///
/// Time: O(m * n)
/// Space: O(m * n)
pub fn walls_and_gates(rooms: &mut Vec<Vec<i32>>) {
    if rooms.is_empty() || rooms[0].is_empty() {
        return;
    }

    let rows = rooms.len();
    let cols = rooms[0].len();
    let mut queue = VecDeque::new();

    for row in 0..rows {
        for col in 0..cols {
            if rooms[row][col] == 0 {
                queue.push_back((row, col));
            }
        }
    }

    while let Some((row, col)) = queue.pop_front() {
        for (next_row, next_col) in neighbors(row, col, rows, cols) {
            if rooms[next_row][next_col] == i32::MAX {
                rooms[next_row][next_col] = rooms[row][col] + 1;
                queue.push_back((next_row, next_col));
            }
        }
    }
}

/// Course Schedule
///
/// Pattern: topological sort with indegrees.
/// Idea: all courses with indegree zero can be taken immediately.
///
/// Time: O(v + e)
/// Space: O(v + e)
pub fn can_finish(num_courses: usize, prerequisites: Vec<(usize, usize)>) -> bool {
    find_course_order(num_courses, prerequisites).len() == num_courses
}

/// Course Schedule II
///
/// Pattern: topological sort with Kahn's algorithm.
/// Idea: repeatedly remove nodes with no remaining prerequisites.
///
/// Time: O(v + e)
/// Space: O(v + e)
pub fn find_course_order(num_courses: usize, prerequisites: Vec<(usize, usize)>) -> Vec<usize> {
    let mut graph = vec![Vec::new(); num_courses];
    let mut indegrees = vec![0; num_courses];

    for (course, prerequisite) in prerequisites {
        graph[prerequisite].push(course);
        indegrees[course] += 1;
    }

    let mut queue = VecDeque::new();
    for (course, &indegree) in indegrees.iter().enumerate() {
        if indegree == 0 {
            queue.push_back(course);
        }
    }

    let mut order = Vec::with_capacity(num_courses);

    while let Some(course) = queue.pop_front() {
        order.push(course);

        for &next_course in &graph[course] {
            indegrees[next_course] -= 1;
            if indegrees[next_course] == 0 {
                queue.push_back(next_course);
            }
        }
    }

    if order.len() == num_courses {
        order
    } else {
        Vec::new()
    }
}

/// Redundant Connection
///
/// Pattern: Union Find.
/// Idea: an edge is redundant when both endpoints are already connected.
///
/// Time: O(e * alpha(v))
/// Space: O(v)
pub fn redundant_connection(edges: Vec<(usize, usize)>) -> Option<(usize, usize)> {
    let max_node = edges
        .iter()
        .flat_map(|&(left, right)| [left, right])
        .max()
        .unwrap_or(0);
    let mut union_find = UnionFind::new(max_node + 1);

    for (left, right) in edges {
        if !union_find.union(left, right) {
            return Some((left, right));
        }
    }

    None
}

/// Number of Connected Components in an Undirected Graph
///
/// Pattern: Union-Find.
/// Idea: start with every node as its own component and merge endpoints.
///
/// Time: O((v + e) * alpha(v))
/// Space: O(v)
pub fn count_connected_components(node_count: usize, edges: Vec<(usize, usize)>) -> i32 {
    let mut union_find = UnionFind::new(node_count);
    let mut components = node_count as i32;

    for (left, right) in edges {
        if left < node_count && right < node_count && union_find.union(left, right) {
            components -= 1;
        }
    }

    components
}

/// Graph Valid Tree
///
/// Pattern: Union-Find.
/// Idea: a tree with n nodes must have n - 1 edges, be connected and have no cycles.
///
/// Time: O(e * alpha(v))
/// Space: O(v)
pub fn graph_valid_tree(node_count: usize, edges: Vec<(usize, usize)>) -> bool {
    if node_count == 0 || edges.len() != node_count - 1 {
        return false;
    }

    let mut union_find = UnionFind::new(node_count);

    for (left, right) in edges {
        if left >= node_count || right >= node_count || !union_find.union(left, right) {
            return false;
        }
    }

    true
}

/// Is Graph Bipartite?
///
/// Pattern: BFS coloring.
/// Idea: every edge must connect nodes with opposite colors.
///
/// Time: O(v + e)
/// Space: O(v)
pub fn is_bipartite(graph: Vec<Vec<usize>>) -> bool {
    let mut colors: Vec<Option<bool>> = vec![None; graph.len()];

    for start in 0..graph.len() {
        if colors[start].is_some() {
            continue;
        }

        colors[start] = Some(false);
        let mut queue = VecDeque::from([start]);

        while let Some(node) = queue.pop_front() {
            let current_color = colors[node].expect("queued nodes are colored");

            for &neighbor in &graph[node] {
                if neighbor >= graph.len() {
                    return false;
                }

                match colors[neighbor] {
                    Some(neighbor_color) if neighbor_color == current_color => return false,
                    Some(_) => {}
                    None => {
                        colors[neighbor] = Some(!current_color);
                        queue.push_back(neighbor);
                    }
                }
            }
        }
    }

    true
}

/// Number of Provinces
///
/// Pattern: Union-Find over an adjacency matrix.
/// Idea: every direct connection merges two cities into one province.
///
/// Time: O(n^2 * alpha(n))
/// Space: O(n)
pub fn find_circle_num(is_connected: Vec<Vec<i32>>) -> i32 {
    let city_count = is_connected.len();
    let mut union_find = UnionFind::new(city_count);
    let mut provinces = city_count as i32;

    for row in 0..city_count {
        for col in row + 1..city_count {
            if is_connected[row].get(col) == Some(&1) && union_find.union(row, col) {
                provinces -= 1;
            }
        }
    }

    provinces
}

/// Possible Bipartition
///
/// Pattern: BFS coloring over a 1-indexed graph.
/// Idea: people who dislike each other must receive opposite colors.
///
/// Time: O(n + e)
/// Space: O(n + e)
pub fn possible_bipartition(person_count: usize, dislikes: Vec<(usize, usize)>) -> bool {
    let mut graph = vec![Vec::new(); person_count + 1];

    for (left, right) in dislikes {
        if left == 0 || right == 0 || left > person_count || right > person_count {
            return false;
        }

        graph[left].push(right);
        graph[right].push(left);
    }

    let mut colors: Vec<Option<bool>> = vec![None; person_count + 1];

    for person in 1..=person_count {
        if colors[person].is_some() {
            continue;
        }

        colors[person] = Some(false);
        let mut queue = VecDeque::from([person]);

        while let Some(current) = queue.pop_front() {
            let current_color = colors[current].expect("queued people are colored");

            for &neighbor in &graph[current] {
                match colors[neighbor] {
                    Some(neighbor_color) if neighbor_color == current_color => return false,
                    Some(_) => {}
                    None => {
                        colors[neighbor] = Some(!current_color);
                        queue.push_back(neighbor);
                    }
                }
            }
        }
    }

    true
}

/// Evaluate Division
///
/// Pattern: weighted graph traversal.
/// Idea: each equation creates two directed edges with reciprocal weights.
///
/// Time: O(q * (v + e))
/// Space: O(v + e)
pub fn evaluate_division(
    equations: Vec<(&str, &str, f64)>,
    queries: Vec<(&str, &str)>,
) -> Vec<Option<f64>> {
    let mut graph: HashMap<String, Vec<(String, f64)>> = HashMap::new();

    for (left, right, value) in equations {
        graph
            .entry(left.to_string())
            .or_default()
            .push((right.to_string(), value));
        graph
            .entry(right.to_string())
            .or_default()
            .push((left.to_string(), 1.0 / value));
    }

    queries
        .into_iter()
        .map(|(start, target)| evaluate_ratio(&graph, start, target))
        .collect()
}

fn evaluate_ratio(
    graph: &HashMap<String, Vec<(String, f64)>>,
    start: &str,
    target: &str,
) -> Option<f64> {
    if !graph.contains_key(start) || !graph.contains_key(target) {
        return None;
    }

    if start == target {
        return Some(1.0);
    }

    let mut visited = HashSet::from([start.to_string()]);
    let mut queue = VecDeque::from([(start.to_string(), 1.0)]);

    while let Some((current, product)) = queue.pop_front() {
        if let Some(neighbors) = graph.get(&current) {
            for (neighbor, weight) in neighbors {
                if !visited.insert(neighbor.clone()) {
                    continue;
                }

                let next_product = product * weight;
                if neighbor == target {
                    return Some(next_product);
                }

                queue.push_back((neighbor.clone(), next_product));
            }
        }
    }

    None
}

/// Alien Dictionary
///
/// Pattern: topological sort over characters.
/// Idea: the first differing character between adjacent words defines an edge.
///
/// Time: O(total characters + edges)
/// Space: O(unique characters + edges)
pub fn alien_order(words: Vec<&str>) -> Option<String> {
    let mut graph: HashMap<char, BTreeSet<char>> = HashMap::new();
    let mut indegrees: HashMap<char, usize> = HashMap::new();

    for word in &words {
        for character in word.chars() {
            graph.entry(character).or_default();
            indegrees.entry(character).or_insert(0);
        }
    }

    for pair in words.windows(2) {
        let first: Vec<char> = pair[0].chars().collect();
        let second: Vec<char> = pair[1].chars().collect();

        if first.len() > second.len() && first.starts_with(&second) {
            return None;
        }

        if let Some((left, right)) = first
            .iter()
            .zip(second.iter())
            .find_map(|(&left, &right)| (left != right).then_some((left, right)))
        {
            let neighbors = graph.entry(left).or_default();
            if neighbors.insert(right) {
                *indegrees.entry(right).or_insert(0) += 1;
            }
        }
    }

    let mut ready: BTreeSet<char> = indegrees
        .iter()
        .filter_map(|(&character, &indegree)| (indegree == 0).then_some(character))
        .collect();
    let mut order = String::new();

    while let Some(&character) = ready.iter().next() {
        ready.remove(&character);
        order.push(character);

        if let Some(neighbors) = graph.get(&character) {
            for &neighbor in neighbors {
                let indegree = indegrees
                    .get_mut(&neighbor)
                    .expect("every neighbor has an indegree");
                *indegree -= 1;

                if *indegree == 0 {
                    ready.insert(neighbor);
                }
            }
        }
    }

    (order.chars().count() == indegrees.len()).then_some(order)
}

/// Accounts Merge
///
/// Pattern: connected components in an email graph.
/// Idea: emails in the same account belong to the same component.
///
/// Time: O(n * k log k)
/// Space: O(n * k)
pub fn accounts_merge(accounts: Vec<Vec<String>>) -> Vec<Vec<String>> {
    let mut graph: HashMap<String, Vec<String>> = HashMap::new();
    let mut email_to_name: HashMap<String, String> = HashMap::new();

    for account in accounts {
        let Some(name) = account.first() else {
            continue;
        };
        let emails = &account[1..];

        for email in emails {
            email_to_name.insert(email.clone(), name.clone());
            graph.entry(email.clone()).or_default();
        }

        if let Some(first_email) = emails.first() {
            for email in emails.iter().skip(1) {
                graph
                    .entry(first_email.clone())
                    .or_default()
                    .push(email.clone());
                graph
                    .entry(email.clone())
                    .or_default()
                    .push(first_email.clone());
            }
        }
    }

    let mut visited = HashSet::new();
    let mut merged = Vec::new();

    for email in graph.keys() {
        if visited.contains(email) {
            continue;
        }

        let mut component = Vec::new();
        let mut stack = vec![email.clone()];
        visited.insert(email.clone());

        while let Some(current) = stack.pop() {
            component.push(current.clone());

            if let Some(neighbors) = graph.get(&current) {
                for neighbor in neighbors {
                    if visited.insert(neighbor.clone()) {
                        stack.push(neighbor.clone());
                    }
                }
            }
        }

        component.sort();
        let name = email_to_name
            .get(&component[0])
            .expect("every email has an owner")
            .clone();
        let mut account = Vec::with_capacity(component.len() + 1);
        account.push(name);
        account.extend(component);
        merged.push(account);
    }

    merged
}

/// Clone Graph
///
/// Pattern: graph traversal over an adjacency list.
/// Idea: visit every node and rebuild its neighbor list in a separate vector.
///
/// Time: O(v + e)
/// Space: O(v + e)
pub fn clone_graph(adjacency: Vec<Vec<usize>>) -> Vec<Vec<usize>> {
    let mut clone = vec![Vec::new(); adjacency.len()];
    let mut visited = vec![false; adjacency.len()];

    for node in 0..adjacency.len() {
        if visited[node] {
            continue;
        }

        let mut queue = VecDeque::from([node]);
        visited[node] = true;

        while let Some(current) = queue.pop_front() {
            clone[current] = adjacency[current].clone();

            for &neighbor in &adjacency[current] {
                if !visited[neighbor] {
                    visited[neighbor] = true;
                    queue.push_back(neighbor);
                }
            }
        }
    }

    clone
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

        match self.rank[left_root].cmp(&self.rank[right_root]) {
            std::cmp::Ordering::Less => self.parent[left_root] = right_root,
            std::cmp::Ordering::Greater => self.parent[right_root] = left_root,
            std::cmp::Ordering::Equal => {
                self.parent[right_root] = left_root;
                self.rank[left_root] += 1;
            }
        }

        true
    }
}

fn neighbors(row: usize, col: usize, rows: usize, cols: usize) -> Vec<(usize, usize)> {
    let mut result = Vec::with_capacity(4);

    for (row_delta, col_delta) in DIRECTIONS {
        let next_row = row as isize + row_delta;
        let next_col = col as isize + col_delta;

        if next_row >= 0 && next_row < rows as isize && next_col >= 0 && next_col < cols as isize {
            result.push((next_row as usize, next_col as usize));
        }
    }

    result
}
