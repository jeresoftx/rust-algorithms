use rust_algorithms::patterns::range_queries::{FenwickTree, RangeSumQuery, SegmentTree};

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
