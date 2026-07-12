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
