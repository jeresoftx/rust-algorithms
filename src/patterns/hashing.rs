use std::collections::HashMap;

/// Two Sum
///
/// Pattern: hashing.
/// Idea: scan once while remembering each previous value and its index.
/// For each number, check whether its complement was already seen.
///
/// Time: O(n)
/// Space: O(n)
pub fn two_sum(nums: Vec<i32>, target: i32) -> Option<(usize, usize)> {
    let mut seen = HashMap::new();

    for (index, value) in nums.into_iter().enumerate() {
        let complement = target - value;

        if let Some(&previous_index) = seen.get(&complement) {
            return Some((previous_index, index));
        }

        seen.insert(value, index);
    }

    None
}
