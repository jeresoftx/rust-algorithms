use rust_algorithms::patterns::string_algorithms::{
    find_anagram_starts, find_pattern_positions, longest_common_prefix,
    longest_duplicate_substring, repeated_substring_pattern,
};

#[test]
fn find_pattern_positions_returns_all_kmp_matches() {
    assert_eq!(find_pattern_positions("ababcabcabababd", "ababd"), vec![10]);
}

#[test]
fn find_pattern_positions_handles_overlapping_matches() {
    assert_eq!(find_pattern_positions("aaaaa", "aaa"), vec![0, 1, 2]);
}

#[test]
fn find_pattern_positions_returns_empty_for_missing_pattern() {
    assert!(find_pattern_positions("abcdef", "gh").is_empty());
}

#[test]
fn find_pattern_positions_returns_empty_for_empty_pattern() {
    assert!(find_pattern_positions("abcdef", "").is_empty());
}

#[test]
fn find_anagram_starts_returns_all_window_starts() {
    assert_eq!(find_anagram_starts("cbaebabacd", "abc"), vec![0, 6]);
}

#[test]
fn find_anagram_starts_handles_repeated_letters() {
    assert_eq!(find_anagram_starts("abab", "ab"), vec![0, 1, 2]);
}

#[test]
fn find_anagram_starts_returns_empty_when_pattern_is_longer() {
    assert!(find_anagram_starts("ab", "abc").is_empty());
}

#[test]
fn repeated_substring_pattern_detects_repeated_unit() {
    assert!(repeated_substring_pattern("abcabcabcabc"));
}

#[test]
fn repeated_substring_pattern_rejects_non_repeated_string() {
    assert!(!repeated_substring_pattern("aba"));
}

#[test]
fn repeated_substring_pattern_rejects_single_character() {
    assert!(!repeated_substring_pattern("a"));
}

#[test]
fn longest_common_prefix_returns_shared_prefix() {
    let words = vec![
        "flower".to_string(),
        "flow".to_string(),
        "flight".to_string(),
    ];

    assert_eq!(longest_common_prefix(words), "fl");
}

#[test]
fn longest_common_prefix_returns_empty_when_no_prefix_exists() {
    let words = vec!["dog".to_string(), "racecar".to_string(), "car".to_string()];

    assert_eq!(longest_common_prefix(words), "");
}

#[test]
fn longest_common_prefix_handles_empty_input() {
    assert_eq!(longest_common_prefix(Vec::new()), "");
}

#[test]
fn longest_duplicate_substring_returns_longest_duplicate() {
    assert_eq!(longest_duplicate_substring("banana"), "ana");
}

#[test]
fn longest_duplicate_substring_returns_empty_when_missing() {
    assert_eq!(longest_duplicate_substring("abcd"), "");
}

#[test]
fn longest_duplicate_substring_handles_repeated_character_runs() {
    assert_eq!(longest_duplicate_substring("aaaa"), "aaa");
}

#[test]
fn longest_duplicate_substring_handles_empty_and_single_character_inputs() {
    assert_eq!(longest_duplicate_substring(""), "");
    assert_eq!(longest_duplicate_substring("z"), "");
}
