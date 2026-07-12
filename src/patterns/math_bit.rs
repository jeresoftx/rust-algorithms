pub fn single_number(nums: Vec<i32>) -> i32 {
    nums.into_iter().fold(0, |unique, value| unique ^ value)
}

pub fn count_ones(mut value: u32) -> u32 {
    let mut count = 0;

    while value != 0 {
        value &= value - 1;
        count += 1;
    }

    count
}

pub fn count_bits(limit: usize) -> Vec<u32> {
    let mut counts = vec![0; limit + 1];

    for value in 1..=limit {
        counts[value] = counts[value >> 1] + (value & 1) as u32;
    }

    counts
}

pub fn reverse_bits(mut value: u32) -> u32 {
    let mut reversed = 0;

    for _ in 0..32 {
        reversed <<= 1;
        reversed |= value & 1;
        value >>= 1;
    }

    reversed
}

pub fn is_power_of_two(value: i32) -> bool {
    value > 0 && (value & (value - 1)) == 0
}

pub fn fast_pow(base: f64, exponent: i32) -> f64 {
    if exponent < 0 {
        return 1.0 / pow_positive(base, -(exponent as i64));
    }

    pow_positive(base, exponent as i64)
}

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

pub fn lcm(left: i64, right: i64) -> i64 {
    if left == 0 || right == 0 {
        return 0;
    }

    (left / gcd(left, right) * right).abs()
}

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
