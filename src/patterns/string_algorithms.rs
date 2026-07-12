use std::collections::HashSet;

pub fn find_pattern_positions(text: &str, pattern: &str) -> Vec<usize> {
    if pattern.is_empty() || pattern.len() > text.len() {
        return Vec::new();
    }

    let text_bytes = text.as_bytes();
    let pattern_bytes = pattern.as_bytes();
    let prefix = prefix_table(pattern_bytes);
    let mut matches = Vec::new();
    let mut pattern_index = 0;

    for (text_index, &byte) in text_bytes.iter().enumerate() {
        while pattern_index > 0 && byte != pattern_bytes[pattern_index] {
            pattern_index = prefix[pattern_index - 1];
        }

        if byte == pattern_bytes[pattern_index] {
            pattern_index += 1;
        }

        if pattern_index == pattern_bytes.len() {
            matches.push(text_index + 1 - pattern_bytes.len());
            pattern_index = prefix[pattern_index - 1];
        }
    }

    matches
}

pub fn find_anagram_starts(text: &str, pattern: &str) -> Vec<usize> {
    if pattern.is_empty() || pattern.len() > text.len() {
        return Vec::new();
    }

    let text_bytes = text.as_bytes();
    let pattern_bytes = pattern.as_bytes();
    let window_size = pattern_bytes.len();
    let mut expected = [0; 26];
    let mut window = [0; 26];
    let mut starts = Vec::new();

    for &byte in pattern_bytes {
        expected[letter_index(byte)] += 1;
    }

    for (index, &byte) in text_bytes.iter().enumerate() {
        window[letter_index(byte)] += 1;

        if index >= window_size {
            window[letter_index(text_bytes[index - window_size])] -= 1;
        }

        if index + 1 >= window_size && window == expected {
            starts.push(index + 1 - window_size);
        }
    }

    starts
}

pub fn repeated_substring_pattern(text: &str) -> bool {
    let length = text.len();

    if length < 2 {
        return false;
    }

    let doubled = format!("{}{}", text, text);
    doubled[1..doubled.len() - 1].contains(text)
}

pub fn longest_duplicate_substring(text: &str) -> String {
    let bytes = text.as_bytes();
    let mut left = 1;
    let mut right = bytes.len();
    let mut best_start = 0;
    let mut best_length = 0;

    while left <= right {
        let length = left + (right - left) / 2;

        if let Some(start) = duplicate_start_of_length(bytes, length) {
            best_start = start;
            best_length = length;
            left = length + 1;
        } else if length == 0 {
            break;
        } else {
            right = length - 1;
        }
    }

    String::from_utf8(bytes[best_start..best_start + best_length].to_vec()).unwrap_or_default()
}

fn duplicate_start_of_length(bytes: &[u8], length: usize) -> Option<usize> {
    if length == 0 {
        return Some(0);
    }

    if length > bytes.len() {
        return None;
    }

    let mut seen = HashSet::new();

    for start in 0..=bytes.len() - length {
        let window = &bytes[start..start + length];

        if !seen.insert(window) {
            return Some(start);
        }
    }

    None
}

fn prefix_table(pattern: &[u8]) -> Vec<usize> {
    let mut prefix = vec![0; pattern.len()];
    let mut length = 0;

    for index in 1..pattern.len() {
        while length > 0 && pattern[index] != pattern[length] {
            length = prefix[length - 1];
        }

        if pattern[index] == pattern[length] {
            length += 1;
            prefix[index] = length;
        }
    }

    prefix
}

fn letter_index(byte: u8) -> usize {
    (byte - b'a') as usize
}
