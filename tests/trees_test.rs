use rust_algorithms::patterns::trees::{
    build_tree_inorder_postorder, build_tree_preorder_inorder, deserialize_tree,
    diameter_of_binary_tree, has_path_sum, invert_tree, is_balanced, is_same_tree, is_subtree,
    is_valid_bst, kth_smallest_bst, level_order, lowest_common_ancestor_bst, max_depth,
    path_sum_ii, right_side_view, serialize_tree, tree_from_level_order, tree_to_level_order,
};

#[test]
fn max_depth_returns_longest_root_to_leaf_path() {
    let tree = tree_from_level_order(vec![
        Some(3),
        Some(9),
        Some(20),
        None,
        None,
        Some(15),
        Some(7),
    ]);

    assert_eq!(max_depth(tree), 3);
}

#[test]
fn max_depth_returns_zero_for_empty_tree() {
    assert_eq!(max_depth(None), 0);
}

#[test]
fn invert_tree_swaps_children_recursively() {
    let tree = tree_from_level_order(vec![
        Some(4),
        Some(2),
        Some(7),
        Some(1),
        Some(3),
        Some(6),
        Some(9),
    ]);

    let result = invert_tree(tree);

    assert_eq!(
        tree_to_level_order(result),
        vec![
            Some(4),
            Some(7),
            Some(2),
            Some(9),
            Some(6),
            Some(3),
            Some(1)
        ]
    );
}

#[test]
fn diameter_of_binary_tree_counts_longest_edge_path() {
    let tree = tree_from_level_order(vec![Some(1), Some(2), Some(3), Some(4), Some(5)]);

    assert_eq!(diameter_of_binary_tree(tree), 3);
}

#[test]
fn is_balanced_returns_true_for_balanced_tree() {
    let tree = tree_from_level_order(vec![
        Some(3),
        Some(9),
        Some(20),
        None,
        None,
        Some(15),
        Some(7),
    ]);

    assert!(is_balanced(tree));
}

#[test]
fn is_balanced_returns_false_for_unbalanced_tree() {
    let tree = tree_from_level_order(vec![
        Some(1),
        Some(2),
        Some(2),
        Some(3),
        Some(3),
        None,
        None,
        Some(4),
        Some(4),
    ]);

    assert!(!is_balanced(tree));
}

#[test]
fn is_same_tree_compares_structure_and_values() {
    let left = tree_from_level_order(vec![Some(1), Some(2), Some(3)]);
    let right = tree_from_level_order(vec![Some(1), Some(2), Some(3)]);

    assert!(is_same_tree(left, right));
}

#[test]
fn is_same_tree_returns_false_for_different_structure() {
    let left = tree_from_level_order(vec![Some(1), Some(2)]);
    let right = tree_from_level_order(vec![Some(1), None, Some(2)]);

    assert!(!is_same_tree(left, right));
}

#[test]
fn is_subtree_returns_true_when_subtree_exists() {
    let root = tree_from_level_order(vec![Some(3), Some(4), Some(5), Some(1), Some(2)]);
    let sub_root = tree_from_level_order(vec![Some(4), Some(1), Some(2)]);

    assert!(is_subtree(root, sub_root));
}

#[test]
fn is_subtree_returns_false_when_structure_differs() {
    let root = tree_from_level_order(vec![
        Some(3),
        Some(4),
        Some(5),
        Some(1),
        Some(2),
        None,
        None,
        None,
        None,
        Some(0),
    ]);
    let sub_root = tree_from_level_order(vec![Some(4), Some(1), Some(2)]);

    assert!(!is_subtree(root, sub_root));
}

#[test]
fn level_order_returns_values_by_depth() {
    let tree = tree_from_level_order(vec![
        Some(3),
        Some(9),
        Some(20),
        None,
        None,
        Some(15),
        Some(7),
    ]);

    assert_eq!(level_order(tree), vec![vec![3], vec![9, 20], vec![15, 7]]);
}

#[test]
fn is_valid_bst_returns_true_for_valid_tree() {
    let tree = tree_from_level_order(vec![Some(2), Some(1), Some(3)]);

    assert!(is_valid_bst(tree));
}

#[test]
fn is_valid_bst_returns_false_for_invalid_descendant() {
    let tree = tree_from_level_order(vec![
        Some(5),
        Some(1),
        Some(4),
        None,
        None,
        Some(3),
        Some(6),
    ]);

    assert!(!is_valid_bst(tree));
}

#[test]
fn is_valid_bst_rejects_duplicate_values_on_either_side() {
    let duplicate_on_left = tree_from_level_order(vec![Some(2), Some(2), Some(3)]);
    let duplicate_on_right = tree_from_level_order(vec![Some(2), Some(1), Some(2)]);

    assert!(!is_valid_bst(duplicate_on_left));
    assert!(!is_valid_bst(duplicate_on_right));
}

#[test]
fn lowest_common_ancestor_bst_returns_split_node() {
    let tree = tree_from_level_order(vec![
        Some(6),
        Some(2),
        Some(8),
        Some(0),
        Some(4),
        Some(7),
        Some(9),
        None,
        None,
        Some(3),
        Some(5),
    ]);

    assert_eq!(lowest_common_ancestor_bst(tree, 2, 8), Some(6));
}

#[test]
fn lowest_common_ancestor_bst_returns_ancestor_node() {
    let tree = tree_from_level_order(vec![
        Some(6),
        Some(2),
        Some(8),
        Some(0),
        Some(4),
        Some(7),
        Some(9),
        None,
        None,
        Some(3),
        Some(5),
    ]);

    assert_eq!(lowest_common_ancestor_bst(tree, 2, 4), Some(2));
}

#[test]
fn build_tree_preorder_inorder_reconstructs_tree() {
    let tree = build_tree_preorder_inorder(vec![3, 9, 20, 15, 7], vec![9, 3, 15, 20, 7]);

    assert_eq!(
        tree_to_level_order(tree),
        vec![Some(3), Some(9), Some(20), None, None, Some(15), Some(7)]
    );
}

#[test]
fn build_tree_preorder_inorder_handles_empty_input() {
    assert_eq!(
        tree_to_level_order(build_tree_preorder_inorder(vec![], vec![])),
        Vec::<Option<i32>>::new()
    );
}

#[test]
fn kth_smallest_bst_returns_ordered_rank() {
    let tree = tree_from_level_order(vec![
        Some(5),
        Some(3),
        Some(6),
        Some(2),
        Some(4),
        None,
        None,
        Some(1),
    ]);

    assert_eq!(kth_smallest_bst(tree, 3), Some(3));
}

#[test]
fn kth_smallest_bst_returns_none_when_rank_is_out_of_bounds() {
    let tree = tree_from_level_order(vec![Some(2), Some(1), Some(3)]);

    assert_eq!(kth_smallest_bst(tree, 0), None);
}

#[test]
fn right_side_view_keeps_last_node_seen_per_level() {
    let tree = tree_from_level_order(vec![
        Some(1),
        Some(2),
        Some(3),
        None,
        Some(5),
        None,
        Some(4),
    ]);

    assert_eq!(right_side_view(tree), vec![1, 3, 4]);
}

#[test]
fn right_side_view_handles_empty_tree() {
    assert_eq!(right_side_view(None), Vec::<i32>::new());
}

#[test]
fn has_path_sum_accepts_root_to_leaf_sum() {
    let tree = tree_from_level_order(vec![
        Some(5),
        Some(4),
        Some(8),
        Some(11),
        None,
        Some(13),
        Some(4),
        Some(7),
        Some(2),
        None,
        None,
        None,
        Some(1),
    ]);

    assert!(has_path_sum(tree, 22));
}

#[test]
fn has_path_sum_rejects_partial_path_sum() {
    let tree = tree_from_level_order(vec![Some(1), Some(2), Some(3)]);

    assert!(!has_path_sum(tree, 5));
}

#[test]
fn path_sum_ii_returns_all_root_to_leaf_paths() {
    let tree = tree_from_level_order(vec![
        Some(5),
        Some(4),
        Some(8),
        Some(11),
        None,
        Some(13),
        Some(4),
        Some(7),
        Some(2),
        None,
        None,
        Some(5),
        Some(1),
    ]);

    assert_eq!(
        path_sum_ii(tree, 22),
        vec![vec![5, 4, 11, 2], vec![5, 8, 4, 5]]
    );
}

#[test]
fn path_sum_ii_returns_empty_when_no_leaf_path_matches() {
    let tree = tree_from_level_order(vec![Some(1), Some(2), Some(3)]);

    assert_eq!(path_sum_ii(tree, 5), Vec::<Vec<i32>>::new());
}

#[test]
fn serialize_tree_round_trips_sparse_tree() {
    let tree = tree_from_level_order(vec![
        Some(1),
        Some(2),
        Some(3),
        None,
        None,
        Some(4),
        Some(5),
    ]);

    let encoded = serialize_tree(tree);
    let decoded = deserialize_tree(&encoded);

    assert_eq!(
        tree_to_level_order(decoded),
        vec![Some(1), Some(2), Some(3), None, None, Some(4), Some(5)]
    );
}

#[test]
fn deserialize_tree_handles_empty_payload() {
    assert_eq!(
        tree_to_level_order(deserialize_tree("")),
        Vec::<Option<i32>>::new()
    );
}

#[test]
fn build_tree_inorder_postorder_reconstructs_tree() {
    let tree = build_tree_inorder_postorder(vec![9, 3, 15, 20, 7], vec![9, 15, 7, 20, 3]);

    assert_eq!(
        tree_to_level_order(tree),
        vec![Some(3), Some(9), Some(20), None, None, Some(15), Some(7)]
    );
}

#[test]
fn build_tree_inorder_postorder_handles_empty_input() {
    assert_eq!(
        tree_to_level_order(build_tree_inorder_postorder(vec![], vec![])),
        Vec::<Option<i32>>::new()
    );
}
