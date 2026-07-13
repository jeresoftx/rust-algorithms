/// Subsets
///
/// Pattern: backtracking decision tree.
/// Idea: for each value, choose whether it belongs to the current subset.
///
/// Time: O(n * 2^n)
/// Space: O(n) recursion depth, excluding returned subsets.
pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut result = Vec::new();
    let mut current = Vec::new();

    build_subsets(0, &nums, &mut current, &mut result);

    result
}

fn build_subsets(index: usize, nums: &[i32], current: &mut Vec<i32>, result: &mut Vec<Vec<i32>>) {
    if index == nums.len() {
        result.push(current.clone());
        return;
    }

    build_subsets(index + 1, nums, current, result);

    current.push(nums[index]);
    build_subsets(index + 1, nums, current, result);
    current.pop();
}

/// Permutations
///
/// Pattern: backtracking with used markers.
/// Idea: each depth chooses one unused value for the current position.
///
/// Time: O(n * n!)
/// Space: O(n)
pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut result = Vec::new();
    let mut current = Vec::new();
    let mut used = vec![false; nums.len()];

    build_permutations(&nums, &mut used, &mut current, &mut result);

    result
}

fn build_permutations(
    nums: &[i32],
    used: &mut [bool],
    current: &mut Vec<i32>,
    result: &mut Vec<Vec<i32>>,
) {
    if current.len() == nums.len() {
        result.push(current.clone());
        return;
    }

    for index in 0..nums.len() {
        if used[index] {
            continue;
        }

        used[index] = true;
        current.push(nums[index]);
        build_permutations(nums, used, current, result);
        current.pop();
        used[index] = false;
    }
}

/// Combination Sum
///
/// Pattern: backtracking with pruning.
/// Idea: sorted candidates allow reuse of the current index and early stopping.
///
/// Time: exponential in target/candidates
/// Space: O(target / min_candidate)
pub fn combination_sum(mut candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    candidates.sort_unstable();
    let mut result = Vec::new();
    let mut current = Vec::new();

    build_combinations(0, target, &candidates, &mut current, &mut result);

    result
}

fn build_combinations(
    start: usize,
    remaining: i32,
    candidates: &[i32],
    current: &mut Vec<i32>,
    result: &mut Vec<Vec<i32>>,
) {
    if remaining == 0 {
        result.push(current.clone());
        return;
    }

    for index in start..candidates.len() {
        let candidate = candidates[index];

        if candidate > remaining {
            break;
        }

        current.push(candidate);
        build_combinations(index, remaining - candidate, candidates, current, result);
        current.pop();
    }
}

/// Generate Parentheses
///
/// Pattern: constrained backtracking.
/// Idea: add an opening parenthesis if available; add a closing one if valid.
///
/// Time: O(Cn), where Cn is the nth Catalan number.
/// Space: O(n)
pub fn generate_parentheses(pairs: usize) -> Vec<String> {
    let mut result = Vec::new();
    let mut current = String::new();

    build_parentheses(pairs, 0, 0, &mut current, &mut result);

    result
}

fn build_parentheses(
    pairs: usize,
    open_count: usize,
    close_count: usize,
    current: &mut String,
    result: &mut Vec<String>,
) {
    if current.len() == pairs * 2 {
        result.push(current.clone());
        return;
    }

    if open_count < pairs {
        current.push('(');
        build_parentheses(pairs, open_count + 1, close_count, current, result);
        current.pop();
    }

    if close_count < open_count {
        current.push(')');
        build_parentheses(pairs, open_count, close_count + 1, current, result);
        current.pop();
    }
}

/// Word Search
///
/// Pattern: DFS backtracking on a grid.
/// Idea: match one character per cell and mark the cell as visited during path exploration.
///
/// Time: O(rows * cols * 4^word_len)
/// Space: O(word_len)
pub fn word_search(mut board: Vec<Vec<char>>, word: &str) -> bool {
    if word.is_empty() {
        return true;
    }

    if board.is_empty() || board[0].is_empty() {
        return false;
    }

    let word: Vec<char> = word.chars().collect();

    for row in 0..board.len() {
        for col in 0..board[0].len() {
            if search_from_cell(row, col, 0, &word, &mut board) {
                return true;
            }
        }
    }

    false
}

fn search_from_cell(
    row: usize,
    col: usize,
    word_index: usize,
    word: &[char],
    board: &mut [Vec<char>],
) -> bool {
    if board[row][col] != word[word_index] {
        return false;
    }

    if word_index == word.len() - 1 {
        return true;
    }

    let original = board[row][col];
    board[row][col] = '\0';

    let rows = board.len();
    let cols = board[0].len();
    let found = (row > 0 && search_from_cell(row - 1, col, word_index + 1, word, board))
        || (row + 1 < rows && search_from_cell(row + 1, col, word_index + 1, word, board))
        || (col > 0 && search_from_cell(row, col - 1, word_index + 1, word, board))
        || (col + 1 < cols && search_from_cell(row, col + 1, word_index + 1, word, board));

    board[row][col] = original;

    found
}

/// Combination Sum II
///
/// Pattern: backtracking with duplicate skipping.
/// Idea: each candidate can be used once; equal values at the same depth are skipped.
///
/// Time: exponential
/// Space: O(n)
pub fn combination_sum_ii(mut candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    candidates.sort_unstable();
    let mut result = Vec::new();
    let mut current = Vec::new();

    build_unique_combinations(0, target, &candidates, &mut current, &mut result);

    result
}

fn build_unique_combinations(
    start: usize,
    remaining: i32,
    candidates: &[i32],
    current: &mut Vec<i32>,
    result: &mut Vec<Vec<i32>>,
) {
    if remaining == 0 {
        result.push(current.clone());
        return;
    }

    for index in start..candidates.len() {
        if index > start && candidates[index] == candidates[index - 1] {
            continue;
        }

        let candidate = candidates[index];
        if candidate > remaining {
            break;
        }

        current.push(candidate);
        build_unique_combinations(
            index + 1,
            remaining - candidate,
            candidates,
            current,
            result,
        );
        current.pop();
    }
}

/// Palindrome Partitioning
///
/// Pattern: backtracking over cut positions.
/// Idea: every prefix chosen must be a palindrome before recursing on the suffix.
///
/// Time: O(n * 2^n)
/// Space: O(n)
pub fn palindrome_partitioning(input: &str) -> Vec<Vec<String>> {
    let chars: Vec<char> = input.chars().collect();
    let mut result = Vec::new();
    let mut current = Vec::new();

    build_palindrome_partitions(0, &chars, &mut current, &mut result);

    result
}

fn build_palindrome_partitions(
    start: usize,
    chars: &[char],
    current: &mut Vec<String>,
    result: &mut Vec<Vec<String>>,
) {
    if start == chars.len() {
        result.push(current.clone());
        return;
    }

    for end in start..chars.len() {
        if is_palindrome(chars, start, end) {
            current.push(chars[start..=end].iter().collect());
            build_palindrome_partitions(end + 1, chars, current, result);
            current.pop();
        }
    }
}

fn is_palindrome(chars: &[char], mut left: usize, mut right: usize) -> bool {
    while left < right {
        if chars[left] != chars[right] {
            return false;
        }

        left += 1;
        right -= 1;
    }

    true
}

/// Letter Combinations of a Phone Number
///
/// Pattern: Cartesian product backtracking.
/// Idea: each digit expands to its mapped letters; unsupported digits make no output.
///
/// Time: O(4^n)
/// Space: O(n)
pub fn letter_combinations(digits: &str) -> Vec<String> {
    if digits.is_empty() {
        return Vec::new();
    }

    let mut groups = Vec::new();
    for digit in digits.bytes() {
        let letters = phone_letters(digit);
        if letters.is_empty() {
            return Vec::new();
        }
        groups.push(letters);
    }

    let mut result = Vec::new();
    let mut current = String::new();
    build_letter_combinations(0, &groups, &mut current, &mut result);
    result
}

fn phone_letters(digit: u8) -> &'static [char] {
    match digit {
        b'2' => &['a', 'b', 'c'],
        b'3' => &['d', 'e', 'f'],
        b'4' => &['g', 'h', 'i'],
        b'5' => &['j', 'k', 'l'],
        b'6' => &['m', 'n', 'o'],
        b'7' => &['p', 'q', 'r', 's'],
        b'8' => &['t', 'u', 'v'],
        b'9' => &['w', 'x', 'y', 'z'],
        _ => &[],
    }
}

fn build_letter_combinations(
    index: usize,
    groups: &[&[char]],
    current: &mut String,
    result: &mut Vec<String>,
) {
    if index == groups.len() {
        result.push(current.clone());
        return;
    }

    for &letter in groups[index] {
        current.push(letter);
        build_letter_combinations(index + 1, groups, current, result);
        current.pop();
    }
}

/// N-Queens
///
/// Pattern: row-by-row constrained backtracking.
/// Idea: sets for columns and diagonals make conflict checks O(1).
///
/// Time: O(n!)
/// Space: O(n)
pub fn n_queens_solutions(size: usize) -> Vec<Vec<String>> {
    let mut board = vec![vec!['.'; size]; size];
    let mut used_cols = vec![false; size];
    let mut used_diag_down = vec![false; size * 2];
    let mut used_diag_up = vec![false; size * 2];
    let mut result = Vec::new();

    build_n_queens(
        0,
        &mut board,
        &mut used_cols,
        &mut used_diag_down,
        &mut used_diag_up,
        &mut result,
    );

    result
}

fn build_n_queens(
    row: usize,
    board: &mut [Vec<char>],
    used_cols: &mut [bool],
    used_diag_down: &mut [bool],
    used_diag_up: &mut [bool],
    result: &mut Vec<Vec<String>>,
) {
    let size = board.len();
    if row == size {
        result.push(board.iter().map(|row| row.iter().collect()).collect());
        return;
    }

    for col in 0..size {
        let down = row + col;
        let up = row + size - 1 - col;

        if used_cols[col] || used_diag_down[down] || used_diag_up[up] {
            continue;
        }

        used_cols[col] = true;
        used_diag_down[down] = true;
        used_diag_up[up] = true;
        board[row][col] = 'Q';

        build_n_queens(
            row + 1,
            board,
            used_cols,
            used_diag_down,
            used_diag_up,
            result,
        );

        board[row][col] = '.';
        used_cols[col] = false;
        used_diag_down[down] = false;
        used_diag_up[up] = false;
    }
}

/// Subsets II
///
/// Pattern: sorted backtracking with duplicate skipping.
/// Idea: values equal to the previous one are skipped at the same decision depth.
///
/// Time: O(n * 2^n)
/// Space: O(n)
pub fn subsets_with_dup(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
    nums.sort_unstable();
    let mut result = Vec::new();
    let mut current = Vec::new();

    build_unique_subsets(0, &nums, &mut current, &mut result);

    result
}

fn build_unique_subsets(
    start: usize,
    nums: &[i32],
    current: &mut Vec<i32>,
    result: &mut Vec<Vec<i32>>,
) {
    result.push(current.clone());

    for index in start..nums.len() {
        if index > start && nums[index] == nums[index - 1] {
            continue;
        }

        current.push(nums[index]);
        build_unique_subsets(index + 1, nums, current, result);
        current.pop();
    }
}
