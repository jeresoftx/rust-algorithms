use std::collections::HashMap;

/// Rotate Image
///
/// Pattern: transpose then reverse rows.
/// Idea: rotating 90 degrees clockwise is equivalent to swapping across the
/// main diagonal and then reversing each row.
///
/// Time: O(n^2)
/// Space: O(1)
pub fn rotate_image(matrix: &mut [Vec<i32>]) -> bool {
    let size = matrix.len();
    if size == 0 || matrix.iter().any(|row| row.len() != size) {
        return false;
    }

    for row in 0..size {
        for col in row + 1..size {
            let temp = matrix[row][col];
            matrix[row][col] = matrix[col][row];
            matrix[col][row] = temp;
        }
    }

    for row in matrix {
        row.reverse();
    }

    true
}

/// Spiral Matrix
///
/// Pattern: shrinking boundaries.
/// Idea: consume top row, right column, bottom row and left column, then shrink.
///
/// Time: O(m * n)
/// Space: O(1) excluding output
pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
    if matrix.is_empty() || matrix[0].is_empty() {
        return Vec::new();
    }

    let mut top = 0_i32;
    let mut bottom = matrix.len() as i32 - 1;
    let mut left = 0_i32;
    let mut right = matrix[0].len() as i32 - 1;
    let mut result = Vec::with_capacity(matrix.len() * matrix[0].len());

    while top <= bottom && left <= right {
        for col in left..=right {
            result.push(matrix[top as usize][col as usize]);
        }
        top += 1;

        for row in top..=bottom {
            result.push(matrix[row as usize][right as usize]);
        }
        right -= 1;

        if top <= bottom {
            for col in (left..=right).rev() {
                result.push(matrix[bottom as usize][col as usize]);
            }
            bottom -= 1;
        }

        if left <= right {
            for row in (top..=bottom).rev() {
                result.push(matrix[row as usize][left as usize]);
            }
            left += 1;
        }
    }

    result
}

/// Set Matrix Zeroes
///
/// Pattern: first row and first column as markers.
/// Idea: remember whether the marker row/column originally had zeroes.
///
/// Time: O(m * n)
/// Space: O(1)
pub fn set_matrix_zeroes(matrix: &mut [Vec<i32>]) -> bool {
    if matrix.is_empty() || matrix[0].is_empty() {
        return false;
    }

    let rows = matrix.len();
    let cols = matrix[0].len();
    if matrix.iter().any(|row| row.len() != cols) {
        return false;
    }

    let first_row_zero = matrix[0].contains(&0);
    let first_col_zero = matrix.iter().any(|row| row[0] == 0);

    for row in 1..rows {
        for col in 1..cols {
            if matrix[row][col] == 0 {
                matrix[row][0] = 0;
                matrix[0][col] = 0;
            }
        }
    }

    for row in 1..rows {
        for col in 1..cols {
            if matrix[row][0] == 0 || matrix[0][col] == 0 {
                matrix[row][col] = 0;
            }
        }
    }

    if first_row_zero {
        matrix[0].fill(0);
    }

    if first_col_zero {
        for row in matrix {
            row[0] = 0;
        }
    }

    true
}

/// Valid Sudoku
///
/// Pattern: bit masks for rows, columns and boxes.
/// Idea: each digit sets one bit in each constraint group.
///
/// Time: O(1), fixed 9x9 board
/// Space: O(1)
pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
    if board.len() != 9 || board.iter().any(|row| row.len() != 9) {
        return false;
    }

    let mut rows = [0_u16; 9];
    let mut cols = [0_u16; 9];
    let mut boxes = [0_u16; 9];

    for row in 0..9 {
        for col in 0..9 {
            let value = board[row][col];
            if value == '.' {
                continue;
            }

            let Some(digit) = value.to_digit(10) else {
                return false;
            };
            if digit == 0 {
                return false;
            }

            let mask = 1_u16 << digit;
            let box_index = (row / 3) * 3 + (col / 3);

            if rows[row] & mask != 0 || cols[col] & mask != 0 || boxes[box_index] & mask != 0 {
                return false;
            }

            rows[row] |= mask;
            cols[col] |= mask;
            boxes[box_index] |= mask;
        }
    }

    true
}

/// Game of Life
///
/// Pattern: in-place state encoding.
/// Idea: store transitional states so neighbor reads still see the original board.
///
/// Time: O(m * n)
/// Space: O(1)
pub fn game_of_life(board: &mut [Vec<i32>]) -> bool {
    if board.is_empty() || board[0].is_empty() {
        return false;
    }

    let rows = board.len();
    let cols = board[0].len();
    if board.iter().any(|row| row.len() != cols) {
        return false;
    }

    for row in 0..rows {
        for col in 0..cols {
            let live_neighbors = count_live_neighbors(board, row, col);

            if board[row][col] == 1 && !(2..=3).contains(&live_neighbors) {
                board[row][col] = -1;
            } else if board[row][col] == 0 && live_neighbors == 3 {
                board[row][col] = 2;
            }
        }
    }

    for row in board {
        for cell in row {
            *cell = if *cell > 0 { 1 } else { 0 };
        }
    }

    true
}

fn count_live_neighbors(board: &[Vec<i32>], row: usize, col: usize) -> i32 {
    let rows = board.len() as i32;
    let cols = board[0].len() as i32;
    let mut total = 0;

    for row_delta in -1..=1 {
        for col_delta in -1..=1 {
            if row_delta == 0 && col_delta == 0 {
                continue;
            }

            let next_row = row as i32 + row_delta;
            let next_col = col as i32 + col_delta;

            if (0..rows).contains(&next_row)
                && (0..cols).contains(&next_col)
                && board[next_row as usize][next_col as usize].abs() == 1
            {
                total += 1;
            }
        }
    }

    total
}

/// Maximal Square
///
/// Pattern: 2D DP with row compression.
/// Idea: a `1` cell extends the minimum of top, left and diagonal squares.
///
/// Time: O(m * n)
/// Space: O(n)
pub fn maximal_square(matrix: Vec<Vec<char>>) -> i32 {
    if matrix.is_empty() || matrix[0].is_empty() {
        return 0;
    }

    let cols = matrix[0].len();
    let mut dp = vec![0; cols + 1];
    let mut best = 0;

    for row in matrix {
        let mut diagonal = 0;
        for col in 0..cols {
            let top = dp[col + 1];
            dp[col + 1] = if row[col] == '1' {
                1 + dp[col].min(dp[col + 1]).min(diagonal)
            } else {
                0
            };
            diagonal = top;
            best = best.max(dp[col + 1]);
        }
    }

    best * best
}

/// Search a 2D Matrix II
///
/// Pattern: staircase search.
/// Idea: start at top-right; move left when too large and down when too small.
///
/// Time: O(m + n)
/// Space: O(1)
pub fn search_matrix_ii(matrix: Vec<Vec<i32>>, target: i32) -> bool {
    if matrix.is_empty() || matrix[0].is_empty() {
        return false;
    }

    let rows = matrix.len();
    let cols = matrix[0].len();
    let mut row = 0;
    let mut col = cols as i32 - 1;

    while row < rows && col >= 0 {
        let value = matrix[row][col as usize];
        if value == target {
            return true;
        }

        if value > target {
            col -= 1;
        } else {
            row += 1;
        }
    }

    false
}

/// Insert Delete GetRandom O(1)
///
/// Pattern: vector plus index map.
/// Idea: removal swaps the deleted value with the last vector slot.
///
/// Time: O(1) average for insert, remove and get_random
/// Space: O(n)
#[derive(Debug, Clone)]
pub struct RandomizedSet {
    values: Vec<i32>,
    indexes: HashMap<i32, usize>,
    state: u64,
}

impl RandomizedSet {
    pub fn new() -> Self {
        Self {
            values: Vec::new(),
            indexes: HashMap::new(),
            state: 0x9E37_79B9_7F4A_7C15,
        }
    }

    pub fn insert(&mut self, value: i32) -> bool {
        if self.indexes.contains_key(&value) {
            return false;
        }

        self.indexes.insert(value, self.values.len());
        self.values.push(value);
        true
    }

    pub fn remove(&mut self, value: i32) -> bool {
        let Some(index) = self.indexes.remove(&value) else {
            return false;
        };

        let last_value = self.values.pop().expect("index map pointed to a value");
        if index < self.values.len() {
            self.values[index] = last_value;
            self.indexes.insert(last_value, index);
        }

        true
    }

    pub fn get_random(&mut self) -> Option<i32> {
        if self.values.is_empty() {
            return None;
        }

        self.state ^= self.state << 7;
        self.state ^= self.state >> 9;
        let index = self.state as usize % self.values.len();
        Some(self.values[index])
    }

    pub fn len(&self) -> usize {
        self.values.len()
    }

    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }
}

impl Default for RandomizedSet {
    fn default() -> Self {
        Self::new()
    }
}
