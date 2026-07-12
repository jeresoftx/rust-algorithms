use std::collections::BTreeMap;

#[derive(Debug, Clone)]
pub struct FenwickTree {
    tree: Vec<i32>,
}

impl FenwickTree {
    pub fn new(size: usize) -> Self {
        Self {
            tree: vec![0; size + 1],
        }
    }

    pub fn from_values(values: &[i32]) -> Self {
        let mut tree = Self::new(values.len());

        for (index, &value) in values.iter().enumerate() {
            tree.add(index, value);
        }

        tree
    }

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

    pub fn len(&self) -> usize {
        self.tree.len().saturating_sub(1)
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

fn lowbit(value: usize) -> usize {
    value & value.wrapping_neg()
}

#[derive(Debug, Clone)]
pub struct RangeSumQuery {
    values: Vec<i32>,
    tree: FenwickTree,
}

impl RangeSumQuery {
    pub fn new(values: Vec<i32>) -> Self {
        Self {
            tree: FenwickTree::from_values(&values),
            values,
        }
    }

    pub fn update(&mut self, index: usize, value: i32) -> bool {
        let Some(current) = self.values.get_mut(index) else {
            return false;
        };

        let delta = value - *current;
        *current = value;
        self.tree.add(index, delta)
    }

    pub fn sum_range(&self, left: usize, right: usize) -> Option<i32> {
        self.tree.range_sum(left, right)
    }
}

#[derive(Debug, Clone)]
pub struct SegmentTree {
    size: usize,
    tree: Vec<i32>,
}

impl SegmentTree {
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

    pub fn update(&mut self, index: usize, value: i32) -> bool {
        if index >= self.size {
            return false;
        }

        update_segment_tree(&mut self.tree, 1, 0, self.size - 1, index, value);
        true
    }

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

    pub fn len(&self) -> usize {
        self.size
    }

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

#[derive(Debug, Clone)]
pub struct DifferenceArray {
    difference: Vec<i32>,
}

impl DifferenceArray {
    pub fn new(size: usize) -> Self {
        Self {
            difference: vec![0; size + 1],
        }
    }

    pub fn increment_range(&mut self, left: usize, right: usize, delta: i32) -> bool {
        if left > right || right >= self.len() {
            return false;
        }

        self.difference[left] += delta;
        self.difference[right + 1] -= delta;
        true
    }

    pub fn values(&self) -> Vec<i32> {
        let mut current = 0;
        let mut result = Vec::with_capacity(self.len());

        for &delta in self.difference.iter().take(self.len()) {
            current += delta;
            result.push(current);
        }

        result
    }

    pub fn len(&self) -> usize {
        self.difference.len().saturating_sub(1)
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

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

#[derive(Debug, Clone, Default)]
pub struct MyCalendar {
    events: BTreeMap<i32, i32>,
}

impl MyCalendar {
    pub fn new() -> Self {
        Self::default()
    }

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

    pub fn len(&self) -> usize {
        self.events.len()
    }

    pub fn is_empty(&self) -> bool {
        self.events.is_empty()
    }
}

#[derive(Debug, Clone, Default)]
pub struct MyCalendarTwo {
    bookings: Vec<(i32, i32)>,
    double_bookings: Vec<(i32, i32)>,
}

impl MyCalendarTwo {
    pub fn new() -> Self {
        Self::default()
    }

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

    pub fn len(&self) -> usize {
        self.bookings.len()
    }

    pub fn is_empty(&self) -> bool {
        self.bookings.is_empty()
    }
}

fn intervals_overlap(left_start: i32, left_end: i32, right_start: i32, right_end: i32) -> bool {
    left_start < right_end && right_start < left_end
}
