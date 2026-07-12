use rust_algorithms::patterns::two_pointers::{max_area, three_sum, valid_palindrome};

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
