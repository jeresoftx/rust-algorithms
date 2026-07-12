use rust_algorithms::patterns::stack_queue::{
    daily_temperatures, largest_rectangle_area, valid_parentheses,
};

#[test]
fn valid_parentheses_accepts_balanced_brackets() {
    assert!(valid_parentheses("()[]{}"));
}

#[test]
fn valid_parentheses_rejects_wrong_closing_order() {
    assert!(!valid_parentheses("(]"));
}

#[test]
fn valid_parentheses_rejects_unclosed_opening_bracket() {
    assert!(!valid_parentheses("([]"));
}

#[test]
fn daily_temperatures_returns_days_until_warmer_temperature() {
    let result = daily_temperatures(vec![73, 74, 75, 71, 69, 72, 76, 73]);

    assert_eq!(result, vec![1, 1, 4, 2, 1, 1, 0, 0]);
}

#[test]
fn daily_temperatures_returns_zero_when_no_warmer_day_exists() {
    let result = daily_temperatures(vec![80, 79, 78]);

    assert_eq!(result, vec![0, 0, 0]);
}

#[test]
fn daily_temperatures_handles_single_temperature() {
    assert_eq!(daily_temperatures(vec![70]), vec![0]);
}

#[test]
fn largest_rectangle_area_returns_best_histogram_rectangle() {
    assert_eq!(largest_rectangle_area(vec![2, 1, 5, 6, 2, 3]), 10);
}

#[test]
fn largest_rectangle_area_handles_increasing_heights() {
    assert_eq!(largest_rectangle_area(vec![1, 2, 3, 4]), 6);
}

#[test]
fn largest_rectangle_area_returns_zero_for_empty_histogram() {
    assert_eq!(largest_rectangle_area(vec![]), 0);
}
