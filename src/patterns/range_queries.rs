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
