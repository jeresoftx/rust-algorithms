use rust_algorithms::patterns::two_pointers::{
    max_area, move_zeroes, remove_duplicates_sorted, sorted_squares, three_sum, two_sum_sorted,
    valid_palindrome,
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
fn valid_palindrome_accepts_empty_cleaned_text() {
    assert!(valid_palindrome(".,"));
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
