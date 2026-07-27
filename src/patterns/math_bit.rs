//! Number theory, arithmetic and bit-manipulation routines.
//!
//! # Example
//!
//! ```
//! use rust_algorithms::patterns::math_bit::{gcd, is_power_of_two};
//!
//! assert_eq!(gcd(54, 24), 6);
//! assert!(is_power_of_two(64));
//! ```

/// Finds the value that appears once when every other value appears twice.
///
/// Time: O(n)
/// Space: O(1)
pub fn single_number(nums: Vec<i32>) -> i32 {
    nums.into_iter().fold(0, |unique, value| unique ^ value)
}

/// Finds the missing value from a permutation of `0..=n`.
///
/// Time: O(n)
/// Space: O(1)
pub fn missing_number(nums: Vec<i32>) -> i32 {
    let mut missing = nums.len() as i32;

    for (index, value) in nums.into_iter().enumerate() {
        missing ^= index as i32;
        missing ^= value;
    }

    missing
}

/// Counts set bits using Brian Kernighan's trick.
///
/// Time: O(number of set bits)
/// Space: O(1)
pub fn count_ones(mut value: u32) -> u32 {
    let mut count = 0;

    while value != 0 {
        value &= value - 1;
        count += 1;
    }

    count
}

/// Returns the bit counts for all values from `0` to `limit`.
///
/// Time: O(n)
/// Space: O(n)
pub fn count_bits(limit: usize) -> Vec<u32> {
    let mut counts = vec![0; limit + 1];

    for value in 1..=limit {
        counts[value] = counts[value >> 1] + (value & 1) as u32;
    }

    counts
}

/// Reverses all 32 bits in an unsigned integer.
///
/// Time: O(32)
/// Space: O(1)
pub fn reverse_bits(mut value: u32) -> u32 {
    let mut reversed = 0;

    for _ in 0..32 {
        reversed <<= 1;
        reversed |= value & 1;
        value >>= 1;
    }

    reversed
}

/// Counts bit positions where two values differ.
///
/// Time: O(number of set bits in xor)
/// Space: O(1)
pub fn hamming_distance(left: u32, right: u32) -> u32 {
    count_ones(left ^ right)
}

/// Checks whether a positive integer has exactly one bit set.
///
/// Time: O(1)
/// Space: O(1)
pub fn is_power_of_two(value: i32) -> bool {
    value > 0 && (value & (value - 1)) == 0
}

/// Checks whether a positive integer is a perfect square.
///
/// Time: O(log n)
/// Space: O(1)
pub fn is_perfect_square(value: i32) -> bool {
    if value <= 0 {
        return false;
    }

    let target = value as i64;
    let mut left = 1_i64;
    let mut right = target;

    while left <= right {
        let middle = left + (right - left) / 2;
        let square = middle * middle;

        if square == target {
            return true;
        }

        if square < target {
            left = middle + 1;
        } else {
            right = middle - 1;
        }
    }

    false
}

/// Computes `base.powi(exponent)` by exponentiation by squaring.
///
/// Time: O(log |exponent|)
/// Space: O(1)
pub fn fast_pow(base: f64, exponent: i32) -> f64 {
    if exponent < 0 {
        return 1.0 / pow_positive(base, -(exponent as i64));
    }

    pow_positive(base, exponent as i64)
}

/// Adds two binary strings and returns the binary representation of the sum.
///
/// Time: O(n + m)
/// Space: O(n + m)
pub fn add_binary(left: &str, right: &str) -> String {
    let mut left_digits = left.as_bytes().iter().rev();
    let mut right_digits = right.as_bytes().iter().rev();
    let mut carry = 0_u8;
    let mut result = Vec::new();

    loop {
        let left_bit = left_digits.next().map(|digit| digit - b'0');
        let right_bit = right_digits.next().map(|digit| digit - b'0');

        if left_bit.is_none() && right_bit.is_none() && carry == 0 {
            break;
        }

        let sum = left_bit.unwrap_or(0) + right_bit.unwrap_or(0) + carry;
        result.push(char::from(b'0' + sum % 2));
        carry = sum / 2;
    }

    result.into_iter().rev().collect()
}

/// Adds one to a decimal number represented as digits.
///
/// Time: O(n)
/// Space: O(1) amortized, excluding possible growth by one digit.
pub fn plus_one(mut digits: Vec<i32>) -> Vec<i32> {
    for index in (0..digits.len()).rev() {
        if digits[index] < 9 {
            digits[index] += 1;
            return digits;
        }

        digits[index] = 0;
    }

    digits.insert(0, 1);
    digits
}

/// Computes the greatest common divisor with Euclid's algorithm.
///
/// Time: O(log min(a, b))
/// Space: O(1)
pub fn gcd(left: i64, right: i64) -> i64 {
    let mut a = left.abs();
    let mut b = right.abs();

    while b != 0 {
        let remainder = a % b;
        a = b;
        b = remainder;
    }

    a
}

/// Computes the least common multiple.
///
/// Time: O(log min(a, b))
/// Space: O(1)
pub fn lcm(left: i64, right: i64) -> i64 {
    if left == 0 || right == 0 {
        return 0;
    }

    (left / gcd(left, right) * right).abs()
}

/// Counts trailing zeroes in `value!`.
///
/// Time: O(log_5 n)
/// Space: O(1)
pub fn trailing_zeroes(mut value: i32) -> i32 {
    let mut zeroes = 0;

    while value > 0 {
        value /= 5;
        zeroes += value;
    }

    zeroes
}

/// Lists all prime numbers up to `limit` with the sieve of Eratosthenes.
///
/// Time: O(n log log n)
/// Space: O(n)
pub fn sieve(limit: usize) -> Vec<usize> {
    if limit < 2 {
        return Vec::new();
    }

    let mut is_prime = vec![true; limit + 1];
    is_prime[0] = false;
    is_prime[1] = false;

    let mut value = 2;
    while value * value <= limit {
        if is_prime[value] {
            let mut multiple = value * value;

            while multiple <= limit {
                is_prime[multiple] = false;
                multiple += value;
            }
        }

        value += 1;
    }

    is_prime
        .iter()
        .enumerate()
        .filter_map(|(value, prime)| prime.then_some(value))
        .collect()
}

/// Returns the maximum sum over any non-empty contiguous subarray.
///
/// Time: O(n)
/// Space: O(1)
pub fn maximum_subarray(nums: Vec<i32>) -> Option<i32> {
    let mut values = nums.into_iter();
    let first = values.next()?;
    let mut current = first;
    let mut best = first;

    for value in values {
        current = value.max(current + value);
        best = best.max(current);
    }

    Some(best)
}

/// Finds the majority candidate with Boyer-Moore voting.
///
/// The function assumes the input follows the majority-element problem
/// contract: when non-empty, a value appears more than half the time.
///
/// Time: O(n)
/// Space: O(1)
pub fn majority_element(nums: Vec<i32>) -> Option<i32> {
    let mut candidate = None;
    let mut count = 0;

    for value in nums {
        if count == 0 {
            candidate = Some(value);
            count = 1;
        } else if candidate == Some(value) {
            count += 1;
        } else {
            count -= 1;
        }
    }

    candidate
}

fn pow_positive(mut base: f64, mut exponent: i64) -> f64 {
    let mut result = 1.0;

    while exponent > 0 {
        if exponent % 2 == 1 {
            result *= base;
        }

        base *= base;
        exponent /= 2;
    }

    result
}
