use rust_algorithms::patterns::recursion_backtracking::{
    combination_sum, combination_sum_ii, generate_parentheses, letter_combinations,
    n_queens_solutions, palindrome_partitioning, permute, subsets, subsets_with_dup, word_search,
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

#[test]
fn combination_sum_ii_returns_unique_combinations_without_reuse() {
    let mut result = combination_sum_ii(vec![10, 1, 2, 7, 6, 1, 5], 8);
    result.sort();

    assert_eq!(
        result,
        vec![vec![1, 1, 6], vec![1, 2, 5], vec![1, 7], vec![2, 6]]
    );
}

#[test]
fn combination_sum_ii_skips_duplicate_combinations() {
    assert_eq!(combination_sum_ii(vec![1, 1, 1], 2), vec![vec![1, 1]]);
}

#[test]
fn palindrome_partitioning_returns_all_palindrome_cuts() {
    let mut result = palindrome_partitioning("aab");
    result.sort();

    assert_eq!(
        result,
        vec![
            vec!["a".to_string(), "a".to_string(), "b".to_string()],
            vec!["aa".to_string(), "b".to_string()]
        ]
    );
}

#[test]
fn palindrome_partitioning_handles_empty_input() {
    assert_eq!(palindrome_partitioning(""), vec![Vec::<String>::new()]);
}

#[test]
fn letter_combinations_returns_phone_mnemonics() {
    let mut result = letter_combinations("23");
    result.sort();

    assert_eq!(
        result,
        vec!["ad", "ae", "af", "bd", "be", "bf", "cd", "ce", "cf"]
    );
}

#[test]
fn letter_combinations_ignores_digits_without_letters() {
    assert_eq!(letter_combinations("10"), Vec::<String>::new());
}

#[test]
fn n_queens_solutions_returns_all_boards_for_four() {
    let mut result = n_queens_solutions(4);
    result.sort();

    assert_eq!(
        result,
        vec![
            vec![
                "..Q.".to_string(),
                "Q...".to_string(),
                "...Q".to_string(),
                ".Q..".to_string()
            ],
            vec![
                ".Q..".to_string(),
                "...Q".to_string(),
                "Q...".to_string(),
                "..Q.".to_string()
            ],
        ]
    );
}

#[test]
fn n_queens_solutions_handles_single_queen() {
    assert_eq!(n_queens_solutions(1), vec![vec!["Q".to_string()]]);
}

#[test]
fn subsets_with_dup_returns_unique_subsets() {
    let mut result = subsets_with_dup(vec![1, 2, 2]);
    result.sort();

    assert_eq!(
        result,
        vec![
            vec![],
            vec![1],
            vec![1, 2],
            vec![1, 2, 2],
            vec![2],
            vec![2, 2]
        ]
    );
}

#[test]
fn subsets_with_dup_handles_all_duplicates() {
    assert_eq!(
        subsets_with_dup(vec![2, 2]),
        vec![vec![], vec![2], vec![2, 2]]
    );
}
