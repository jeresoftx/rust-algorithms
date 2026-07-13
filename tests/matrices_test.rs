use rust_algorithms::patterns::matrices::{
    game_of_life, is_valid_sudoku, maximal_square, rotate_image, search_matrix_ii,
    set_matrix_zeroes, spiral_order, RandomizedSet,
};

#[test]
fn rotate_image_rotates_square_matrix_in_place() {
    let mut matrix = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];

    assert!(rotate_image(&mut matrix));

    assert_eq!(matrix, vec![vec![7, 4, 1], vec![8, 5, 2], vec![9, 6, 3]]);
}

#[test]
fn rotate_image_rejects_non_square_matrix() {
    let mut matrix = vec![vec![1, 2, 3], vec![4, 5, 6]];

    assert!(!rotate_image(&mut matrix));
}

#[test]
fn spiral_order_reads_matrix_clockwise() {
    let matrix = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];

    assert_eq!(spiral_order(matrix), vec![1, 2, 3, 6, 9, 8, 7, 4, 5]);
}

#[test]
fn spiral_order_handles_single_row() {
    assert_eq!(spiral_order(vec![vec![1, 2, 3]]), vec![1, 2, 3]);
}

#[test]
fn set_matrix_zeroes_zeros_rows_and_columns_in_place() {
    let mut matrix = vec![vec![1, 1, 1], vec![1, 0, 1], vec![1, 1, 1]];

    assert!(set_matrix_zeroes(&mut matrix));

    assert_eq!(matrix, vec![vec![1, 0, 1], vec![0, 0, 0], vec![1, 0, 1]]);
}

#[test]
fn set_matrix_zeroes_handles_empty_matrix() {
    let mut matrix: Vec<Vec<i32>> = Vec::new();

    assert!(!set_matrix_zeroes(&mut matrix));
}

#[test]
fn is_valid_sudoku_accepts_valid_board() {
    let board = sudoku_board([
        "53..7....",
        "6..195...",
        ".98....6.",
        "8...6...3",
        "4..8.3..1",
        "7...2...6",
        ".6....28.",
        "...419..5",
        "....8..79",
    ]);

    assert!(is_valid_sudoku(board));
}

#[test]
fn is_valid_sudoku_rejects_row_column_or_box_duplicates() {
    let board = sudoku_board([
        "83..7....",
        "6..195...",
        ".98....6.",
        "8...6...3",
        "4..8.3..1",
        "7...2...6",
        ".6....28.",
        "...419..5",
        "....8..79",
    ]);

    assert!(!is_valid_sudoku(board));
}

#[test]
fn game_of_life_updates_board_simultaneously() {
    let mut board = vec![vec![0, 1, 0], vec![0, 0, 1], vec![1, 1, 1], vec![0, 0, 0]];

    assert!(game_of_life(&mut board));

    assert_eq!(
        board,
        vec![vec![0, 0, 0], vec![1, 0, 1], vec![0, 1, 1], vec![0, 1, 0]]
    );
}

#[test]
fn game_of_life_rejects_empty_board() {
    let mut board = Vec::new();

    assert!(!game_of_life(&mut board));
}

#[test]
fn maximal_square_returns_largest_square_area() {
    let matrix = vec![
        vec!['1', '0', '1', '0', '0'],
        vec!['1', '0', '1', '1', '1'],
        vec!['1', '1', '1', '1', '1'],
        vec!['1', '0', '0', '1', '0'],
    ];

    assert_eq!(maximal_square(matrix), 4);
}

#[test]
fn maximal_square_returns_zero_when_no_ones_exist() {
    assert_eq!(maximal_square(vec![vec!['0', '0']]), 0);
}

#[test]
fn search_matrix_ii_finds_target_in_sorted_matrix() {
    let matrix = vec![
        vec![1, 4, 7, 11, 15],
        vec![2, 5, 8, 12, 19],
        vec![3, 6, 9, 16, 22],
        vec![10, 13, 14, 17, 24],
        vec![18, 21, 23, 26, 30],
    ];

    assert!(search_matrix_ii(matrix, 5));
}

#[test]
fn search_matrix_ii_returns_false_when_target_is_absent() {
    let matrix = vec![vec![1, 4], vec![2, 5]];

    assert!(!search_matrix_ii(matrix, 3));
}

#[test]
fn randomized_set_supports_insert_remove_and_membership_random() {
    let mut set = RandomizedSet::new();

    assert!(set.insert(1));
    assert!(!set.insert(1));
    assert!(set.insert(2));
    assert!(set.remove(1));

    assert_eq!(set.get_random(), Some(2));
}

#[test]
fn randomized_set_returns_none_when_empty() {
    let mut set = RandomizedSet::new();

    assert_eq!(set.get_random(), None);
}

fn sudoku_board(rows: [&str; 9]) -> Vec<Vec<char>> {
    rows.into_iter().map(|row| row.chars().collect()).collect()
}
