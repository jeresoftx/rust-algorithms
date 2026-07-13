/// Valid Palindrome
///
/// Pattern: two pointers.
/// Idea: compare alphanumeric characters from both ends, normalizing case.
///
/// Time: O(n)
/// Space: O(1)
pub fn valid_palindrome(text: &str) -> bool {
    let characters: Vec<char> = text.chars().collect();

    if characters.is_empty() {
        return true;
    }

    let mut left = 0;
    let mut right = characters.len() - 1;

    while left < right {
        while left < right && !characters[left].is_alphanumeric() {
            left += 1;
        }

        while left < right && !characters[right].is_alphanumeric() {
            right -= 1;
        }

        if !characters[left].eq_ignore_ascii_case(&characters[right]) {
            return false;
        }

        left += 1;
        right -= 1;
    }

    true
}

/// 3Sum
///
/// Pattern: sort + two pointers.
/// Idea: fix one value, then scan the remaining sorted slice from both ends.
///
/// Time: O(n^2)
/// Space: O(1) extra space, excluding the returned triplets.
pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
    nums.sort_unstable();
    let mut triplets = Vec::new();

    for index in 0..nums.len() {
        if index > 0 && nums[index] == nums[index - 1] {
            continue;
        }

        let mut left = index + 1;
        let mut right = nums.len().saturating_sub(1);

        while left < right {
            let sum = nums[index] + nums[left] + nums[right];

            if sum == 0 {
                triplets.push(vec![nums[index], nums[left], nums[right]]);
                left += 1;
                right -= 1;

                while left < right && nums[left] == nums[left - 1] {
                    left += 1;
                }

                while left < right && nums[right] == nums[right + 1] {
                    right -= 1;
                }
            } else if sum < 0 {
                left += 1;
            } else {
                right -= 1;
            }
        }
    }

    triplets
}

/// Container With Most Water
///
/// Pattern: two pointers.
/// Idea: the smaller wall limits the area, so move that side inward.
///
/// Time: O(n)
/// Space: O(1)
pub fn max_area(heights: Vec<i32>) -> i32 {
    if heights.len() < 2 {
        return 0;
    }

    let mut left = 0;
    let mut right = heights.len() - 1;
    let mut best = 0;

    while left < right {
        let width = (right - left) as i32;
        let height = heights[left].min(heights[right]);
        best = best.max(width * height);

        if heights[left] < heights[right] {
            left += 1;
        } else {
            right -= 1;
        }
    }

    best
}

/// Two Sum II - Input Array Is Sorted
///
/// Pattern: two pointers over a sorted array.
/// Idea: move the left side to increase the sum and the right side to reduce it.
///
/// Time: O(n)
/// Space: O(1)
pub fn two_sum_sorted(nums: Vec<i32>, target: i32) -> Option<(usize, usize)> {
    if nums.len() < 2 {
        return None;
    }

    let mut left = 0;
    let mut right = nums.len() - 1;

    while left < right {
        let sum = nums[left] + nums[right];

        if sum == target {
            return Some((left, right));
        }

        if sum < target {
            left += 1;
        } else {
            right -= 1;
        }
    }

    None
}

/// Remove Duplicates from Sorted Array
///
/// Pattern: slow writer pointer.
/// Idea: keep only the first occurrence of each sorted value.
///
/// Time: O(n)
/// Space: O(n), because this study-friendly version returns a compact vector.
pub fn remove_duplicates_sorted(nums: Vec<i32>) -> Vec<i32> {
    let mut unique = Vec::new();

    for value in nums {
        if unique.last() != Some(&value) {
            unique.push(value);
        }
    }

    unique
}

/// Move Zeroes
///
/// Pattern: stable writer pointer.
/// Idea: write non-zero values first, then fill the suffix with zeroes.
///
/// Time: O(n)
/// Space: O(1) extra space.
pub fn move_zeroes(mut nums: Vec<i32>) -> Vec<i32> {
    let mut write = 0;

    for read in 0..nums.len() {
        if nums[read] != 0 {
            nums[write] = nums[read];
            write += 1;
        }
    }

    nums[write..].fill(0);
    nums
}

/// Squares of a Sorted Array
///
/// Pattern: two pointers from both extremes.
/// Idea: the largest square comes from the largest absolute value.
///
/// Time: O(n)
/// Space: O(n)
pub fn sorted_squares(nums: Vec<i32>) -> Vec<i32> {
    if nums.is_empty() {
        return Vec::new();
    }

    let mut result = vec![0; nums.len()];
    let mut left = 0;
    let mut right = nums.len() - 1;

    for write in (0..nums.len()).rev() {
        if nums[left].abs() > nums[right].abs() {
            result[write] = nums[left] * nums[left];
            left += 1;
        } else {
            result[write] = nums[right] * nums[right];

            if right == 0 {
                break;
            }

            right -= 1;
        }
    }

    result
}
