use std::cmp::Reverse;
use std::collections::BinaryHeap;

use crate::patterns::linked_lists::{list_from_vec, Link};

/// Kth Largest Element in an Array
///
/// Pattern: bounded min heap.
/// Idea: keep only the k largest values seen so far.
///
/// Time: O(n log k)
/// Space: O(k)
pub fn kth_largest(nums: Vec<i32>, k: usize) -> Option<i32> {
    if k == 0 || k > nums.len() {
        return None;
    }

    let mut heap = BinaryHeap::new();

    for value in nums {
        heap.push(Reverse(value));

        if heap.len() > k {
            heap.pop();
        }
    }

    heap.peek().map(|Reverse(value)| *value)
}

/// Find Median from Data Stream
///
/// Pattern: two heaps.
/// Idea: lower half is a max heap and upper half is a min heap.
///
/// Time:
/// - `add_num`: O(log n)
/// - `find_median`: O(1)
///
/// Space: O(n)
#[derive(Debug, Default)]
pub struct MedianFinder {
    lower: BinaryHeap<i32>,
    upper: BinaryHeap<Reverse<i32>>,
}

impl MedianFinder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_num(&mut self, value: i32) {
        if self.lower.peek().is_none_or(|&top| value <= top) {
            self.lower.push(value);
        } else {
            self.upper.push(Reverse(value));
        }

        self.rebalance();
    }

    pub fn find_median(&self) -> Option<f64> {
        match (self.lower.peek(), self.upper.peek()) {
            (None, None) => None,
            (Some(&left), Some(&Reverse(right))) if self.lower.len() == self.upper.len() => {
                Some((left as f64 + right as f64) / 2.0)
            }
            (Some(&left), _) => Some(left as f64),
            (None, Some(&Reverse(right))) => Some(right as f64),
        }
    }

    fn rebalance(&mut self) {
        if self.lower.len() > self.upper.len() + 1 {
            if let Some(value) = self.lower.pop() {
                self.upper.push(Reverse(value));
            }
        } else if self.upper.len() > self.lower.len() {
            if let Some(Reverse(value)) = self.upper.pop() {
                self.lower.push(value);
            }
        }
    }
}

/// Merge K Sorted Lists
///
/// Pattern: min heap over current list heads.
/// Idea: always consume the smallest available head and push that list's next node.
///
/// Time: O(n log k)
/// Space: O(k + n) for the output list values
pub fn merge_k_sorted_lists(mut lists: Vec<Link>) -> Link {
    let mut heap = BinaryHeap::new();

    for (index, list) in lists.iter().enumerate() {
        if let Some(node) = list.as_ref() {
            heap.push(Reverse((node.val, index)));
        }
    }

    let mut values = Vec::new();

    while let Some(Reverse((value, index))) = heap.pop() {
        values.push(value);

        if let Some(mut node) = lists[index].take() {
            lists[index] = node.next.take();
            if let Some(next) = lists[index].as_ref() {
                heap.push(Reverse((next.val, index)));
            }
        }
    }

    list_from_vec(values)
}

/// K Closest Points to Origin
///
/// Pattern: bounded max heap by distance.
/// Idea: keep only the k smallest squared distances seen so far.
///
/// Time: O(n log k)
/// Space: O(k)
pub fn k_closest_points_heap(points: Vec<(i32, i32)>, k: usize) -> Vec<(i32, i32)> {
    if k == 0 {
        return Vec::new();
    }

    let mut heap = BinaryHeap::new();

    for point @ (x, y) in points {
        let distance = x * x + y * y;
        heap.push((distance, point));

        if heap.len() > k {
            heap.pop();
        }
    }

    heap.into_iter().map(|(_, point)| point).collect()
}

/// Last Stone Weight
///
/// Pattern: max heap simulation.
/// Idea: repeatedly smash the two heaviest stones and push the remaining
/// difference when they are unequal.
///
/// Time: O(n log n)
/// Space: O(n)
pub fn last_stone_weight(stones: Vec<i32>) -> i32 {
    let mut heap = BinaryHeap::from(stones);

    while heap.len() > 1 {
        let first = heap.pop().expect("heap has at least two stones");
        let second = heap.pop().expect("heap has at least two stones");

        if first != second {
            heap.push(first - second);
        }
    }

    heap.pop().unwrap_or(0)
}
