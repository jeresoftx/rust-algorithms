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
