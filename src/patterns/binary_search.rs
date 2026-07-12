/// Binary Search
///
/// Pattern: exact binary search.
/// Idea: shrink the search interval based on the middle value.
///
/// Time: O(log n)
/// Space: O(1)
pub fn binary_search(nums: Vec<i32>, target: i32) -> Option<usize> {
    let mut left = 0;
    let mut right = nums.len();

    while left < right {
        let mid = left + (right - left) / 2;

        if nums[mid] == target {
            return Some(mid);
        }

        if nums[mid] < target {
            left = mid + 1;
        } else {
            right = mid;
        }
    }

    None
}

/// Search Insert Position
///
/// Pattern: lower bound.
/// Idea: find the first index whose value is greater than or equal to target.
///
/// Time: O(log n)
/// Space: O(1)
pub fn search_insert(nums: Vec<i32>, target: i32) -> usize {
    let mut left = 0;
    let mut right = nums.len();

    while left < right {
        let mid = left + (right - left) / 2;

        if nums[mid] < target {
            left = mid + 1;
        } else {
            right = mid;
        }
    }

    left
}

/// Search in Rotated Sorted Array
///
/// Pattern: binary search with sorted-half detection.
/// Idea: at least one side of the midpoint is sorted; decide whether target
/// belongs to that side.
///
/// Time: O(log n)
/// Space: O(1)
pub fn search_rotated(nums: Vec<i32>, target: i32) -> Option<usize> {
    if nums.is_empty() {
        return None;
    }

    let mut left = 0;
    let mut right = nums.len() - 1;

    while left <= right {
        let mid = left + (right - left) / 2;

        if nums[mid] == target {
            return Some(mid);
        }

        if nums[left] <= nums[mid] {
            if nums[left] <= target && target < nums[mid] {
                if mid == 0 {
                    break;
                }
                right = mid - 1;
            } else {
                left = mid + 1;
            }
        } else if nums[mid] < target && target <= nums[right] {
            left = mid + 1;
        } else {
            if mid == 0 {
                break;
            }
            right = mid - 1;
        }
    }

    None
}

/// Find Minimum in Rotated Sorted Array
///
/// Pattern: binary search on pivot.
/// Idea: compare mid with the right edge to decide where the minimum lives.
///
/// Time: O(log n)
/// Space: O(1)
pub fn find_min_rotated(nums: Vec<i32>) -> Option<i32> {
    if nums.is_empty() {
        return None;
    }

    let mut left = 0;
    let mut right = nums.len() - 1;

    while left < right {
        let mid = left + (right - left) / 2;

        if nums[mid] > nums[right] {
            left = mid + 1;
        } else {
            right = mid;
        }
    }

    Some(nums[left])
}

/// Koko Eating Bananas
///
/// Pattern: binary search on answer.
/// Idea: eating speed is monotonic; if a speed works, every higher speed works.
///
/// Time: O(n log m), where m is the largest pile.
/// Space: O(1)
pub fn min_eating_speed(piles: Vec<i32>, hours: i32) -> i32 {
    let Some(&max_pile) = piles.iter().max() else {
        return 0;
    };

    let mut left = 1;
    let mut right = max_pile;

    while left < right {
        let speed = left + (right - left) / 2;
        let needed_hours: i32 = piles
            .iter()
            .map(|&pile| divide_rounding_up(pile, speed))
            .sum();

        if needed_hours <= hours {
            right = speed;
        } else {
            left = speed + 1;
        }
    }

    left
}

/// Capacity To Ship Packages Within D Days
///
/// Pattern: binary search on answer.
/// Idea: capacity is monotonic; if one capacity ships in time, every larger
/// capacity also ships in time.
///
/// Time: O(n log s), where s is the sum of weights.
/// Space: O(1)
pub fn ship_within_days(weights: Vec<i32>, days: i32) -> i32 {
    let Some(&max_weight) = weights.iter().max() else {
        return 0;
    };

    let mut left = max_weight;
    let mut right: i32 = weights.iter().sum();

    while left < right {
        let capacity = left + (right - left) / 2;
        let needed_days = days_needed(&weights, capacity);

        if needed_days <= days {
            right = capacity;
        } else {
            left = capacity + 1;
        }
    }

    left
}

fn days_needed(weights: &[i32], capacity: i32) -> i32 {
    let mut days = 1;
    let mut current_load = 0;

    for &weight in weights {
        if current_load + weight > capacity {
            days += 1;
            current_load = 0;
        }

        current_load += weight;
    }

    days
}

fn divide_rounding_up(value: i32, divisor: i32) -> i32 {
    (value + divisor - 1) / divisor
}
