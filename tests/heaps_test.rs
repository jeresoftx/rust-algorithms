use rust_algorithms::patterns::heaps::{
    k_closest_points_heap, kth_largest, last_stone_weight, merge_k_sorted_lists, MedianFinder,
};
use rust_algorithms::patterns::linked_lists::{list_from_vec, list_to_vec};

#[test]
fn kth_largest_returns_second_largest_value() {
    let result = kth_largest(vec![3, 2, 1, 5, 6, 4], 2);

    assert_eq!(result, Some(5));
}

#[test]
fn kth_largest_handles_duplicates() {
    let result = kth_largest(vec![3, 2, 3, 1, 2, 4, 5, 5, 6], 4);

    assert_eq!(result, Some(4));
}

#[test]
fn kth_largest_returns_none_when_k_is_out_of_range() {
    assert_eq!(kth_largest(vec![1, 2], 0), None);
    assert_eq!(kth_largest(vec![1, 2], 3), None);
}

#[test]
fn median_finder_returns_median_after_each_insert() {
    let mut finder = MedianFinder::new();

    finder.add_num(1);
    assert_eq!(finder.find_median(), Some(1.0));

    finder.add_num(2);
    assert_eq!(finder.find_median(), Some(1.5));

    finder.add_num(3);
    assert_eq!(finder.find_median(), Some(2.0));
}

#[test]
fn median_finder_balances_negative_and_positive_values() {
    let mut finder = MedianFinder::new();

    for value in [-5, 10, 3, 4] {
        finder.add_num(value);
    }

    assert_eq!(finder.find_median(), Some(3.5));
}

#[test]
fn median_finder_returns_none_when_empty() {
    let finder = MedianFinder::new();

    assert_eq!(finder.find_median(), None);
}

#[test]
fn merge_k_sorted_lists_merges_values_from_all_lists() {
    let lists = vec![
        list_from_vec(vec![1, 4, 5]),
        list_from_vec(vec![1, 3, 4]),
        list_from_vec(vec![2, 6]),
    ];

    let merged = merge_k_sorted_lists(lists);

    assert_eq!(list_to_vec(&merged), vec![1, 1, 2, 3, 4, 4, 5, 6]);
}

#[test]
fn merge_k_sorted_lists_handles_empty_lists() {
    let lists = vec![None, list_from_vec(vec![0]), None];

    let merged = merge_k_sorted_lists(lists);

    assert_eq!(list_to_vec(&merged), vec![0]);
}

#[test]
fn k_closest_points_heap_keeps_nearest_points() {
    let mut result = k_closest_points_heap(vec![(1, 3), (-2, 2), (5, 8)], 2);
    result.sort_unstable();

    assert_eq!(result, vec![(-2, 2), (1, 3)]);
}

#[test]
fn k_closest_points_heap_returns_all_when_k_exceeds_length() {
    let mut result = k_closest_points_heap(vec![(3, 3), (1, 1)], 5);
    result.sort_unstable();

    assert_eq!(result, vec![(1, 1), (3, 3)]);
}

#[test]
fn last_stone_weight_smashes_two_largest_stones() {
    assert_eq!(last_stone_weight(vec![2, 7, 4, 1, 8, 1]), 1);
}

#[test]
fn last_stone_weight_returns_zero_when_all_stones_cancel() {
    assert_eq!(last_stone_weight(vec![3, 3]), 0);
}
