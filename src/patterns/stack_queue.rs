/// Valid Parentheses
///
/// Pattern: stack.
/// Idea: push expected closing brackets and require each closing bracket to
/// match the top of the stack.
///
/// Time: O(n)
/// Space: O(n)
pub fn valid_parentheses(text: &str) -> bool {
    let mut expected = Vec::new();

    for character in text.chars() {
        match character {
            '(' => expected.push(')'),
            '[' => expected.push(']'),
            '{' => expected.push('}'),
            ')' | ']' | '}' => {
                if expected.pop() != Some(character) {
                    return false;
                }
            }
            _ => {}
        }
    }

    expected.is_empty()
}

/// Daily Temperatures
///
/// Pattern: monotonic decreasing stack.
/// Idea: keep indexes whose warmer day has not been found yet.
///
/// Time: O(n)
/// Space: O(n)
pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
    let mut result = vec![0; temperatures.len()];
    let mut stack: Vec<usize> = Vec::new();

    for (index, &temperature) in temperatures.iter().enumerate() {
        while let Some(&previous_index) = stack.last() {
            if temperatures[previous_index] >= temperature {
                break;
            }

            stack.pop();
            result[previous_index] = (index - previous_index) as i32;
        }

        stack.push(index);
    }

    result
}

/// Largest Rectangle in Histogram
///
/// Pattern: monotonic increasing stack.
/// Idea: when a lower bar appears, it closes rectangles for taller bars.
///
/// Time: O(n)
/// Space: O(n)
pub fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
    let mut extended = heights;
    extended.push(0);

    let mut stack: Vec<usize> = Vec::new();
    let mut best = 0;

    for (index, &height) in extended.iter().enumerate() {
        while let Some(&top_index) = stack.last() {
            if extended[top_index] <= height {
                break;
            }

            stack.pop();
            let rectangle_height = extended[top_index];
            let width = if let Some(&left_index) = stack.last() {
                index - left_index - 1
            } else {
                index
            };

            best = best.max(rectangle_height * width as i32);
        }

        stack.push(index);
    }

    best
}
