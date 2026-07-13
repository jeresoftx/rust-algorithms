use rust_algorithms::patterns::greedy::{can_complete_circuit, jump_game_ii, task_scheduler};

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

#[test]
fn jump_game_ii_returns_minimum_number_of_jumps() {
    assert_eq!(jump_game_ii(vec![2, 3, 1, 1, 4]), Some(2));
}

#[test]
fn jump_game_ii_returns_none_when_end_is_unreachable() {
    assert_eq!(jump_game_ii(vec![3, 2, 1, 0, 4]), None);
}

#[test]
fn can_complete_circuit_returns_valid_start_index() {
    assert_eq!(
        can_complete_circuit(vec![1, 2, 3, 4, 5], vec![3, 4, 5, 1, 2]),
        Some(3)
    );
}

#[test]
fn can_complete_circuit_returns_none_when_total_gas_is_insufficient() {
    assert_eq!(can_complete_circuit(vec![2, 3, 4], vec![3, 4, 3]), None);
}
