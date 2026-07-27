use proptest::prelude::*;
use rust_algorithms::patterns::binary_search::search_insert;
use rust_algorithms::patterns::math_bit::{gcd, lcm};
use rust_algorithms::patterns::range_queries::FenwickTree;
use rust_algorithms::patterns::two_pointers::sorted_squares;

fn sorted_values() -> impl Strategy<Value = Vec<i32>> {
    proptest::collection::vec(-1_000..=1_000, 0..100).prop_map(|mut values| {
        values.sort_unstable();
        values
    })
}

proptest! {
    #[test]
    fn search_insert_matches_partition_point(values in sorted_values(), target in -1_200..=1_200) {
        let expected = values.partition_point(|&value| value < target);

        prop_assert_eq!(search_insert(values, target), expected);
    }

    #[test]
    fn sorted_squares_preserves_square_multiset(values in sorted_values()) {
        let mut expected: Vec<i32> = values.iter().map(|value| value * value).collect();
        expected.sort_unstable();

        prop_assert_eq!(sorted_squares(values), expected);
    }

    #[test]
    fn gcd_is_non_negative_and_divides_inputs(
        left in -1_000_000_i64..=1_000_000,
        right in -1_000_000_i64..=1_000_000,
    ) {
        let divisor = gcd(left, right);

        prop_assert!(divisor >= 0);
        if left == 0 && right == 0 {
            prop_assert_eq!(divisor, 0);
        } else {
            prop_assert_eq!(left % divisor, 0);
            prop_assert_eq!(right % divisor, 0);
        }
    }

    #[test]
    fn lcm_matches_gcd_relation(
        left in -10_000_i64..=10_000,
        right in -10_000_i64..=10_000,
    ) {
        let multiple = lcm(left, right);

        if left == 0 || right == 0 {
            prop_assert_eq!(multiple, 0);
        } else {
            let divisor = gcd(left, right);
            let expected = (left / divisor * right).abs();

            prop_assert_eq!(multiple, expected);
            prop_assert_eq!(multiple % left.abs(), 0);
            prop_assert_eq!(multiple % right.abs(), 0);
        }
    }

    #[test]
    fn fenwick_tree_matches_naive_prefix_sums(
        values in proptest::collection::vec(-1_000_i32..=1_000, 1..80),
        updates in proptest::collection::vec((0_usize..80, -100_i32..=100), 0..80),
    ) {
        let mut tree = FenwickTree::from_values(&values);
        let mut naive = values;

        for (index, delta) in updates {
            let index = index % naive.len();

            prop_assert!(tree.add(index, delta));
            naive[index] += delta;

            let mut running = 0;
            for (prefix_end, value) in naive.iter().enumerate() {
                running += value;
                prop_assert_eq!(tree.prefix_sum(prefix_end), Some(running));
            }
        }
    }
}
