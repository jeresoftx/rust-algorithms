use rust_algorithms::patterns::hashing::{
    contains_duplicate, group_anagrams, two_sum, valid_anagram,
};

#[test]
fn two_sum_returns_indices_for_pair_that_adds_to_target() {
    let nums = vec![2, 7, 11, 15];

    let result = two_sum(nums, 9);

    assert_eq!(result, Some((0, 1)));
}

#[test]
fn two_sum_returns_indices_when_pair_is_not_adjacent() {
    let nums = vec![3, 2, 4];

    let result = two_sum(nums, 6);

    assert_eq!(result, Some((1, 2)));
}

#[test]
fn two_sum_returns_none_when_no_pair_matches_target() {
    let nums = vec![1, 2, 3];

    let result = two_sum(nums, 7);

    assert_eq!(result, None);
}

#[test]
fn valid_anagram_returns_true_for_same_character_counts() {
    assert!(valid_anagram("anagram", "nagaram"));
}

#[test]
fn valid_anagram_returns_false_for_different_character_counts() {
    assert!(!valid_anagram("rat", "car"));
}

#[test]
fn valid_anagram_returns_false_for_different_lengths() {
    assert!(!valid_anagram("ab", "a"));
}

#[test]
fn contains_duplicate_returns_true_when_value_repeats() {
    assert!(contains_duplicate(vec![1, 2, 3, 1]));
}

#[test]
fn contains_duplicate_returns_false_when_all_values_are_unique() {
    assert!(!contains_duplicate(vec![1, 2, 3, 4]));
}

#[test]
fn group_anagrams_groups_words_with_matching_character_counts() {
    let words = vec![
        String::from("eat"),
        String::from("tea"),
        String::from("tan"),
        String::from("ate"),
        String::from("nat"),
        String::from("bat"),
    ];

    let mut result = group_anagrams(words);
    for group in &mut result {
        group.sort();
    }
    result.sort();

    assert_eq!(
        result,
        vec![
            vec![
                String::from("ate"),
                String::from("eat"),
                String::from("tea")
            ],
            vec![String::from("bat")],
            vec![String::from("nat"), String::from("tan")],
        ]
    );
}

#[test]
fn group_anagrams_handles_empty_string_group() {
    let result = group_anagrams(vec![String::from("")]);

    assert_eq!(result, vec![vec![String::from("")]]);
}
