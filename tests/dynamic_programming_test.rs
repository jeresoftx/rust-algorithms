use rust_algorithms::patterns::dynamic_programming::{
    climb_stairs, coin_change, decode_ways, house_robber,
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
