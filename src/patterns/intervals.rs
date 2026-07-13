use std::cmp::Reverse;
use std::collections::BinaryHeap;

/// Merge Intervals
///
/// Pattern: sort by start, then merge into the last range.
///
/// Time: O(n log n)
/// Space: O(n)
pub fn merge_intervals(mut intervals: Vec<(i32, i32)>) -> Vec<(i32, i32)> {
    if intervals.is_empty() {
        return Vec::new();
    }

    intervals.sort_unstable_by_key(|&(start, end)| (start, end));
    let mut merged = Vec::new();

    for (start, end) in intervals {
        match merged.last_mut() {
            Some((_, previous_end)) if start <= *previous_end => {
                *previous_end = (*previous_end).max(end);
            }
            _ => merged.push((start, end)),
        }
    }

    merged
}

/// Insert Interval
///
/// Pattern: append and reuse merge.
///
/// Time: O(n log n)
/// Space: O(n)
pub fn insert_interval(
    mut intervals: Vec<(i32, i32)>,
    new_interval: (i32, i32),
) -> Vec<(i32, i32)> {
    intervals.push(new_interval);
    merge_intervals(intervals)
}

/// Meeting Rooms
///
/// Pattern: sorted intervals.
/// Idea: a meeting can start when the previous one has already ended.
///
/// Time: O(n log n)
/// Space: O(1) extra space
pub fn can_attend_meetings(mut meetings: Vec<(i32, i32)>) -> bool {
    meetings.sort_unstable_by_key(|&(start, end)| (start, end));

    meetings.windows(2).all(|pair| pair[0].1 <= pair[1].0)
}

/// Meeting Rooms II
///
/// Pattern: min heap of end times.
/// Idea: reuse the room that ends earliest when possible.
///
/// Time: O(n log n)
/// Space: O(n)
pub fn min_meeting_rooms(mut meetings: Vec<(i32, i32)>) -> i32 {
    if meetings.is_empty() {
        return 0;
    }

    meetings.sort_unstable_by_key(|&(start, end)| (start, end));
    let mut room_end_times = BinaryHeap::new();
    let mut best = 0;

    for (start, end) in meetings {
        while room_end_times
            .peek()
            .is_some_and(|&Reverse(earliest_end)| earliest_end <= start)
        {
            room_end_times.pop();
        }

        room_end_times.push(Reverse(end));
        best = best.max(room_end_times.len());
    }

    best as i32
}

/// Non-overlapping Intervals
///
/// Pattern: greedy by earliest end.
/// Idea: keep the interval that leaves the most room for future intervals.
///
/// Time: O(n log n)
/// Space: O(1) extra space
pub fn erase_overlap_intervals(mut intervals: Vec<(i32, i32)>) -> i32 {
    if intervals.is_empty() {
        return 0;
    }

    intervals.sort_unstable_by_key(|&(start, end)| (end, start));
    let mut removals = 0;
    let mut last_end = intervals[0].1;

    for (start, end) in intervals.into_iter().skip(1) {
        if start < last_end {
            removals += 1;
        } else {
            last_end = end;
        }
    }

    removals
}

/// Minimum Number of Arrows to Burst Balloons
///
/// Pattern: greedy by earliest end.
/// Idea: one arrow can burst every balloon that overlaps the current end point.
///
/// Time: O(n log n)
/// Space: O(1) extra space
pub fn min_arrows_to_burst_balloons(mut balloons: Vec<(i32, i32)>) -> i32 {
    if balloons.is_empty() {
        return 0;
    }

    balloons.sort_unstable_by_key(|&(start, end)| (end, start));
    let mut arrows = 1;
    let mut current_end = balloons[0].1;

    for (start, end) in balloons.into_iter().skip(1) {
        if start > current_end {
            arrows += 1;
            current_end = end;
        }
    }

    arrows
}
