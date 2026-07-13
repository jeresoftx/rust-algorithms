use std::collections::HashMap;

/// Task Scheduler
///
/// Pattern: greedy by maximum frequency.
/// Idea: the most frequent task defines the minimum frame length.
///
/// Time: O(n)
/// Space: O(k), where k is the number of distinct tasks.
pub fn task_scheduler(tasks: Vec<char>, cooldown: i32) -> i32 {
    if tasks.is_empty() {
        return 0;
    }

    let mut counts = HashMap::new();
    for task in &tasks {
        *counts.entry(*task).or_insert(0) += 1;
    }

    let max_count = counts.values().copied().max().unwrap_or(0);
    let most_frequent_tasks = counts.values().filter(|&&count| count == max_count).count() as i32;
    let frame_length = (max_count - 1) * (cooldown + 1) + most_frequent_tasks;

    frame_length.max(tasks.len() as i32)
}

/// Jump Game II
///
/// Pattern: greedy frontier expansion.
/// Idea: each jump covers the current window and chooses the farthest next window.
///
/// Time: O(n)
/// Space: O(1)
pub fn jump_game_ii(nums: Vec<i32>) -> Option<i32> {
    if nums.len() <= 1 {
        return Some(0);
    }

    let mut jumps = 0;
    let mut current_end = 0;
    let mut farthest = 0;

    for index in 0..nums.len() - 1 {
        farthest = farthest.max(index + nums[index].max(0) as usize);

        if index == current_end {
            if farthest == current_end {
                return None;
            }

            jumps += 1;
            current_end = farthest;

            if current_end >= nums.len() - 1 {
                return Some(jumps);
            }
        }
    }

    Some(jumps)
}

/// Gas Station
///
/// Pattern: greedy reset on negative tank.
/// Idea: if a segment cannot reach the current station, no index inside it can
/// be the valid start.
///
/// Time: O(n)
/// Space: O(1)
pub fn can_complete_circuit(gas: Vec<i32>, cost: Vec<i32>) -> Option<usize> {
    if gas.len() != cost.len() || gas.is_empty() {
        return None;
    }

    let mut total = 0;
    let mut tank = 0;
    let mut start = 0;

    for index in 0..gas.len() {
        let balance = gas[index] - cost[index];
        total += balance;
        tank += balance;

        if tank < 0 {
            start = index + 1;
            tank = 0;
        }
    }

    if total >= 0 && start < gas.len() {
        Some(start)
    } else {
        None
    }
}
