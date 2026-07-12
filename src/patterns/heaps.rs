use std::cmp::Reverse;
use std::collections::BinaryHeap;

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
