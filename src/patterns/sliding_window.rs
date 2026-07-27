use std::collections::HashMap;

/// Best Time to Buy and Sell Stock
///
/// Pattern: one-pass sliding decision window.
/// Idea: keep the cheapest price seen so far and test selling today.
///
/// Time: O(n)
/// Space: O(1)
pub fn max_profit(prices: Vec<i32>) -> i32 {
    let mut best_profit = 0;
    let mut lowest_price = i32::MAX;

    for price in prices {
        lowest_price = lowest_price.min(price);
        best_profit = best_profit.max(price - lowest_price);
    }

    best_profit
}

/// Longest Substring Without Repeating Characters
///
/// Pattern: variable-size sliding window.
/// Idea: move the left side past the last repeated character.
///
/// Time: O(n)
/// Space: O(k), where k is the number of distinct characters in the window.
pub fn length_of_longest_substring(text: &str) -> usize {
    let mut last_seen = HashMap::new();
    let mut left = 0;
    let mut best = 0;

    for (right, character) in text.chars().enumerate() {
        if let Some(&previous_index) = last_seen.get(&character) {
            left = left.max(previous_index + 1);
        }

        last_seen.insert(character, right);
        best = best.max(right - left + 1);
    }

    best
}

/// Longest Repeating Character Replacement
///
/// Pattern: variable-size sliding window with frequency maximum.
/// Idea: a window is repairable when its non-majority characters fit inside the
/// replacement budget, so shrink only when that budget is exceeded.
///
/// Time: O(n)
/// Space: O(k), where k is the number of distinct characters in the window.
pub fn longest_repeating_character_replacement(text: &str, replacements: usize) -> usize {
    let characters: Vec<char> = text.chars().collect();
    let mut counts = HashMap::new();
    let mut left = 0;
    let mut highest_frequency = 0;
    let mut longest = 0;

    for (right, &character) in characters.iter().enumerate() {
        let count = counts.entry(character).or_insert(0_usize);
        *count += 1;
        highest_frequency = highest_frequency.max(*count);

        while right - left + 1 - highest_frequency > replacements {
            let left_character = characters[left];
            *counts
                .get_mut(&left_character)
                .expect("window character exists") -= 1;
            left += 1;
        }

        longest = longest.max(right - left + 1);
    }

    longest
}

/// Permutation in String
///
/// Pattern: fixed-size sliding window with frequency deficit.
/// Idea: move a window as long as the pattern and track how many required
/// characters are still missing, instead of generating any permutation.
///
/// Time: O(n + m)
/// Space: O(k), where k is the number of distinct pattern characters.
pub fn permutation_in_string(pattern: &str, text: &str) -> bool {
    let pattern_length = pattern.chars().count();
    if pattern_length == 0 {
        return true;
    }

    let characters: Vec<char> = text.chars().collect();
    if pattern_length > characters.len() {
        return false;
    }

    let mut needed = HashMap::new();
    for character in pattern.chars() {
        *needed.entry(character).or_insert(0_i32) += 1;
    }

    let mut missing = pattern_length as i32;
    let mut left = 0;

    for (right, &character) in characters.iter().enumerate() {
        if let Some(count) = needed.get_mut(&character) {
            if *count > 0 {
                missing -= 1;
            }
            *count -= 1;
        }

        if right - left + 1 > pattern_length {
            let left_character = characters[left];
            if let Some(count) = needed.get_mut(&left_character) {
                *count += 1;
                if *count > 0 {
                    missing += 1;
                }
            }
            left += 1;
        }

        if right - left + 1 == pattern_length && missing == 0 {
            return true;
        }
    }

    false
}

/// Minimum Size Subarray Sum
///
/// Pattern: positive-value sliding window.
/// Idea: once a positive window reaches the target, shrinking it is the only
/// way to find a shorter valid window ending at the same right boundary.
///
/// Time: O(n)
/// Space: O(1)
pub fn minimum_size_subarray_sum(target: i32, values: &[i32]) -> usize {
    if target <= 0 {
        return 0;
    }
    let mut left = 0;
    let mut sum = 0_i64;
    let mut best = usize::MAX;
    for (right, &value) in values.iter().enumerate() {
        sum += i64::from(value);
        while sum >= i64::from(target) {
            best = best.min(right - left + 1);
            sum -= i64::from(values[left]);
            left += 1;
        }
    }
    if best == usize::MAX {
        0
    } else {
        best
    }
}

/// Minimum Window Substring
///
/// Pattern: variable-size sliding window.
/// Idea: expand until all required counts are covered, then shrink while valid.
///
/// Note: this implementation uses bytes because the classic interview problem
/// is ASCII-oriented.
///
/// Time: O(n + m)
/// Space: O(1), bounded by the byte alphabet.
pub fn min_window(source: &str, target: &str) -> String {
    if target.is_empty() || source.is_empty() {
        return String::new();
    }

    let source_bytes = source.as_bytes();
    let target_bytes = target.as_bytes();
    let mut required = [0; 256];

    for &byte in target_bytes {
        required[byte as usize] += 1;
    }

    let mut missing = target_bytes.len() as i32;
    let mut left = 0;
    let mut best_start = 0;
    let mut best_len = usize::MAX;

    for right in 0..source_bytes.len() {
        let right_byte = source_bytes[right] as usize;

        if required[right_byte] > 0 {
            missing -= 1;
        }

        required[right_byte] -= 1;

        while missing == 0 {
            let window_len = right - left + 1;

            if window_len < best_len {
                best_start = left;
                best_len = window_len;
            }

            let left_byte = source_bytes[left] as usize;
            required[left_byte] += 1;

            if required[left_byte] > 0 {
                missing += 1;
            }

            left += 1;
        }
    }

    if best_len == usize::MAX {
        String::new()
    } else {
        source[best_start..best_start + best_len].to_string()
    }
}
