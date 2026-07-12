use rust_algorithms::patterns::heaps::{kth_largest, MedianFinder};

#[test]
fn kth_largest_returns_second_largest_value() {
    let result = kth_largest(vec![3, 2, 1, 5, 6, 4], 2);

    assert_eq!(result, Some(5));
}

#[test]
fn kth_largest_handles_duplicates() {
    let result = kth_largest(vec![3, 2, 3, 1, 2, 4, 5, 5, 6], 4);

    assert_eq!(result, Some(4));
}

#[test]
fn kth_largest_returns_none_when_k_is_out_of_range() {
    assert_eq!(kth_largest(vec![1, 2], 0), None);
    assert_eq!(kth_largest(vec![1, 2], 3), None);
}

#[test]
fn median_finder_returns_median_after_each_insert() {
    let mut finder = MedianFinder::new();

    finder.add_num(1);
    assert_eq!(finder.find_median(), Some(1.0));

    finder.add_num(2);
    assert_eq!(finder.find_median(), Some(1.5));

    finder.add_num(3);
    assert_eq!(finder.find_median(), Some(2.0));
}

#[test]
fn median_finder_balances_negative_and_positive_values() {
    let mut finder = MedianFinder::new();

    for value in [-5, 10, 3, 4] {
        finder.add_num(value);
    }

    assert_eq!(finder.find_median(), Some(3.5));
}

#[test]
fn median_finder_returns_none_when_empty() {
    let finder = MedianFinder::new();

    assert_eq!(finder.find_median(), None);
}
