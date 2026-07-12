use rust_algorithms::patterns::linked_lists::{
    has_cycle, list_from_vec, list_to_vec, merge_two_lists, reorder_list, reverse_list,
};

#[test]
fn reverse_list_reverses_all_nodes() {
    let list = list_from_vec(vec![1, 2, 3, 4, 5]);

    let result = reverse_list(list);

    assert_eq!(list_to_vec(&result), vec![5, 4, 3, 2, 1]);
}

#[test]
fn reverse_list_handles_empty_list() {
    assert_eq!(list_to_vec(&reverse_list(None)), Vec::<i32>::new());
}

#[test]
fn merge_two_lists_merges_sorted_lists() {
    let left = list_from_vec(vec![1, 2, 4]);
    let right = list_from_vec(vec![1, 3, 4]);

    let result = merge_two_lists(left, right);

    assert_eq!(list_to_vec(&result), vec![1, 1, 2, 3, 4, 4]);
}

#[test]
fn merge_two_lists_handles_one_empty_list() {
    let result = merge_two_lists(None, list_from_vec(vec![0]));

    assert_eq!(list_to_vec(&result), vec![0]);
}

#[test]
fn has_cycle_detects_cycle_in_next_index_representation() {
    let next = vec![Some(1), Some(2), Some(0)];

    assert!(has_cycle(next));
}

#[test]
fn has_cycle_returns_false_for_acyclic_next_index_representation() {
    let next = vec![Some(1), Some(2), None];

    assert!(!has_cycle(next));
}

#[test]
fn reorder_list_interleaves_front_and_back_nodes() {
    let list = list_from_vec(vec![1, 2, 3, 4, 5]);

    let result = reorder_list(list);

    assert_eq!(list_to_vec(&result), vec![1, 5, 2, 4, 3]);
}

#[test]
fn reorder_list_handles_even_length_list() {
    let list = list_from_vec(vec![1, 2, 3, 4]);

    let result = reorder_list(list);

    assert_eq!(list_to_vec(&result), vec![1, 4, 2, 3]);
}
