use rust_algorithms::patterns::binary_search::{
    binary_search, find_min_rotated, min_eating_speed, search_insert, search_rotated,
    ship_within_days,
};

#[test]
fn binary_search_returns_index_when_target_exists() {
    assert_eq!(binary_search(vec![-1, 0, 3, 5, 9, 12], 9), Some(4));
}

#[test]
fn binary_search_returns_none_when_target_is_absent() {
    assert_eq!(binary_search(vec![-1, 0, 3, 5, 9, 12], 2), None);
}

#[test]
fn binary_search_handles_empty_input() {
    assert_eq!(binary_search(vec![], 1), None);
}

#[test]
fn search_insert_returns_existing_index() {
    assert_eq!(search_insert(vec![1, 3, 5, 6], 5), 2);
}

#[test]
fn search_insert_returns_position_between_values() {
    assert_eq!(search_insert(vec![1, 3, 5, 6], 2), 1);
}

#[test]
fn search_insert_returns_position_at_end() {
    assert_eq!(search_insert(vec![1, 3, 5, 6], 7), 4);
}

#[test]
fn search_rotated_returns_index_when_target_exists() {
    assert_eq!(search_rotated(vec![4, 5, 6, 7, 0, 1, 2], 0), Some(4));
}

#[test]
fn search_rotated_returns_none_when_target_is_absent() {
    assert_eq!(search_rotated(vec![4, 5, 6, 7, 0, 1, 2], 3), None);
}

#[test]
fn search_rotated_handles_single_value() {
    assert_eq!(search_rotated(vec![1], 1), Some(0));
}

#[test]
fn find_min_rotated_returns_minimum_after_rotation() {
    assert_eq!(find_min_rotated(vec![3, 4, 5, 1, 2]), Some(1));
}

#[test]
fn find_min_rotated_returns_first_value_when_not_rotated() {
    assert_eq!(find_min_rotated(vec![1, 2, 3, 4]), Some(1));
}

#[test]
fn find_min_rotated_returns_none_for_empty_input() {
    assert_eq!(find_min_rotated(vec![]), None);
}

#[test]
fn min_eating_speed_returns_minimum_speed_to_finish_in_time() {
    assert_eq!(min_eating_speed(vec![3, 6, 7, 11], 8), 4);
}

#[test]
fn min_eating_speed_handles_large_piles_and_tight_hours() {
    assert_eq!(min_eating_speed(vec![30, 11, 23, 4, 20], 5), 30);
}

#[test]
fn min_eating_speed_returns_zero_for_empty_piles() {
    assert_eq!(min_eating_speed(vec![], 8), 0);
}

#[test]
fn ship_within_days_returns_minimum_capacity() {
    assert_eq!(ship_within_days(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10], 5), 15);
}

#[test]
fn ship_within_days_handles_tight_single_day_deadline() {
    assert_eq!(ship_within_days(vec![3, 2, 2, 4, 1, 4], 3), 6);
}

#[test]
fn ship_within_days_returns_zero_for_empty_weights() {
    assert_eq!(ship_within_days(vec![], 3), 0);
}
