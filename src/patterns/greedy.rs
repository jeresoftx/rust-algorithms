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
