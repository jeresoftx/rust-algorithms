use rust_algorithms::patterns::recursion_backtracking::{
    combination_sum, generate_parentheses, permute, subsets, word_search,
};

#[test]
fn subsets_returns_all_subsets() {
    let mut result = subsets(vec![1, 2, 3]);
    result.sort();

    assert_eq!(
        result,
        vec![
            vec![],
            vec![1],
            vec![1, 2],
            vec![1, 2, 3],
            vec![1, 3],
            vec![2],
            vec![2, 3],
            vec![3],
        ]
    );
}

#[test]
fn subsets_handles_empty_input() {
    assert_eq!(subsets(vec![]), vec![Vec::<i32>::new()]);
}

#[test]
fn permute_returns_all_permutations() {
    let mut result = permute(vec![1, 2, 3]);
    result.sort();

    assert_eq!(
        result,
        vec![
            vec![1, 2, 3],
            vec![1, 3, 2],
            vec![2, 1, 3],
            vec![2, 3, 1],
            vec![3, 1, 2],
            vec![3, 2, 1],
        ]
    );
}

#[test]
fn permute_handles_single_value() {
    assert_eq!(permute(vec![7]), vec![vec![7]]);
}

#[test]
fn combination_sum_returns_unique_combinations() {
    let mut result = combination_sum(vec![2, 3, 6, 7], 7);
    result.sort();

    assert_eq!(result, vec![vec![2, 2, 3], vec![7]]);
}

#[test]
fn combination_sum_returns_empty_when_target_cannot_be_reached() {
    assert!(combination_sum(vec![5, 10], 3).is_empty());
}

#[test]
fn generate_parentheses_returns_valid_combinations() {
    let mut result = generate_parentheses(3);
    result.sort();

    assert_eq!(
        result,
        vec!["((()))", "(()())", "(())()", "()(())", "()()()"]
    );
}

#[test]
fn generate_parentheses_handles_zero_pairs() {
    assert_eq!(generate_parentheses(0), vec![String::new()]);
}

#[test]
fn word_search_returns_true_when_word_exists() {
    let board = vec![
        vec!['A', 'B', 'C', 'E'],
        vec!['S', 'F', 'C', 'S'],
        vec!['A', 'D', 'E', 'E'],
    ];

    assert!(word_search(board, "ABCCED"));
}

#[test]
fn word_search_returns_false_when_cell_reuse_would_be_required() {
    let board = vec![
        vec!['A', 'B', 'C', 'E'],
        vec!['S', 'F', 'C', 'S'],
        vec!['A', 'D', 'E', 'E'],
    ];

    assert!(!word_search(board, "ABCB"));
}
