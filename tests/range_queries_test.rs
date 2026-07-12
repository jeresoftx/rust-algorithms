use rust_algorithms::patterns::range_queries::{
    car_pooling, corporate_flight_bookings, count_smaller_numbers_after_self, reverse_pairs,
    DifferenceArray, FenwickTree, MyCalendar, MyCalendarTwo, RangeSumQuery, SegmentTree,
};

#[test]
fn fenwick_tree_returns_prefix_and_range_sums() {
    let tree = FenwickTree::from_values(&[1, 3, 5, 7, 9]);

    assert_eq!(tree.prefix_sum(0), Some(1));
    assert_eq!(tree.prefix_sum(2), Some(9));
    assert_eq!(tree.range_sum(1, 3), Some(15));
}

#[test]
fn fenwick_tree_add_updates_future_sums() {
    let mut tree = FenwickTree::from_values(&[2, 4, 6, 8]);

    assert!(tree.add(1, 5));

    assert_eq!(tree.prefix_sum(1), Some(11));
    assert_eq!(tree.range_sum(1, 3), Some(23));
}

#[test]
fn fenwick_tree_rejects_out_of_range_queries() {
    let mut tree = FenwickTree::from_values(&[4, 1, 9]);

    assert_eq!(tree.prefix_sum(3), None);
    assert_eq!(tree.range_sum(2, 1), None);
    assert!(!tree.add(5, 10));
}

#[test]
fn range_sum_query_updates_values_and_queries_ranges() {
    let mut query = RangeSumQuery::new(vec![1, 3, 5]);

    assert_eq!(query.sum_range(0, 2), Some(9));

    assert!(query.update(1, 2));

    assert_eq!(query.sum_range(0, 2), Some(8));
    assert_eq!(query.sum_range(1, 2), Some(7));
}

#[test]
fn range_sum_query_rejects_invalid_indexes() {
    let mut query = RangeSumQuery::new(vec![1, 2, 3]);

    assert_eq!(query.sum_range(2, 1), None);
    assert_eq!(query.sum_range(0, 3), None);
    assert!(!query.update(3, 10));
}

#[test]
fn segment_tree_returns_range_minimums() {
    let tree = SegmentTree::from_values(&[5, 2, 6, 3, 1, 7]);

    assert_eq!(tree.range_min(0, 5), Some(1));
    assert_eq!(tree.range_min(1, 3), Some(2));
    assert_eq!(tree.range_min(3, 5), Some(1));
}

#[test]
fn segment_tree_updates_values_and_refreshes_minimums() {
    let mut tree = SegmentTree::from_values(&[5, 2, 6, 3, 1, 7]);

    assert!(tree.update(4, 8));

    assert_eq!(tree.range_min(0, 5), Some(2));
    assert_eq!(tree.range_min(3, 5), Some(3));
}

#[test]
fn segment_tree_handles_single_value() {
    let mut tree = SegmentTree::from_values(&[42]);

    assert_eq!(tree.range_min(0, 0), Some(42));
    assert!(tree.update(0, -7));
    assert_eq!(tree.range_min(0, 0), Some(-7));
}

#[test]
fn segment_tree_rejects_invalid_ranges_and_updates() {
    let mut tree = SegmentTree::from_values(&[3, 1, 4]);

    assert_eq!(tree.range_min(2, 1), None);
    assert_eq!(tree.range_min(0, 3), None);
    assert!(!tree.update(3, 9));
}

#[test]
fn difference_array_applies_range_increments() {
    let mut difference = DifferenceArray::new(5);

    assert!(difference.increment_range(1, 3, 2));
    assert!(difference.increment_range(2, 4, 3));

    assert_eq!(difference.values(), vec![0, 2, 5, 5, 3]);
}

#[test]
fn difference_array_rejects_invalid_ranges() {
    let mut difference = DifferenceArray::new(3);

    assert!(!difference.increment_range(2, 1, 5));
    assert!(!difference.increment_range(0, 3, 5));
    assert_eq!(difference.values(), vec![0, 0, 0]);
}

#[test]
fn corporate_flight_bookings_accumulates_reserved_seats() {
    let bookings = vec![(1, 2, 10), (2, 3, 20), (2, 5, 25)];

    assert_eq!(
        corporate_flight_bookings(&bookings, 5),
        vec![10, 55, 45, 25, 25]
    );
}

#[test]
fn corporate_flight_bookings_ignores_invalid_bookings() {
    let bookings = vec![(0, 2, 10), (2, 4, 20), (3, 2, 30), (1, 3, 5)];

    assert_eq!(corporate_flight_bookings(&bookings, 3), vec![5, 5, 5]);
}

#[test]
fn car_pooling_rejects_trips_that_exceed_capacity() {
    let trips = vec![(2, 1, 5), (3, 3, 7)];

    assert!(!car_pooling(&trips, 4));
    assert!(car_pooling(&trips, 5));
}

#[test]
fn car_pooling_treats_destination_as_exclusive() {
    let trips = vec![(2, 1, 5), (3, 5, 7)];

    assert!(car_pooling(&trips, 3));
}

#[test]
fn car_pooling_rejects_invalid_trips_and_capacity() {
    assert!(!car_pooling(&[(1, 3, 3)], 4));
    assert!(!car_pooling(&[(0, 1, 3)], 4));
    assert!(!car_pooling(&[(1, 1, 3)], -1));
}

#[test]
fn count_smaller_numbers_after_self_counts_right_side_values() {
    assert_eq!(
        count_smaller_numbers_after_self(vec![5, 2, 6, 1]),
        vec![2, 1, 1, 0]
    );
}

#[test]
fn count_smaller_numbers_after_self_handles_duplicates_and_negatives() {
    assert_eq!(
        count_smaller_numbers_after_self(vec![-1, -1, -2, 0]),
        vec![1, 1, 0, 0]
    );
}

#[test]
fn count_smaller_numbers_after_self_handles_empty_input() {
    assert_eq!(
        count_smaller_numbers_after_self(Vec::new()),
        Vec::<i32>::new()
    );
}

#[test]
fn reverse_pairs_counts_values_more_than_double_right_side() {
    assert_eq!(reverse_pairs(vec![1, 3, 2, 3, 1]), 2);
    assert_eq!(reverse_pairs(vec![2, 4, 3, 5, 1]), 3);
}

#[test]
fn reverse_pairs_handles_negatives_and_extreme_values() {
    assert_eq!(reverse_pairs(vec![-5, -5]), 1);
    assert_eq!(reverse_pairs(vec![i32::MAX, i32::MAX, i32::MIN]), 2);
}

#[test]
fn reverse_pairs_handles_sorted_and_empty_inputs() {
    assert_eq!(reverse_pairs(vec![1, 2, 3, 4]), 0);
    assert_eq!(reverse_pairs(Vec::new()), 0);
}

#[test]
fn my_calendar_books_non_overlapping_events() {
    let mut calendar = MyCalendar::new();

    assert!(calendar.book(10, 20));
    assert!(calendar.book(20, 30));
    assert!(calendar.book(5, 10));
}

#[test]
fn my_calendar_rejects_overlapping_events() {
    let mut calendar = MyCalendar::new();

    assert!(calendar.book(10, 20));

    assert!(!calendar.book(15, 25));
    assert!(!calendar.book(5, 15));
    assert!(!calendar.book(10, 20));
}

#[test]
fn my_calendar_rejects_empty_or_reversed_ranges() {
    let mut calendar = MyCalendar::new();

    assert!(!calendar.book(10, 10));
    assert!(!calendar.book(20, 10));
    assert!(calendar.book(1, 2));
}

#[test]
fn my_calendar_two_allows_double_bookings() {
    let mut calendar = MyCalendarTwo::new();

    assert!(calendar.book(10, 20));
    assert!(calendar.book(15, 25));
    assert!(calendar.book(20, 30));
}

#[test]
fn my_calendar_two_rejects_triple_bookings() {
    let mut calendar = MyCalendarTwo::new();

    assert!(calendar.book(10, 20));
    assert!(calendar.book(15, 25));

    assert!(!calendar.book(17, 22));
    assert!(calendar.book(25, 35));
}

#[test]
fn my_calendar_two_rejects_empty_or_reversed_ranges() {
    let mut calendar = MyCalendarTwo::new();

    assert!(!calendar.book(5, 5));
    assert!(!calendar.book(9, 4));
    assert!(calendar.book(1, 3));
}
