use rust_algorithms::patterns::trees::{
    diameter_of_binary_tree, invert_tree, is_balanced, is_same_tree, is_subtree, level_order,
    max_depth, tree_from_level_order, tree_to_level_order,
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
