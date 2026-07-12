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
    let mut skip_previous = 0;
    let mut best_previous = 0;

    for value in nums {
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
