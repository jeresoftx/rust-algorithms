use std::collections::{HashMap, HashSet};

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

/// Valid Anagram
///
/// Pattern: frequency counting.
/// Idea: anagrams have the same character counts.
///
/// Time: O(n)
/// Space: O(k), where k is the number of distinct characters.
pub fn valid_anagram(first: &str, second: &str) -> bool {
    if first.len() != second.len() {
        return false;
    }

    let mut counts = HashMap::new();

    for character in first.chars() {
        *counts.entry(character).or_insert(0) += 1;
    }

    for character in second.chars() {
        let Some(count) = counts.get_mut(&character) else {
            return false;
        };

        *count -= 1;
        if *count == 0 {
            counts.remove(&character);
        }
    }

    counts.is_empty()
}

/// Contains Duplicate
///
/// Pattern: set membership.
/// Idea: if inserting into a set fails, the value was already present.
///
/// Time: O(n)
/// Space: O(n)
pub fn contains_duplicate(nums: Vec<i32>) -> bool {
    let mut seen = HashSet::new();

    for value in nums {
        if !seen.insert(value) {
            return true;
        }
    }

    false
}

/// Group Anagrams
///
/// Pattern: canonical hash key.
/// Idea: words with the same sorted characters belong to the same group.
///
/// Time: O(n * m log m), where m is the maximum word length.
/// Space: O(n * m)
pub fn group_anagrams(words: Vec<String>) -> Vec<Vec<String>> {
    let mut groups: HashMap<String, Vec<String>> = HashMap::new();

    for word in words {
        let mut characters: Vec<char> = word.chars().collect();
        characters.sort_unstable();
        let key: String = characters.into_iter().collect();

        groups.entry(key).or_default().push(word);
    }

    groups.into_values().collect()
}

/// Product of Array Except Self
///
/// Pattern: prefix and suffix products.
/// Idea: store the product before each index, then multiply by the product
/// after each index in a reverse pass.
///
/// Time: O(n)
/// Space: O(1) extra space, excluding the returned vector.
pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
    let mut result = vec![1; nums.len()];
    let mut prefix = 1;

    for (index, value) in nums.iter().enumerate() {
        result[index] = prefix;
        prefix *= value;
    }

    let mut suffix = 1;

    for (index, value) in nums.iter().enumerate().rev() {
        result[index] *= suffix;
        suffix *= value;
    }

    result
}
