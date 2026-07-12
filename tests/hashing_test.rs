use rust_algorithms::patterns::hashing::two_sum;

#[test]
fn two_sum_returns_indices_for_pair_that_adds_to_target() {
    let nums = vec![2, 7, 11, 15];

    let result = two_sum(nums, 9);

    assert_eq!(result, Some((0, 1)));
}

#[test]
fn two_sum_returns_indices_when_pair_is_not_adjacent() {
    let nums = vec![3, 2, 4];

    let result = two_sum(nums, 6);

    assert_eq!(result, Some((1, 2)));
}

#[test]
fn two_sum_returns_none_when_no_pair_matches_target() {
    let nums = vec![1, 2, 3];

    let result = two_sum(nums, 7);

    assert_eq!(result, None);
}
