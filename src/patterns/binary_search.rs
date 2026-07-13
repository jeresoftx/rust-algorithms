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

/// Find First and Last Position of Element in Sorted Array
///
/// Pattern: lower bound twice.
/// Idea: first find the first target, then the first value greater than target.
///
/// Time: O(log n)
/// Space: O(1)
pub fn search_range(nums: Vec<i32>, target: i32) -> Option<(usize, usize)> {
    let first = lower_bound(&nums, target);

    if first == nums.len() || nums[first] != target {
        return None;
    }

    let after_last = upper_bound(&nums, target);
    Some((first, after_last - 1))
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

/// Search a 2D Matrix
///
/// Pattern: binary search over a flattened sorted matrix.
/// Idea: map a virtual index to row and column without allocating.
///
/// Time: O(log(m*n))
/// Space: O(1)
pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
    if matrix.is_empty() || matrix[0].is_empty() {
        return false;
    }

    let rows = matrix.len();
    let cols = matrix[0].len();
    let mut left = 0;
    let mut right = rows * cols;

    while left < right {
        let middle = left + (right - left) / 2;
        let value = matrix[middle / cols][middle % cols];

        if value == target {
            return true;
        }

        if value < target {
            left = middle + 1;
        } else {
            right = middle;
        }
    }

    false
}

/// Find Peak Element
///
/// Pattern: binary search on slope.
/// Idea: if mid is lower than mid + 1, a peak exists to the right.
///
/// Time: O(log n)
/// Space: O(1)
pub fn find_peak_element(nums: Vec<i32>) -> Option<usize> {
    if nums.is_empty() {
        return None;
    }

    let mut left = 0;
    let mut right = nums.len() - 1;

    while left < right {
        let middle = left + (right - left) / 2;

        if nums[middle] < nums[middle + 1] {
            left = middle + 1;
        } else {
            right = middle;
        }
    }

    Some(left)
}

/// Arranging Coins
///
/// Pattern: binary search on answer.
/// Idea: row count is monotonic because triangular numbers only increase.
///
/// Time: O(log n)
/// Space: O(1)
pub fn arrange_coins(coins: i32) -> i32 {
    if coins <= 0 {
        return 0;
    }

    let total = coins as i64;
    let mut left = 1_i64;
    let mut right = total;

    while left <= right {
        let rows = left + (right - left) / 2;
        let needed = rows * (rows + 1) / 2;

        if needed == total {
            return rows as i32;
        }

        if needed < total {
            left = rows + 1;
        } else {
            right = rows - 1;
        }
    }

    right as i32
}

fn lower_bound(nums: &[i32], target: i32) -> usize {
    let mut left = 0;
    let mut right = nums.len();

    while left < right {
        let middle = left + (right - left) / 2;

        if nums[middle] < target {
            left = middle + 1;
        } else {
            right = middle;
        }
    }

    left
}

fn upper_bound(nums: &[i32], target: i32) -> usize {
    let mut left = 0;
    let mut right = nums.len();

    while left < right {
        let middle = left + (right - left) / 2;

        if nums[middle] <= target {
            left = middle + 1;
        } else {
            right = middle;
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
