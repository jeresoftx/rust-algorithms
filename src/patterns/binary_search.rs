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
