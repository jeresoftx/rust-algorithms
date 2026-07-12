use rust_algorithms::patterns::greedy::task_scheduler;

#[test]
fn task_scheduler_returns_minimum_intervals_with_idle_time() {
    let tasks = vec!['A', 'A', 'A', 'B', 'B', 'B'];

    assert_eq!(task_scheduler(tasks, 2), 8);
}

#[test]
fn task_scheduler_returns_task_count_when_no_idle_time_is_needed() {
    let tasks = vec!['A', 'A', 'A', 'B', 'B', 'B'];

    assert_eq!(task_scheduler(tasks, 0), 6);
}

#[test]
fn task_scheduler_handles_multiple_most_frequent_tasks() {
    let tasks = vec!['A', 'A', 'A', 'B', 'B', 'B', 'C', 'C'];

    assert_eq!(task_scheduler(tasks, 2), 8);
}

#[test]
fn task_scheduler_returns_zero_for_empty_tasks() {
    assert_eq!(task_scheduler(Vec::new(), 2), 0);
}
