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
            ')' | ']' | '}' if expected.pop() != Some(character) => return false,
            ')' | ']' | '}' => {}
            _ => {}
        }
    }

    expected.is_empty()
}

/// Backspace String Compare
///
/// Pattern: stack simulation.
/// Idea: build each effective text by removing the latest character for every
/// backspace, then compare the resulting character stacks.
///
/// Time: O(n + m)
/// Space: O(n + m)
pub fn backspace_compare(left: &str, right: &str) -> bool {
    effective_text(left) == effective_text(right)
}

fn effective_text(text: &str) -> Vec<char> {
    let mut characters = Vec::new();

    for character in text.chars() {
        if character == '#' {
            characters.pop();
        } else {
            characters.push(character);
        }
    }

    characters
}

/// Baseball Game
///
/// Pattern: stack simulation.
/// Idea: the stack keeps every valid round, so cancel, duplicate and sum
/// operations can inspect only the latest scores.
///
/// Time: O(n)
/// Space: O(n)
pub fn baseball_game(operations: &[&str]) -> Option<i32> {
    let mut scores = Vec::new();

    for &operation in operations {
        match operation {
            "C" => {
                scores.pop()?;
            }
            "D" => {
                let score = scores.last()? * 2;
                scores.push(score);
            }
            "+" => {
                let (&previous, &last) =
                    (scores.get(scores.len().checked_sub(2)?)?, scores.last()?);
                scores.push(previous + last);
            }
            value => scores.push(value.parse().ok()?),
        }
    }

    Some(scores.iter().sum())
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

/// Asteroid Collision
///
/// Pattern: stack of unresolved survivors.
/// Idea: collisions are possible only when a positive survivor is immediately
/// before a negative incoming asteroid, so the stack resolves that frontier.
///
/// Time: O(n)
/// Space: O(n)
pub fn asteroid_collision(asteroids: Vec<i32>) -> Vec<i32> {
    let mut survivors = Vec::new();

    for asteroid in asteroids {
        let mut survives = true;

        while survives && asteroid < 0 {
            let Some(&previous) = survivors.last() else {
                break;
            };

            if previous < 0 {
                break;
            }

            let incoming_magnitude = -i64::from(asteroid);

            if i64::from(previous) < incoming_magnitude {
                survivors.pop();
            } else {
                if i64::from(previous) == incoming_magnitude {
                    survivors.pop();
                }
                survives = false;
            }
        }

        if survives {
            survivors.push(asteroid);
        }
    }

    survivors
}

/// Online Stock Span
///
/// Pattern: monotonic decreasing stack with accumulated spans.
/// Idea: each stack entry absorbs consecutive earlier prices no greater than
/// its price, so every price is pushed and popped at most once.
///
/// Time: amortized O(1) per `next` call.
/// Space: O(n)
#[derive(Default)]
pub struct StockSpanner {
    prices: Vec<(i32, usize)>,
}

impl StockSpanner {
    /// Creates an empty price stream.
    pub fn new() -> Self {
        Self::default()
    }

    /// Records `price` and returns its consecutive span.
    pub fn next(&mut self, price: i32) -> usize {
        let mut span = 1;

        while let Some(&(previous_price, previous_span)) = self.prices.last() {
            if previous_price > price {
                break;
            }

            self.prices.pop();
            span += previous_span;
        }

        self.prices.push((price, span));
        span
    }
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
