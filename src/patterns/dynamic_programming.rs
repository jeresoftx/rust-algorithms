use std::collections::HashSet;

/// Climbing Stairs
///
/// Pattern: 1D DP with state compression.
/// Idea: ways(n) = ways(n - 1) + ways(n - 2).
///
/// Time: O(n)
/// Space: O(1)
pub fn climb_stairs(n: usize) -> i32 {
    let mut previous = 1;
    let mut current = 1;

    for _ in 0..n {
        let next = previous + current;
        previous = current;
        current = next;
    }

    previous
}

/// House Robber
///
/// Pattern: 1D decision DP with state compression.
/// Idea: at each house, either rob it plus the best two houses back, or skip it.
///
/// Time: O(n)
/// Space: O(1)
pub fn house_robber(nums: Vec<i32>) -> i32 {
    house_robber_linear(&nums)
}

/// Min Cost Climbing Stairs
///
/// Pattern: 1D minimization DP with state compression.
/// Idea: cost to reach the top comes from one or two steps below.
///
/// Time: O(n)
/// Space: O(1)
pub fn min_cost_climbing_stairs(cost: Vec<i32>) -> i32 {
    if cost.len() < 2 {
        return 0;
    }

    let mut two_back = 0;
    let mut one_back = 0;

    for step in 2..=cost.len() {
        let current = (one_back + cost[step - 1]).min(two_back + cost[step - 2]);
        two_back = one_back;
        one_back = current;
    }

    one_back
}

/// House Robber II
///
/// Pattern: split circular dependency into two linear ranges.
/// Idea: the first and last houses cannot both be selected.
///
/// Time: O(n)
/// Space: O(1)
pub fn house_robber_circular(nums: Vec<i32>) -> i32 {
    match nums.len() {
        0 => 0,
        1 => nums[0],
        _ => house_robber_linear(&nums[..nums.len() - 1]).max(house_robber_linear(&nums[1..])),
    }
}

fn house_robber_linear(nums: &[i32]) -> i32 {
    let mut skip_previous = 0;
    let mut best_previous = 0;

    for &value in nums {
        let best_current = best_previous.max(skip_previous + value);
        skip_previous = best_previous;
        best_previous = best_current;
    }

    best_previous
}

/// Coin Change
///
/// Pattern: 1D minimization DP.
/// Idea: dp[amount] is the fewest coins needed to build that amount.
///
/// Time: O(amount * coins)
/// Space: O(amount)
pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
    if amount < 0 {
        return -1;
    }

    let amount = amount as usize;
    let unreachable = amount as i32 + 1;
    let mut dp = vec![unreachable; amount + 1];
    dp[0] = 0;

    for current_amount in 1..=amount {
        for &coin in &coins {
            if coin <= 0 {
                continue;
            }

            let coin = coin as usize;
            if coin <= current_amount {
                dp[current_amount] = dp[current_amount].min(dp[current_amount - coin] + 1);
            }
        }
    }

    if dp[amount] == unreachable {
        -1
    } else {
        dp[amount]
    }
}

/// Decode Ways
///
/// Pattern: 1D DP with state compression.
/// Idea: each position can consume one valid digit or two valid digits.
///
/// Time: O(n)
/// Space: O(1)
pub fn decode_ways(input: &str) -> i32 {
    if input.is_empty() {
        return 0;
    }

    let bytes = input.as_bytes();
    let mut two_back = 1;
    let mut one_back = if bytes[0] == b'0' { 0 } else { 1 };

    for index in 1..bytes.len() {
        let mut current = 0;

        if bytes[index] != b'0' {
            current += one_back;
        }

        let two_digit = (bytes[index - 1] - b'0') * 10 + (bytes[index] - b'0');
        if (10..=26).contains(&two_digit) {
            current += two_back;
        }

        two_back = one_back;
        one_back = current;
    }

    one_back
}

/// Longest Increasing Subsequence
///
/// Pattern: patience sorting with binary search.
/// Idea: tails[len] stores the smallest possible tail for an increasing
/// subsequence of length len + 1.
///
/// Time: O(n log n)
/// Space: O(n)
pub fn longest_increasing_subsequence(nums: Vec<i32>) -> usize {
    let mut tails: Vec<i32> = Vec::new();

    for value in nums {
        match tails.binary_search(&value) {
            Ok(index) | Err(index) => {
                if index == tails.len() {
                    tails.push(value);
                } else {
                    tails[index] = value;
                }
            }
        }
    }

    tails.len()
}

/// Word Break
///
/// Pattern: decision DP over prefixes.
/// Idea: dp[i] is true when s[..i] can be segmented into dictionary words.
///
/// Time: O(n^2)
/// Space: O(n + d)
pub fn word_break(input: &str, word_dict: Vec<&str>) -> bool {
    let words: HashSet<&str> = word_dict.into_iter().collect();
    let mut dp = vec![false; input.len() + 1];
    dp[0] = true;

    for end in 1..=input.len() {
        for start in 0..end {
            if dp[start] && words.contains(&input[start..end]) {
                dp[end] = true;
                break;
            }
        }
    }

    dp[input.len()]
}

/// Unique Paths
///
/// Pattern: 2D grid DP with state compression.
/// Idea: paths to a cell equal paths from top plus paths from left.
///
/// Time: O(m * n)
/// Space: O(n)
pub fn unique_paths(rows: usize, cols: usize) -> i32 {
    if rows == 0 || cols == 0 {
        return 0;
    }

    let mut dp = vec![1; cols];

    for _ in 1..rows {
        for col in 1..cols {
            dp[col] += dp[col - 1];
        }
    }

    dp[cols - 1]
}

/// Longest Common Subsequence
///
/// Pattern: 2D string DP with state compression.
/// Idea: matching characters extend a subsequence; otherwise keep the best
/// result from skipping one character from either string.
///
/// Time: O(m * n)
/// Space: O(n)
pub fn longest_common_subsequence(first: &str, second: &str) -> usize {
    let first: Vec<char> = first.chars().collect();
    let second: Vec<char> = second.chars().collect();
    let mut previous = vec![0; second.len() + 1];

    for first_char in first {
        let mut current = vec![0; second.len() + 1];

        for (index, &second_char) in second.iter().enumerate() {
            current[index + 1] = if first_char == second_char {
                previous[index] + 1
            } else {
                previous[index + 1].max(current[index])
            };
        }

        previous = current;
    }

    previous[second.len()]
}

/// Edit Distance
///
/// Pattern: 2D string DP with row compression.
/// Idea: replace, insert and delete each depend on adjacent solved prefixes.
///
/// Time: O(m * n)
/// Space: O(n)
pub fn edit_distance(first: &str, second: &str) -> usize {
    let first: Vec<char> = first.chars().collect();
    let second: Vec<char> = second.chars().collect();
    let mut previous: Vec<usize> = (0..=second.len()).collect();

    for (first_index, &first_char) in first.iter().enumerate() {
        let mut current = vec![first_index + 1; second.len() + 1];

        for (second_index, &second_char) in second.iter().enumerate() {
            current[second_index + 1] = if first_char == second_char {
                previous[second_index]
            } else {
                1 + previous[second_index]
                    .min(previous[second_index + 1])
                    .min(current[second_index])
            };
        }

        previous = current;
    }

    previous[second.len()]
}

/// Longest Palindromic Subsequence
///
/// Pattern: interval DP.
/// Idea: matching ends extend the best inner palindrome.
///
/// Time: O(n^2)
/// Space: O(n^2)
pub fn longest_palindromic_subsequence(input: &str) -> usize {
    let chars: Vec<char> = input.chars().collect();
    let length = chars.len();

    if length == 0 {
        return 0;
    }

    let mut dp = vec![vec![0; length]; length];

    for left in (0..length).rev() {
        dp[left][left] = 1;

        for right in left + 1..length {
            dp[left][right] = if chars[left] == chars[right] {
                2 + dp[left + 1][right - 1]
            } else {
                dp[left + 1][right].max(dp[left][right - 1])
            };
        }
    }

    dp[0][length - 1]
}

/// Partition Equal Subset Sum
///
/// Pattern: 0/1 knapsack decision DP.
/// Idea: determine whether any subset reaches half of the total sum.
///
/// Time: O(n * target)
/// Space: O(target)
pub fn can_partition(nums: Vec<i32>) -> bool {
    let sum: i32 = nums.iter().sum();
    if sum % 2 != 0 || sum < 0 {
        return false;
    }

    let target = (sum / 2) as usize;
    let mut dp = vec![false; target + 1];
    dp[0] = true;

    for value in nums {
        if value < 0 {
            return false;
        }

        let value = value as usize;
        for current in (value..=target).rev() {
            dp[current] = dp[current] || dp[current - value];
        }
    }

    dp[target]
}
