use rust_algorithms::patterns::math_bit::{
    count_bits, count_ones, fast_pow, gcd, is_power_of_two, lcm, maximum_subarray, reverse_bits,
    sieve, single_number,
};

fn assert_close(actual: f64, expected: f64) {
    let delta = (actual - expected).abs();
    assert!(
        delta < 1e-10,
        "expected {actual} to be within 1e-10 of {expected}"
    );
}

#[test]
fn single_number_returns_value_without_pair() {
    assert_eq!(single_number(vec![4, 1, 2, 1, 2]), 4);
}

#[test]
fn single_number_handles_negative_values() {
    assert_eq!(single_number(vec![-7, 3, 3, 2, 2]), -7);
}

#[test]
fn count_ones_counts_enabled_bits() {
    assert_eq!(count_ones(0b1011), 3);
}

#[test]
fn count_ones_returns_zero_for_zero() {
    assert_eq!(count_ones(0), 0);
}

#[test]
fn count_bits_returns_count_for_each_prefix_value() {
    assert_eq!(count_bits(5), vec![0, 1, 1, 2, 1, 2]);
}

#[test]
fn count_bits_handles_zero_limit() {
    assert_eq!(count_bits(0), vec![0]);
}

#[test]
fn reverse_bits_reverses_all_thirty_two_positions() {
    assert_eq!(
        reverse_bits(0b00000010100101000001111010011100),
        0b00111001011110000010100101000000
    );
}

#[test]
fn reverse_bits_preserves_zero() {
    assert_eq!(reverse_bits(0), 0);
}

#[test]
fn is_power_of_two_accepts_positive_single_bit_values() {
    assert!(is_power_of_two(1024));
}

#[test]
fn is_power_of_two_rejects_zero_negative_and_multi_bit_values() {
    assert!(!is_power_of_two(0));
    assert!(!is_power_of_two(-16));
    assert!(!is_power_of_two(18));
}

#[test]
fn fast_pow_handles_positive_exponent() {
    assert_close(fast_pow(2.0, 10), 1024.0);
}

#[test]
fn fast_pow_handles_negative_exponent() {
    assert_close(fast_pow(2.0, -2), 0.25);
}

#[test]
fn fast_pow_handles_zero_exponent() {
    assert_close(fast_pow(9.5, 0), 1.0);
}

#[test]
fn gcd_returns_greatest_common_divisor() {
    assert_eq!(gcd(54, 24), 6);
}

#[test]
fn gcd_handles_zero_and_negative_values() {
    assert_eq!(gcd(-42, 0), 42);
    assert_eq!(gcd(-42, 56), 14);
}

#[test]
fn lcm_returns_least_common_multiple() {
    assert_eq!(lcm(21, 6), 42);
}

#[test]
fn lcm_returns_zero_when_any_value_is_zero() {
    assert_eq!(lcm(0, 9), 0);
}

#[test]
fn sieve_returns_primes_up_to_limit() {
    assert_eq!(sieve(20), vec![2, 3, 5, 7, 11, 13, 17, 19]);
}

#[test]
fn sieve_returns_empty_when_limit_is_less_than_two() {
    assert!(sieve(1).is_empty());
}

#[test]
fn maximum_subarray_returns_best_contiguous_sum() {
    assert_eq!(
        maximum_subarray(vec![-2, 1, -3, 4, -1, 2, 1, -5, 4]),
        Some(6)
    );
}

#[test]
fn maximum_subarray_handles_all_negative_values() {
    assert_eq!(maximum_subarray(vec![-8, -3, -6, -2, -5, -4]), Some(-2));
}

#[test]
fn maximum_subarray_returns_none_for_empty_input() {
    assert_eq!(maximum_subarray(Vec::new()), None);
}
