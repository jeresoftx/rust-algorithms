use rust_algorithms::patterns::sliding_window::{
    length_of_longest_substring, max_profit, min_window,
};

#[test]
fn max_profit_returns_best_single_buy_sell_profit() {
    assert_eq!(max_profit(vec![7, 1, 5, 3, 6, 4]), 5);
}

#[test]
fn max_profit_returns_zero_when_price_never_increases() {
    assert_eq!(max_profit(vec![7, 6, 4, 3, 1]), 0);
}

#[test]
fn max_profit_returns_zero_for_empty_prices() {
    assert_eq!(max_profit(vec![]), 0);
}

#[test]
fn length_of_longest_substring_counts_longest_window_without_repeats() {
    assert_eq!(length_of_longest_substring("abcabcbb"), 3);
}

#[test]
fn length_of_longest_substring_handles_repeated_single_character() {
    assert_eq!(length_of_longest_substring("bbbbb"), 1);
}

#[test]
fn length_of_longest_substring_handles_empty_string() {
    assert_eq!(length_of_longest_substring(""), 0);
}

#[test]
fn min_window_returns_shortest_substring_covering_target_counts() {
    assert_eq!(min_window("ADOBECODEBANC", "ABC"), "BANC");
}

#[test]
fn min_window_returns_empty_when_target_cannot_be_covered() {
    assert_eq!(min_window("a", "aa"), "");
}

#[test]
fn min_window_handles_single_character_match() {
    assert_eq!(min_window("a", "a"), "a");
}

#[test]
fn min_window_respects_repeated_target_characters() {
    assert_eq!(min_window("AAABBC", "AABC"), "AABBC");
}

#[test]
fn min_window_returns_empty_for_empty_target() {
    assert_eq!(min_window("abc", ""), "");
}
