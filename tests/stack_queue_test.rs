use rust_algorithms::patterns::stack_queue::{
    backspace_compare, baseball_game, daily_temperatures, largest_rectangle_area,
    valid_parentheses, StockSpanner,
};

#[test]
fn backspace_compare_accepts_equal_effective_text() {
    assert!(backspace_compare("ab#c", "ad#c"));
}

#[test]
fn backspace_compare_rejects_different_effective_text() {
    assert!(!backspace_compare("a#c", "b"));
}

#[test]
fn backspace_compare_handles_chained_and_leading_backspaces() {
    assert!(backspace_compare("ab##", "c#d#"));
    assert!(backspace_compare("##a", "a"));
}

#[test]
fn baseball_game_sums_a_composed_history() {
    assert_eq!(baseball_game(&["5", "2", "C", "D", "+"]), Some(30));
}

#[test]
fn baseball_game_handles_duplicate_and_sum_operations() {
    assert_eq!(
        baseball_game(&["5", "-2", "4", "C", "D", "9", "+", "+"]),
        Some(27)
    );
}

#[test]
fn baseball_game_rejects_operations_without_required_history() {
    assert_eq!(baseball_game(&["C"]), None);
    assert_eq!(baseball_game(&["5", "+"]), None);
    assert_eq!(baseball_game(&["invalid"]), None);
}

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
fn stock_spanner_accumulates_spans_for_a_typical_sequence() {
    let mut spanner = StockSpanner::new();

    let spans: Vec<usize> = [100, 80, 60, 70, 60, 75, 85]
        .into_iter()
        .map(|price| spanner.next(price))
        .collect();

    assert_eq!(spans, vec![1, 1, 1, 2, 1, 4, 6]);
}

#[test]
fn stock_spanner_handles_equal_prices_and_descending_state() {
    let mut spanner = StockSpanner::new();

    assert_eq!(spanner.next(50), 1);
    assert_eq!(spanner.next(50), 2);
    assert_eq!(spanner.next(40), 1);
    assert_eq!(spanner.next(30), 1);
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
