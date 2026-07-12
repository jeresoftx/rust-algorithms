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
