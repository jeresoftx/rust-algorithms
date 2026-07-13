use rust_algorithms::patterns::intervals::{
    can_attend_meetings, erase_overlap_intervals, insert_interval, merge_intervals,
    min_arrows_to_burst_balloons, min_meeting_rooms,
};

#[test]
fn merge_intervals_combines_overlapping_ranges() {
    let intervals = vec![(1, 3), (2, 6), (8, 10), (15, 18)];

    assert_eq!(merge_intervals(intervals), vec![(1, 6), (8, 10), (15, 18)]);
}

#[test]
fn merge_intervals_handles_unsorted_input() {
    let intervals = vec![(8, 10), (1, 4), (2, 3)];

    assert_eq!(merge_intervals(intervals), vec![(1, 4), (8, 10)]);
}

#[test]
fn insert_interval_merges_new_interval_into_existing_ranges() {
    let intervals = vec![(1, 3), (6, 9)];

    assert_eq!(insert_interval(intervals, (2, 5)), vec![(1, 5), (6, 9)]);
}

#[test]
fn insert_interval_merges_multiple_ranges() {
    let intervals = vec![(1, 2), (3, 5), (6, 7), (8, 10), (12, 16)];

    assert_eq!(
        insert_interval(intervals, (4, 8)),
        vec![(1, 2), (3, 10), (12, 16)]
    );
}

#[test]
fn can_attend_meetings_returns_true_when_no_meetings_overlap() {
    let meetings = vec![(7, 10), (2, 4)];

    assert!(can_attend_meetings(meetings));
}

#[test]
fn can_attend_meetings_returns_false_when_meetings_overlap() {
    let meetings = vec![(0, 30), (5, 10), (15, 20)];

    assert!(!can_attend_meetings(meetings));
}

#[test]
fn min_meeting_rooms_returns_peak_number_of_overlaps() {
    let meetings = vec![(0, 30), (5, 10), (15, 20)];

    assert_eq!(min_meeting_rooms(meetings), 2);
}

#[test]
fn min_meeting_rooms_reuses_room_when_meeting_ends_at_next_start() {
    let meetings = vec![(7, 10), (2, 4), (4, 7)];

    assert_eq!(min_meeting_rooms(meetings), 1);
}

#[test]
fn erase_overlap_intervals_returns_minimum_removals() {
    let intervals = vec![(1, 2), (2, 3), (3, 4), (1, 3)];

    assert_eq!(erase_overlap_intervals(intervals), 1);
}

#[test]
fn erase_overlap_intervals_removes_all_but_one_when_all_overlap() {
    let intervals = vec![(1, 2), (1, 2), (1, 2)];

    assert_eq!(erase_overlap_intervals(intervals), 2);
}

#[test]
fn min_arrows_to_burst_balloons_groups_overlapping_ranges() {
    let balloons = vec![(10, 16), (2, 8), (1, 6), (7, 12)];

    assert_eq!(min_arrows_to_burst_balloons(balloons), 2);
}

#[test]
fn min_arrows_to_burst_balloons_handles_empty_input() {
    assert_eq!(min_arrows_to_burst_balloons(Vec::new()), 0);
}
