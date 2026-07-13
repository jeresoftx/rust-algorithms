use rust_algorithms::patterns::dynamic_programming::{
    can_partition, climb_stairs, coin_change, decode_ways, edit_distance, house_robber,
    house_robber_circular, longest_common_subsequence, longest_increasing_subsequence,
    longest_palindromic_subsequence, min_cost_climbing_stairs, unique_paths, word_break,
};

#[test]
fn climb_stairs_counts_ways_to_reach_top() {
    assert_eq!(climb_stairs(2), 2);
    assert_eq!(climb_stairs(5), 8);
}

#[test]
fn climb_stairs_handles_zero_steps() {
    assert_eq!(climb_stairs(0), 1);
}

#[test]
fn house_robber_returns_best_non_adjacent_sum() {
    assert_eq!(house_robber(vec![1, 2, 3, 1]), 4);
    assert_eq!(house_robber(vec![2, 7, 9, 3, 1]), 12);
}

#[test]
fn house_robber_handles_empty_input() {
    assert_eq!(house_robber(Vec::new()), 0);
}

#[test]
fn min_cost_climbing_stairs_returns_cheapest_top_cost() {
    assert_eq!(min_cost_climbing_stairs(vec![10, 15, 20]), 15);
    assert_eq!(
        min_cost_climbing_stairs(vec![1, 100, 1, 1, 1, 100, 1, 1, 100, 1]),
        6
    );
}

#[test]
fn min_cost_climbing_stairs_handles_short_inputs() {
    assert_eq!(min_cost_climbing_stairs(vec![7]), 0);
    assert_eq!(min_cost_climbing_stairs(Vec::new()), 0);
}

#[test]
fn house_robber_circular_returns_best_non_adjacent_sum_in_cycle() {
    assert_eq!(house_robber_circular(vec![2, 3, 2]), 3);
    assert_eq!(house_robber_circular(vec![1, 2, 3, 1]), 4);
}

#[test]
fn house_robber_circular_handles_small_inputs() {
    assert_eq!(house_robber_circular(vec![5]), 5);
    assert_eq!(house_robber_circular(Vec::new()), 0);
}

#[test]
fn coin_change_returns_minimum_number_of_coins() {
    assert_eq!(coin_change(vec![1, 2, 5], 11), 3);
}

#[test]
fn coin_change_returns_minus_one_when_amount_cannot_be_formed() {
    assert_eq!(coin_change(vec![2], 3), -1);
}

#[test]
fn coin_change_returns_zero_for_zero_amount() {
    assert_eq!(coin_change(vec![1, 2, 5], 0), 0);
}

#[test]
fn decode_ways_counts_valid_decodings() {
    assert_eq!(decode_ways("12"), 2);
    assert_eq!(decode_ways("226"), 3);
}

#[test]
fn decode_ways_rejects_invalid_zero_prefix() {
    assert_eq!(decode_ways("06"), 0);
}

#[test]
fn decode_ways_handles_embedded_zero() {
    assert_eq!(decode_ways("2101"), 1);
}

#[test]
fn longest_increasing_subsequence_returns_best_length() {
    assert_eq!(
        longest_increasing_subsequence(vec![10, 9, 2, 5, 3, 7, 101, 18]),
        4
    );
}

#[test]
fn longest_increasing_subsequence_handles_decreasing_input() {
    assert_eq!(longest_increasing_subsequence(vec![5, 4, 3, 2, 1]), 1);
}

#[test]
fn word_break_returns_true_when_string_can_be_segmented() {
    assert!(word_break("leetcode", vec!["leet", "code"]));
    assert!(word_break("applepenapple", vec!["apple", "pen"]));
}

#[test]
fn word_break_returns_false_when_suffix_cannot_be_segmented() {
    assert!(!word_break(
        "catsandog",
        vec!["cats", "dog", "sand", "and", "cat"]
    ));
}

#[test]
fn unique_paths_counts_paths_in_grid() {
    assert_eq!(unique_paths(3, 7), 28);
    assert_eq!(unique_paths(3, 2), 3);
}

#[test]
fn unique_paths_returns_zero_for_empty_dimension() {
    assert_eq!(unique_paths(0, 3), 0);
}

#[test]
fn longest_common_subsequence_returns_shared_subsequence_length() {
    assert_eq!(longest_common_subsequence("abcde", "ace"), 3);
}

#[test]
fn longest_common_subsequence_returns_zero_when_no_characters_match() {
    assert_eq!(longest_common_subsequence("abc", "def"), 0);
}

#[test]
fn edit_distance_counts_minimum_insert_delete_replace_operations() {
    assert_eq!(edit_distance("horse", "ros"), 3);
    assert_eq!(edit_distance("intention", "execution"), 5);
}

#[test]
fn edit_distance_handles_empty_side() {
    assert_eq!(edit_distance("", "abc"), 3);
    assert_eq!(edit_distance("abc", ""), 3);
}

#[test]
fn longest_palindromic_subsequence_returns_best_length() {
    assert_eq!(longest_palindromic_subsequence("bbbab"), 4);
    assert_eq!(longest_palindromic_subsequence("cbbd"), 2);
}

#[test]
fn longest_palindromic_subsequence_handles_empty_input() {
    assert_eq!(longest_palindromic_subsequence(""), 0);
}

#[test]
fn can_partition_returns_true_when_array_can_split_equal_sum() {
    assert!(can_partition(vec![1, 5, 11, 5]));
}

#[test]
fn can_partition_returns_false_when_total_sum_is_odd() {
    assert!(!can_partition(vec![1, 2, 3, 5]));
}

#[test]
fn can_partition_does_not_reuse_values_more_than_once() {
    assert!(!can_partition(vec![1, 2, 5]));
    assert!(can_partition(vec![0, 0]));
}
