//! Range-query and range-update data structures.
//!
//! # Example
//!
//! ```
//! use rust_algorithms::patterns::range_queries::FenwickTree;
//!
//! let tree = FenwickTree::from_values(&[2, 4, 6]);
//! assert_eq!(tree.range_sum(1, 2), Some(10));
//! ```

use std::collections::{BTreeMap, VecDeque};

/// Find Pivot Index
///
/// Pattern: prefix sum.
/// Idea: keep the sum to the left while deriving the right sum from the total,
/// so each index is checked in one pass without an auxiliary prefix array.
///
/// Time: O(n)
/// Space: O(1)
pub fn pivot_index(values: &[i32]) -> Option<usize> {
    let total: i64 = values.iter().map(|&value| i64::from(value)).sum();
    let mut left_sum = 0_i64;

    for (index, &value) in values.iter().enumerate() {
        let right_sum = total - left_sum - i64::from(value);

        if left_sum == right_sum {
            return Some(index);
        }

        left_sum += i64::from(value);
    }

    None
}

/// Contiguous Array
///
/// Pattern: prefix balance with first-seen positions.
/// Idea: treat zero as -1 and one as +1. Equal balances delimit a subarray
/// containing the same number of zeros and ones.
///
/// Time: O(n)
/// Space: O(n)
pub fn contiguous_array(values: &[i32]) -> usize {
    let mut first_seen = BTreeMap::from([(0_i32, -1_i64)]);
    let mut balance = 0_i32;
    let mut longest = 0_usize;

    for (index, &value) in values.iter().enumerate() {
        balance += if value == 0 { -1 } else { 1 };
        let index = index as i64;

        if let Some(&first_index) = first_seen.get(&balance) {
            longest = longest.max((index - first_index) as usize);
        } else {
            first_seen.insert(balance, index);
        }
    }

    longest
}

/// Shortest Subarray with Sum at Least K
///
/// Pattern: prefix sums with a monotonic deque.
/// Idea: negative values prevent ordinary window contraction. Increasing prefix
/// sums in the deque retain only starts that can still produce a shorter window.
///
/// Time: O(n)
/// Space: O(n)
pub fn shortest_subarray_at_least_k(values: &[i32], target: i32) -> Option<usize> {
    let mut prefixes = Vec::with_capacity(values.len() + 1);
    prefixes.push(0_i64);

    for &value in values {
        prefixes.push(prefixes.last().copied().unwrap_or(0) + i64::from(value));
    }

    let target = i64::from(target);
    let mut candidates = VecDeque::new();
    let mut shortest = usize::MAX;

    for (index, &prefix) in prefixes.iter().enumerate() {
        while let Some(&start) = candidates.front() {
            if prefix - prefixes[start] < target {
                break;
            }

            shortest = shortest.min(index - start);
            candidates.pop_front();
        }

        while let Some(&last) = candidates.back() {
            if prefixes[last] < prefix {
                break;
            }

            candidates.pop_back();
        }

        candidates.push_back(index);
    }

    (shortest != usize::MAX).then_some(shortest)
}

/// Fenwick tree for prefix and range sums over point updates.
///
/// Time:
/// - `add`: O(log n)
/// - `prefix_sum`: O(log n)
/// - `range_sum`: O(log n)
///
/// Space: O(n)
#[derive(Debug, Clone)]
pub struct FenwickTree {
    tree: Vec<i32>,
}

impl FenwickTree {
    /// Creates an empty tree with `size` logical positions.
    pub fn new(size: usize) -> Self {
        Self {
            tree: vec![0; size + 1],
        }
    }

    /// Builds a Fenwick tree from initial values.
    pub fn from_values(values: &[i32]) -> Self {
        let mut tree = Self::new(values.len());

        for (index, &value) in values.iter().enumerate() {
            tree.add(index, value);
        }

        tree
    }

    /// Adds `delta` to one position.
    pub fn add(&mut self, index: usize, delta: i32) -> bool {
        if index >= self.len() {
            return false;
        }

        let mut current = index + 1;

        while current < self.tree.len() {
            self.tree[current] += delta;
            current += lowbit(current);
        }

        true
    }

    /// Returns the sum from index `0` through `index`.
    pub fn prefix_sum(&self, index: usize) -> Option<i32> {
        if index >= self.len() {
            return None;
        }

        let mut current = index + 1;
        let mut total = 0;

        while current > 0 {
            total += self.tree[current];
            current -= lowbit(current);
        }

        Some(total)
    }

    /// Returns the inclusive range sum from `left` through `right`.
    pub fn range_sum(&self, left: usize, right: usize) -> Option<i32> {
        if left > right || right >= self.len() {
            return None;
        }

        let right_sum = self.prefix_sum(right)?;
        let left_prefix = if left == 0 {
            0
        } else {
            self.prefix_sum(left - 1)?
        };

        Some(right_sum - left_prefix)
    }

    /// Returns the number of logical positions.
    pub fn len(&self) -> usize {
        self.tree.len().saturating_sub(1)
    }

    /// Returns whether the tree has no logical positions.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

fn lowbit(value: usize) -> usize {
    value & value.wrapping_neg()
}

/// Mutable range-sum query backed by a Fenwick tree.
///
/// Time:
/// - `update`: O(log n)
/// - `sum_range`: O(log n)
///
/// Space: O(n)
#[derive(Debug, Clone)]
pub struct RangeSumQuery {
    values: Vec<i32>,
    tree: FenwickTree,
}

impl RangeSumQuery {
    /// Builds the structure from initial values.
    pub fn new(values: Vec<i32>) -> Self {
        Self {
            tree: FenwickTree::from_values(&values),
            values,
        }
    }

    /// Replaces one value.
    pub fn update(&mut self, index: usize, value: i32) -> bool {
        let Some(current) = self.values.get_mut(index) else {
            return false;
        };

        let delta = value - *current;
        *current = value;
        self.tree.add(index, delta)
    }

    /// Returns the inclusive range sum from `left` through `right`.
    pub fn sum_range(&self, left: usize, right: usize) -> Option<i32> {
        self.tree.range_sum(left, right)
    }
}

/// Immutable two-dimensional range-sum query.
#[derive(Debug, Clone)]
pub struct RangeSumQuery2D {
    prefix: Vec<Vec<i32>>,
    rows: usize,
    cols: usize,
}

impl RangeSumQuery2D {
    /// Range Sum Query 2D Immutable
    ///
    /// Pattern: 2D prefix sum.
    /// Idea: each region is four prefix rectangles combined by inclusion-exclusion.
    ///
    /// Time:
    /// - `new`: O(m * n)
    /// - `sum_region`: O(1)
    ///
    /// Space: O(m * n)
    pub fn new(matrix: Vec<Vec<i32>>) -> Self {
        let rows = matrix.len();
        let cols = matrix.first().map_or(0, Vec::len);
        let mut prefix = vec![vec![0; cols + 1]; rows + 1];

        for row in 0..rows {
            for col in 0..cols {
                prefix[row + 1][col + 1] =
                    matrix[row][col] + prefix[row][col + 1] + prefix[row + 1][col]
                        - prefix[row][col];
            }
        }

        Self { prefix, rows, cols }
    }

    /// Returns the sum inside the inclusive rectangle.
    pub fn sum_region(&self, row1: usize, col1: usize, row2: usize, col2: usize) -> Option<i32> {
        if row1 > row2 || col1 > col2 || row2 >= self.rows || col2 >= self.cols {
            return None;
        }

        Some(
            self.prefix[row2 + 1][col2 + 1]
                - self.prefix[row1][col2 + 1]
                - self.prefix[row2 + 1][col1]
                + self.prefix[row1][col1],
        )
    }
}

/// Segment tree for point updates and range minimum queries.
///
/// Time:
/// - `update`: O(log n)
/// - `range_min`: O(log n)
///
/// Space: O(n)
#[derive(Debug, Clone)]
pub struct SegmentTree {
    size: usize,
    tree: Vec<i32>,
}

impl SegmentTree {
    /// Builds a segment tree from initial values.
    pub fn from_values(values: &[i32]) -> Self {
        if values.is_empty() {
            return Self {
                size: 0,
                tree: Vec::new(),
            };
        }

        let mut tree = vec![i32::MAX; values.len() * 4];
        build_segment_tree(values, &mut tree, 1, 0, values.len() - 1);

        Self {
            size: values.len(),
            tree,
        }
    }

    /// Replaces one value.
    pub fn update(&mut self, index: usize, value: i32) -> bool {
        if index >= self.size {
            return false;
        }

        update_segment_tree(&mut self.tree, 1, 0, self.size - 1, index, value);
        true
    }

    /// Returns the minimum value in an inclusive range.
    pub fn range_min(&self, left: usize, right: usize) -> Option<i32> {
        if self.size == 0 || left > right || right >= self.size {
            return None;
        }

        Some(query_segment_tree(
            &self.tree,
            1,
            0,
            self.size - 1,
            left,
            right,
        ))
    }

    /// Returns the number of logical positions.
    pub fn len(&self) -> usize {
        self.size
    }

    /// Returns whether the tree has no values.
    pub fn is_empty(&self) -> bool {
        self.size == 0
    }
}

fn build_segment_tree(values: &[i32], tree: &mut [i32], node: usize, left: usize, right: usize) {
    if left == right {
        tree[node] = values[left];
        return;
    }

    let middle = left + (right - left) / 2;
    build_segment_tree(values, tree, node * 2, left, middle);
    build_segment_tree(values, tree, node * 2 + 1, middle + 1, right);
    tree[node] = tree[node * 2].min(tree[node * 2 + 1]);
}

fn update_segment_tree(
    tree: &mut [i32],
    node: usize,
    left: usize,
    right: usize,
    index: usize,
    value: i32,
) {
    if left == right {
        tree[node] = value;
        return;
    }

    let middle = left + (right - left) / 2;

    if index <= middle {
        update_segment_tree(tree, node * 2, left, middle, index, value);
    } else {
        update_segment_tree(tree, node * 2 + 1, middle + 1, right, index, value);
    }

    tree[node] = tree[node * 2].min(tree[node * 2 + 1]);
}

fn query_segment_tree(
    tree: &[i32],
    node: usize,
    left: usize,
    right: usize,
    query_left: usize,
    query_right: usize,
) -> i32 {
    if query_left <= left && right <= query_right {
        return tree[node];
    }

    let middle = left + (right - left) / 2;
    let mut best = i32::MAX;

    if query_left <= middle {
        best = best.min(query_segment_tree(
            tree,
            node * 2,
            left,
            middle,
            query_left,
            query_right,
        ));
    }

    if query_right > middle {
        best = best.min(query_segment_tree(
            tree,
            node * 2 + 1,
            middle + 1,
            right,
            query_left,
            query_right,
        ));
    }

    best
}

/// Lazy segment tree for range additions and range sums.
///
/// Time:
/// - `range_add`: O(log n)
/// - `range_sum`: O(log n)
///
/// Space: O(n)
#[derive(Debug, Clone)]
pub struct LazySegmentTree {
    size: usize,
    tree: Vec<i32>,
    lazy: Vec<i32>,
}

impl LazySegmentTree {
    /// Builds a lazy segment tree from initial values.
    pub fn from_values(values: &[i32]) -> Self {
        if values.is_empty() {
            return Self {
                size: 0,
                tree: Vec::new(),
                lazy: Vec::new(),
            };
        }

        let mut tree = vec![0; values.len() * 4];
        build_sum_segment_tree(values, &mut tree, 1, 0, values.len() - 1);

        Self {
            size: values.len(),
            lazy: vec![0; values.len() * 4],
            tree,
        }
    }

    /// Adds `delta` to every value in an inclusive range.
    pub fn range_add(&mut self, left: usize, right: usize, delta: i32) -> bool {
        if self.size == 0 || left > right || right >= self.size {
            return false;
        }

        add_lazy_segment_tree(
            &mut self.tree,
            &mut self.lazy,
            1,
            0,
            self.size - 1,
            left,
            right,
            delta,
        );
        true
    }

    /// Returns the inclusive range sum, pushing pending updates as needed.
    pub fn range_sum(&mut self, left: usize, right: usize) -> Option<i32> {
        if self.size == 0 || left > right || right >= self.size {
            return None;
        }

        Some(query_lazy_segment_tree(
            &mut self.tree,
            &mut self.lazy,
            1,
            0,
            self.size - 1,
            left,
            right,
        ))
    }

    /// Returns the number of logical positions.
    pub fn len(&self) -> usize {
        self.size
    }

    /// Returns whether the tree has no values.
    pub fn is_empty(&self) -> bool {
        self.size == 0
    }
}

fn build_sum_segment_tree(
    values: &[i32],
    tree: &mut [i32],
    node: usize,
    left: usize,
    right: usize,
) {
    if left == right {
        tree[node] = values[left];
        return;
    }

    let middle = left + (right - left) / 2;
    build_sum_segment_tree(values, tree, node * 2, left, middle);
    build_sum_segment_tree(values, tree, node * 2 + 1, middle + 1, right);
    tree[node] = tree[node * 2] + tree[node * 2 + 1];
}

#[allow(clippy::too_many_arguments)]
fn add_lazy_segment_tree(
    tree: &mut [i32],
    lazy: &mut [i32],
    node: usize,
    left: usize,
    right: usize,
    query_left: usize,
    query_right: usize,
    delta: i32,
) {
    if query_left <= left && right <= query_right {
        apply_lazy_delta(tree, lazy, node, left, right, delta);
        return;
    }

    push_lazy_delta(tree, lazy, node, left, right);

    let middle = left + (right - left) / 2;

    if query_left <= middle {
        add_lazy_segment_tree(
            tree,
            lazy,
            node * 2,
            left,
            middle,
            query_left,
            query_right,
            delta,
        );
    }

    if query_right > middle {
        add_lazy_segment_tree(
            tree,
            lazy,
            node * 2 + 1,
            middle + 1,
            right,
            query_left,
            query_right,
            delta,
        );
    }

    tree[node] = tree[node * 2] + tree[node * 2 + 1];
}

fn query_lazy_segment_tree(
    tree: &mut [i32],
    lazy: &mut [i32],
    node: usize,
    left: usize,
    right: usize,
    query_left: usize,
    query_right: usize,
) -> i32 {
    if query_left <= left && right <= query_right {
        return tree[node];
    }

    push_lazy_delta(tree, lazy, node, left, right);

    let middle = left + (right - left) / 2;
    let mut total = 0;

    if query_left <= middle {
        total +=
            query_lazy_segment_tree(tree, lazy, node * 2, left, middle, query_left, query_right);
    }

    if query_right > middle {
        total += query_lazy_segment_tree(
            tree,
            lazy,
            node * 2 + 1,
            middle + 1,
            right,
            query_left,
            query_right,
        );
    }

    total
}

fn push_lazy_delta(tree: &mut [i32], lazy: &mut [i32], node: usize, left: usize, right: usize) {
    if lazy[node] == 0 || left == right {
        return;
    }

    let middle = left + (right - left) / 2;
    let delta = lazy[node];

    apply_lazy_delta(tree, lazy, node * 2, left, middle, delta);
    apply_lazy_delta(tree, lazy, node * 2 + 1, middle + 1, right, delta);
    lazy[node] = 0;
}

fn apply_lazy_delta(
    tree: &mut [i32],
    lazy: &mut [i32],
    node: usize,
    left: usize,
    right: usize,
    delta: i32,
) {
    tree[node] += delta * (right - left + 1) as i32;
    lazy[node] += delta;
}

/// Difference array for batched inclusive range increments.
///
/// Time:
/// - `increment_range`: O(1)
/// - `values`: O(n)
///
/// Space: O(n)
#[derive(Debug, Clone)]
pub struct DifferenceArray {
    difference: Vec<i32>,
}

impl DifferenceArray {
    /// Creates a zero-filled difference array of `size` values.
    pub fn new(size: usize) -> Self {
        Self {
            difference: vec![0; size + 1],
        }
    }

    /// Adds `delta` to every value in an inclusive range.
    pub fn increment_range(&mut self, left: usize, right: usize, delta: i32) -> bool {
        if left > right || right >= self.len() {
            return false;
        }

        self.difference[left] += delta;
        self.difference[right + 1] -= delta;
        true
    }

    /// Materializes the values represented by the difference array.
    pub fn values(&self) -> Vec<i32> {
        let mut current = 0;
        let mut result = Vec::with_capacity(self.len());

        for &delta in self.difference.iter().take(self.len()) {
            current += delta;
            result.push(current);
        }

        result
    }

    /// Returns the number of logical values.
    pub fn len(&self) -> usize {
        self.difference.len().saturating_sub(1)
    }

    /// Returns whether there are no logical values.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

/// Applies seat-booking increments to flights numbered from `1`.
///
/// Pattern: difference array.
/// Time: O(n + b)
/// Space: O(n)
pub fn corporate_flight_bookings(
    bookings: &[(usize, usize, i32)],
    flight_count: usize,
) -> Vec<i32> {
    let mut difference = DifferenceArray::new(flight_count);

    for &(first, last, seats) in bookings {
        if first == 0 || last == 0 {
            continue;
        }

        difference.increment_range(first - 1, last - 1, seats);
    }

    difference.values()
}

/// Range Addition
///
/// Pattern: difference array.
/// Idea: mark the start and the position after the end, then prefix-scan once.
///
/// Time: O(n + u)
/// Space: O(n)
pub fn range_addition(length: usize, updates: &[(usize, usize, i32)]) -> Vec<i32> {
    let mut difference = DifferenceArray::new(length);

    for &(left, right, delta) in updates {
        difference.increment_range(left, right, delta);
    }

    difference.values()
}

/// Checks whether all car-pooling trips fit within capacity.
///
/// Pattern: difference array over route positions.
/// Time: O(n + max destination)
/// Space: O(max destination)
pub fn car_pooling(trips: &[(i32, usize, usize)], capacity: i32) -> bool {
    if capacity < 0 {
        return false;
    }

    let Some(max_destination) = trips.iter().map(|&(_, _, destination)| destination).max() else {
        return true;
    };

    let mut difference = DifferenceArray::new(max_destination);

    for &(passengers, origin, destination) in trips {
        if passengers <= 0 || origin >= destination {
            return false;
        }

        difference.increment_range(origin, destination - 1, passengers);
    }

    difference
        .values()
        .into_iter()
        .all(|passengers| passengers <= capacity)
}

/// Counts, for each value, how many smaller values appear to its right.
///
/// Pattern: coordinate compression + Fenwick tree.
/// Time: O(n log n)
/// Space: O(n)
pub fn count_smaller_numbers_after_self(values: Vec<i32>) -> Vec<i32> {
    let mut sorted_values = values.clone();
    sorted_values.sort_unstable();
    sorted_values.dedup();

    let mut tree = FenwickTree::new(sorted_values.len());
    let mut counts = vec![0; values.len()];

    for (index, value) in values.iter().enumerate().rev() {
        let rank = sorted_values
            .binary_search(value)
            .expect("value comes from the compressed coordinate set");

        counts[index] = if rank == 0 {
            0
        } else {
            tree.prefix_sum(rank - 1).unwrap_or(0)
        };

        tree.add(rank, 1);
    }

    counts
}

/// Counts reverse pairs where `left > 2 * right`.
///
/// Pattern: modified merge sort.
/// Time: O(n log n)
/// Space: O(n)
pub fn reverse_pairs(values: Vec<i32>) -> i32 {
    let mut values: Vec<i64> = values.into_iter().map(i64::from).collect();
    let mut buffer = values.clone();

    count_reverse_pairs(&mut values, &mut buffer) as i32
}

/// Count of Range Sum
///
/// Pattern: prefix sums + modified merge sort.
/// Idea: for each left prefix, count right prefixes whose difference is in range.
///
/// Time: O(n log n)
/// Space: O(n)
pub fn count_range_sum(nums: Vec<i32>, lower: i64, upper: i64) -> i32 {
    let mut prefix = Vec::with_capacity(nums.len() + 1);
    let mut running = 0_i64;
    prefix.push(running);

    for value in nums {
        running += i64::from(value);
        prefix.push(running);
    }

    let mut buffer = prefix.clone();
    count_range_sum_sorted(&mut prefix, &mut buffer, lower, upper) as i32
}

fn count_range_sum_sorted(prefix: &mut [i64], buffer: &mut [i64], lower: i64, upper: i64) -> i64 {
    if prefix.len() <= 1 {
        return 0;
    }

    let middle = prefix.len() / 2;
    let (left_prefix, right_prefix) = prefix.split_at_mut(middle);
    let (left_buffer, right_buffer) = buffer.split_at_mut(middle);

    let mut total = count_range_sum_sorted(left_prefix, left_buffer, lower, upper);
    total += count_range_sum_sorted(right_prefix, right_buffer, lower, upper);

    let mut lower_index = 0;
    let mut upper_index = 0;
    for &left_value in left_prefix.iter() {
        while lower_index < right_prefix.len() && right_prefix[lower_index] - left_value < lower {
            lower_index += 1;
        }
        while upper_index < right_prefix.len() && right_prefix[upper_index] - left_value <= upper {
            upper_index += 1;
        }
        total += (upper_index - lower_index) as i64;
    }

    merge_sorted_slices(left_prefix, right_prefix, buffer);
    prefix.copy_from_slice(&buffer[..prefix.len()]);

    total
}

/// Sliding Window Maximum
///
/// Pattern: monotonic deque.
/// Idea: keep candidate indexes with values in decreasing order.
///
/// Time: O(n)
/// Space: O(k)
pub fn sliding_window_maximum(values: Vec<i32>, window_size: usize) -> Vec<i32> {
    if window_size == 0 || values.len() < window_size {
        return Vec::new();
    }

    let mut deque = VecDeque::new();
    let mut result = Vec::new();

    for index in 0..values.len() {
        while deque
            .front()
            .is_some_and(|&front| front + window_size <= index)
        {
            deque.pop_front();
        }

        while deque
            .back()
            .is_some_and(|&back| values[back] <= values[index])
        {
            deque.pop_back();
        }

        deque.push_back(index);

        if index + 1 >= window_size {
            let front = *deque.front().expect("window has at least one candidate");
            result.push(values[front]);
        }
    }

    result
}

/// Queue Reconstruction by Height
///
/// Pattern: sort tall people first, then insert by k.
/// Idea: shorter people inserted later do not affect the count of taller people.
///
/// Time: O(n^2)
/// Space: O(n)
pub fn queue_reconstruction_by_height(mut people: Vec<(i32, i32)>) -> Vec<(i32, i32)> {
    people.sort_unstable_by(|&(left_height, left_k), &(right_height, right_k)| {
        right_height
            .cmp(&left_height)
            .then_with(|| left_k.cmp(&right_k))
    });

    let mut queue = Vec::with_capacity(people.len());

    for person @ (_, k) in people {
        let index = (k as usize).min(queue.len());
        queue.insert(index, person);
    }

    queue
}

/// Versioned array with point updates and snapshot reads.
#[derive(Debug, Clone)]
pub struct SnapshotArray {
    current_snap: usize,
    values: Vec<Vec<(usize, i32)>>,
}

impl SnapshotArray {
    /// Snapshot Array
    ///
    /// Pattern: versioned values per index.
    /// Idea: store only changes and binary-search the latest value at a snapshot.
    ///
    /// Time:
    /// - `set`: O(1)
    /// - `snap`: O(1)
    /// - `get`: O(log c)
    ///
    /// Space: O(length + changes)
    pub fn new(length: usize) -> Self {
        Self {
            current_snap: 0,
            values: vec![vec![(0, 0)]; length],
        }
    }

    /// Sets a value for the current snapshot version.
    pub fn set(&mut self, index: usize, value: i32) -> bool {
        let Some(changes) = self.values.get_mut(index) else {
            return false;
        };

        if let Some(last) = changes.last_mut() {
            if last.0 == self.current_snap {
                last.1 = value;
                return true;
            }
        }

        changes.push((self.current_snap, value));
        true
    }

    /// Freezes the current version and returns its snapshot id.
    pub fn snap(&mut self) -> usize {
        let id = self.current_snap;
        self.current_snap += 1;
        id
    }

    /// Reads the latest value at or before `snap_id`.
    pub fn get(&self, index: usize, snap_id: usize) -> Option<i32> {
        if snap_id >= self.current_snap {
            return None;
        }

        let changes = self.values.get(index)?;
        let position = changes.partition_point(|&(id, _)| id <= snap_id);
        Some(changes[position.saturating_sub(1)].1)
    }
}

fn count_reverse_pairs(values: &mut [i64], buffer: &mut [i64]) -> i64 {
    if values.len() <= 1 {
        return 0;
    }

    let middle = values.len() / 2;
    let (left_values, right_values) = values.split_at_mut(middle);
    let (left_buffer, right_buffer) = buffer.split_at_mut(middle);

    let mut total = count_reverse_pairs(left_values, left_buffer);
    total += count_reverse_pairs(right_values, right_buffer);

    let mut right_index = 0;
    for &left_value in left_values.iter() {
        while right_index < right_values.len() && left_value > 2 * right_values[right_index] {
            right_index += 1;
        }

        total += right_index as i64;
    }

    merge_sorted_slices(left_values, right_values, buffer);
    values.copy_from_slice(&buffer[..values.len()]);

    total
}

fn merge_sorted_slices(left: &[i64], right: &[i64], output: &mut [i64]) {
    let mut left_index = 0;
    let mut right_index = 0;
    let mut output_index = 0;

    while left_index < left.len() && right_index < right.len() {
        if left[left_index] <= right[right_index] {
            output[output_index] = left[left_index];
            left_index += 1;
        } else {
            output[output_index] = right[right_index];
            right_index += 1;
        }

        output_index += 1;
    }

    while left_index < left.len() {
        output[output_index] = left[left_index];
        left_index += 1;
        output_index += 1;
    }

    while right_index < right.len() {
        output[output_index] = right[right_index];
        right_index += 1;
        output_index += 1;
    }
}

/// Calendar that accepts only non-overlapping bookings.
///
/// Time: O(log n) per booking.
/// Space: O(n)
#[derive(Debug, Clone, Default)]
pub struct MyCalendar {
    events: BTreeMap<i32, i32>,
}

impl MyCalendar {
    /// Creates an empty calendar.
    pub fn new() -> Self {
        Self::default()
    }

    /// Attempts to book a half-open interval `[start, end)`.
    pub fn book(&mut self, start: i32, end: i32) -> bool {
        if start >= end {
            return false;
        }

        if let Some((_, &previous_end)) = self.events.range(..=start).next_back() {
            if previous_end > start {
                return false;
            }
        }

        if let Some((&next_start, _)) = self.events.range(start..).next() {
            if next_start < end {
                return false;
            }
        }

        self.events.insert(start, end);
        true
    }

    /// Returns the number of accepted bookings.
    pub fn len(&self) -> usize {
        self.events.len()
    }

    /// Returns whether the calendar has no bookings.
    pub fn is_empty(&self) -> bool {
        self.events.is_empty()
    }
}

/// Calendar that allows double bookings but rejects triple bookings.
///
/// Time: O(n) per booking.
/// Space: O(n)
#[derive(Debug, Clone, Default)]
pub struct MyCalendarTwo {
    bookings: Vec<(i32, i32)>,
    double_bookings: Vec<(i32, i32)>,
}

impl MyCalendarTwo {
    /// Creates an empty calendar.
    pub fn new() -> Self {
        Self::default()
    }

    /// Attempts to book a half-open interval `[start, end)`.
    pub fn book(&mut self, start: i32, end: i32) -> bool {
        if start >= end {
            return false;
        }

        if self
            .double_bookings
            .iter()
            .any(|&(booked_start, booked_end)| {
                intervals_overlap(start, end, booked_start, booked_end)
            })
        {
            return false;
        }

        for &(booked_start, booked_end) in &self.bookings {
            if intervals_overlap(start, end, booked_start, booked_end) {
                self.double_bookings
                    .push((start.max(booked_start), end.min(booked_end)));
            }
        }

        self.bookings.push((start, end));
        true
    }

    /// Returns the number of accepted bookings.
    pub fn len(&self) -> usize {
        self.bookings.len()
    }

    /// Returns whether the calendar has no bookings.
    pub fn is_empty(&self) -> bool {
        self.bookings.is_empty()
    }
}

fn intervals_overlap(left_start: i32, left_end: i32, right_start: i32, right_end: i32) -> bool {
    left_start < right_end && right_start < left_end
}
