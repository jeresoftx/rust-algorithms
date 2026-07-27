use rust_algorithms::patterns::two_pointers::{
    is_subsequence, max_area, merge_sorted_array, move_zeroes, remove_duplicates_sorted,
    remove_duplicates_sorted_at_most_twice, sort_colors, sorted_squares, three_sum,
    trap_rain_water, two_sum_sorted, valid_palindrome, valid_palindrome_with_one_removal,
};

#[test]
fn valid_palindrome_ignores_case_and_non_alphanumeric_characters() {
    assert!(valid_palindrome("A man, a plan, a canal: Panama"));
}

#[test]
fn valid_palindrome_returns_false_when_cleaned_text_is_not_palindrome() {
    assert!(!valid_palindrome("race a car"));
}

#[test]
fn trap_rain_water_counts_basins_between_boundaries() {
    assert_eq!(trap_rain_water(&[0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1]), 6);
    assert_eq!(trap_rain_water(&[]), 0);
}

#[test]
fn valid_palindrome_accepts_empty_cleaned_text() {
    assert!(valid_palindrome(".,"));
}

#[test]
fn valid_palindrome_with_one_removal_accepts_already_valid_text() {
    assert!(valid_palindrome_with_one_removal("racecar"));
}

#[test]
fn valid_palindrome_with_one_removal_accepts_left_or_right_removal() {
    assert!(valid_palindrome_with_one_removal("abca"));
    assert!(valid_palindrome_with_one_removal("deeee"));
}

#[test]
fn valid_palindrome_with_one_removal_rejects_two_required_removals() {
    assert!(!valid_palindrome_with_one_removal("abc"));
}

#[test]
fn valid_palindrome_with_one_removal_handles_short_text() {
    assert!(valid_palindrome_with_one_removal("a"));
    assert!(valid_palindrome_with_one_removal(""));
}

#[test]
fn is_subsequence_accepts_characters_in_relative_order() {
    assert!(is_subsequence("ace", "abcde"));
}

#[test]
fn is_subsequence_rejects_missing_or_reordered_characters() {
    assert!(!is_subsequence("aec", "abcde"));
    assert!(!is_subsequence("axc", "abcde"));
}

#[test]
fn is_subsequence_handles_empty_inputs() {
    assert!(is_subsequence("", "abc"));
    assert!(!is_subsequence("a", ""));
}

#[test]
fn is_subsequence_advances_past_repeated_text_characters() {
    assert!(is_subsequence("aab", "aaab"));
    assert!(!is_subsequence("aaaa", "aaab"));
}

#[test]
fn merge_sorted_array_merges_interleaved_values_in_place() {
    let mut destination = vec![1, 2, 3, 0, 0, 0];
    merge_sorted_array(&mut destination, 3, &[2, 5, 6]);
    assert_eq!(destination, vec![1, 2, 2, 3, 5, 6]);
}

#[test]
fn merge_sorted_array_handles_empty_source_or_destination_prefix() {
    let mut destination = vec![1];
    merge_sorted_array(&mut destination, 1, &[]);
    assert_eq!(destination, vec![1]);

    let mut destination = vec![0];
    merge_sorted_array(&mut destination, 0, &[1]);
    assert_eq!(destination, vec![1]);
}

#[test]
fn merge_sorted_array_preserves_duplicate_values() {
    let mut destination = vec![1, 2, 2, 0, 0];
    merge_sorted_array(&mut destination, 3, &[2, 2]);
    assert_eq!(destination, vec![1, 2, 2, 2, 2]);
}

#[test]
fn remove_duplicates_sorted_at_most_twice_keeps_two_copies() {
    assert_eq!(
        remove_duplicates_sorted_at_most_twice(vec![0, 0, 1, 1, 1, 1, 2, 3, 3]),
        vec![0, 0, 1, 1, 2, 3, 3]
    );
}

#[test]
fn remove_duplicates_sorted_at_most_twice_handles_short_inputs() {
    assert_eq!(
        remove_duplicates_sorted_at_most_twice(vec![]),
        Vec::<i32>::new()
    );
    assert_eq!(remove_duplicates_sorted_at_most_twice(vec![4]), vec![4]);
    assert_eq!(
        remove_duplicates_sorted_at_most_twice(vec![4, 4]),
        vec![4, 4]
    );
}

#[test]
fn remove_duplicates_sorted_at_most_twice_keeps_distinct_values() {
    assert_eq!(
        remove_duplicates_sorted_at_most_twice(vec![1, 2, 3]),
        vec![1, 2, 3]
    );
}

#[test]
fn sort_colors_partitions_all_three_values() {
    assert_eq!(sort_colors(vec![2, 0, 2, 1, 1, 0]), vec![0, 0, 1, 1, 2, 2]);
}

#[test]
fn sort_colors_handles_repeated_and_minimal_inputs() {
    assert_eq!(sort_colors(vec![2, 2, 2]), vec![2, 2, 2]);
    assert_eq!(sort_colors(vec![1]), vec![1]);
    assert!(sort_colors(Vec::new()).is_empty());
}

#[test]
fn three_sum_returns_unique_triplets_that_sum_to_zero() {
    let mut result = three_sum(vec![-1, 0, 1, 2, -1, -4]);
    result.sort();

    assert_eq!(result, vec![vec![-1, -1, 2], vec![-1, 0, 1]]);
}

#[test]
fn three_sum_returns_empty_when_no_triplet_matches() {
    assert!(three_sum(vec![1, 2, -2, -1]).is_empty());
}

#[test]
fn three_sum_deduplicates_repeated_zero_triplet() {
    assert_eq!(three_sum(vec![0, 0, 0, 0]), vec![vec![0, 0, 0]]);
}

#[test]
fn max_area_returns_largest_container_area() {
    assert_eq!(max_area(vec![1, 8, 6, 2, 5, 4, 8, 3, 7]), 49);
}

#[test]
fn max_area_handles_two_lines() {
    assert_eq!(max_area(vec![1, 1]), 1);
}

#[test]
fn max_area_returns_zero_when_there_are_not_enough_lines() {
    assert_eq!(max_area(vec![4]), 0);
}

#[test]
fn two_sum_sorted_returns_pair_indices_for_target() {
    assert_eq!(two_sum_sorted(vec![2, 7, 11, 15], 9), Some((0, 1)));
}

#[test]
fn two_sum_sorted_returns_none_when_pair_is_absent() {
    assert_eq!(two_sum_sorted(vec![1, 2, 3, 9], 8), None);
}

#[test]
fn remove_duplicates_sorted_keeps_one_copy_per_value() {
    assert_eq!(
        remove_duplicates_sorted(vec![0, 0, 1, 1, 1, 2, 2]),
        vec![0, 1, 2]
    );
}

#[test]
fn remove_duplicates_sorted_handles_empty_input() {
    assert!(remove_duplicates_sorted(Vec::new()).is_empty());
}

#[test]
fn move_zeroes_preserves_relative_order_of_non_zero_values() {
    assert_eq!(move_zeroes(vec![0, 1, 0, 3, 12]), vec![1, 3, 12, 0, 0]);
}

#[test]
fn move_zeroes_handles_all_zeroes() {
    assert_eq!(move_zeroes(vec![0, 0]), vec![0, 0]);
}

#[test]
fn sorted_squares_returns_squares_in_non_decreasing_order() {
    assert_eq!(
        sorted_squares(vec![-4, -1, 0, 3, 10]),
        vec![0, 1, 9, 16, 100]
    );
}

#[test]
fn sorted_squares_handles_all_negative_values() {
    assert_eq!(sorted_squares(vec![-7, -3, -1]), vec![1, 9, 49]);
}
