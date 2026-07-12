use std::collections::{BTreeMap, HashSet, VecDeque};

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

pub fn find_multi_pattern_positions(text: &str, patterns: Vec<&str>) -> Vec<(String, Vec<usize>)> {
    let pattern_lengths: Vec<usize> = patterns.iter().map(|pattern| pattern.len()).collect();
    let mut matches = vec![Vec::new(); patterns.len()];
    let mut automaton = vec![AhoNode::default()];

    for (pattern_index, pattern) in patterns.iter().enumerate() {
        if pattern.is_empty() {
            continue;
        }

        let mut node = 0;

        for &byte in pattern.as_bytes() {
            if let Some(&next) = automaton[node].children.get(&byte) {
                node = next;
            } else {
                automaton.push(AhoNode::default());
                let next = automaton.len() - 1;
                automaton[node].children.insert(byte, next);
                node = next;
            }
        }

        automaton[node].outputs.push(pattern_index);
    }

    build_failure_links(&mut automaton);

    let mut node = 0;

    for (text_index, &byte) in text.as_bytes().iter().enumerate() {
        while node != 0 && !automaton[node].children.contains_key(&byte) {
            node = automaton[node].failure;
        }

        node = automaton[node].children.get(&byte).copied().unwrap_or(0);

        for &pattern_index in &automaton[node].outputs {
            matches[pattern_index].push(text_index + 1 - pattern_lengths[pattern_index]);
        }
    }

    patterns
        .into_iter()
        .enumerate()
        .map(|(index, pattern)| (pattern.to_string(), matches[index].clone()))
        .collect()
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

pub fn longest_common_prefix(words: Vec<String>) -> String {
    if words.is_empty() {
        return String::new();
    }

    let mut prefix: Vec<char> = words[0].chars().collect();

    for word in words.iter().skip(1) {
        let common_length = prefix
            .iter()
            .zip(word.chars())
            .take_while(|(left, right)| **left == *right)
            .count();

        prefix.truncate(common_length);

        if prefix.is_empty() {
            break;
        }
    }

    prefix.into_iter().collect()
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

#[derive(Default)]
struct AhoNode {
    children: BTreeMap<u8, usize>,
    failure: usize,
    outputs: Vec<usize>,
}

fn build_failure_links(automaton: &mut [AhoNode]) {
    let mut queue = VecDeque::new();
    let root_children: Vec<usize> = automaton[0].children.values().copied().collect();

    for child in root_children {
        queue.push_back(child);
    }

    while let Some(node) = queue.pop_front() {
        let transitions: Vec<(u8, usize)> = automaton[node]
            .children
            .iter()
            .map(|(&byte, &child)| (byte, child))
            .collect();

        for (byte, child) in transitions {
            let mut failure = automaton[node].failure;

            while failure != 0 && !automaton[failure].children.contains_key(&byte) {
                failure = automaton[failure].failure;
            }

            automaton[child].failure = automaton[failure].children.get(&byte).copied().unwrap_or(0);

            let inherited_outputs = automaton[automaton[child].failure].outputs.clone();
            automaton[child].outputs.extend(inherited_outputs);
            queue.push_back(child);
        }
    }
}
