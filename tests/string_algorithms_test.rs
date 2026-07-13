use rust_algorithms::patterns::string_algorithms::{
    count_palindromic_substrings, find_anagram_starts, find_multi_pattern_positions,
    find_pattern_positions, longest_common_prefix, longest_duplicate_substring,
    longest_palindromic_substring, rabin_karp_positions, repeated_substring_pattern,
    shortest_palindrome, z_function,
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
fn rabin_karp_positions_returns_all_matches() {
    assert_eq!(rabin_karp_positions("abracadabra", "abra"), vec![0, 7]);
}

#[test]
fn rabin_karp_positions_handles_overlapping_matches() {
    assert_eq!(rabin_karp_positions("aaaaa", "aaa"), vec![0, 1, 2]);
}

#[test]
fn rabin_karp_positions_returns_empty_for_empty_or_missing_pattern() {
    assert!(rabin_karp_positions("abc", "").is_empty());
    assert!(rabin_karp_positions("abc", "z").is_empty());
}

#[test]
fn z_function_computes_prefix_match_lengths() {
    assert_eq!(
        z_function("aabcaabxaaaz"),
        vec![0, 1, 0, 0, 3, 1, 0, 0, 2, 2, 1, 0]
    );
}

#[test]
fn z_function_handles_empty_input() {
    assert!(z_function("").is_empty());
}

#[test]
fn find_multi_pattern_positions_returns_all_overlapping_matches() {
    let matches = find_multi_pattern_positions("ahishers", vec!["he", "she", "his", "hers"]);

    assert_eq!(
        matches,
        vec![
            ("he".to_string(), vec![4]),
            ("she".to_string(), vec![3]),
            ("his".to_string(), vec![1]),
            ("hers".to_string(), vec![4]),
        ]
    );
}

#[test]
fn find_multi_pattern_positions_keeps_patterns_with_no_matches() {
    let matches = find_multi_pattern_positions("aaaa", vec!["a", "aa", "b", ""]);

    assert_eq!(
        matches,
        vec![
            ("a".to_string(), vec![0, 1, 2, 3]),
            ("aa".to_string(), vec![0, 1, 2]),
            ("b".to_string(), Vec::<usize>::new()),
            ("".to_string(), Vec::<usize>::new()),
        ]
    );
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

#[test]
fn longest_palindromic_substring_returns_longest_center_expansion_match() {
    let result = longest_palindromic_substring("babad");

    assert!(result == "bab" || result == "aba");
}

#[test]
fn longest_palindromic_substring_handles_even_length_palindrome() {
    assert_eq!(longest_palindromic_substring("cbbd"), "bb");
}

#[test]
fn count_palindromic_substrings_counts_all_centers() {
    assert_eq!(count_palindromic_substrings("aaa"), 6);
    assert_eq!(count_palindromic_substrings("abc"), 3);
}

#[test]
fn count_palindromic_substrings_handles_empty_input() {
    assert_eq!(count_palindromic_substrings(""), 0);
}

#[test]
fn shortest_palindrome_adds_minimum_prefix() {
    assert_eq!(shortest_palindrome("aacecaaa"), "aaacecaaa");
    assert_eq!(shortest_palindrome("abcd"), "dcbabcd");
}

#[test]
fn shortest_palindrome_handles_already_palindromic_or_empty_input() {
    assert_eq!(shortest_palindrome("aba"), "aba");
    assert_eq!(shortest_palindrome(""), "");
}
