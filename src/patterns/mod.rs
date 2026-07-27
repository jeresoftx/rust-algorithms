//! Pattern-oriented catalog of algorithm implementations.
//!
//! # Example
//!
//! ```
//! use rust_algorithms::patterns::two_pointers::sorted_squares;
//!
//! assert_eq!(sorted_squares(vec![-4, -1, 0, 3]), vec![0, 1, 9, 16]);
//! ```

/// Binary-search patterns over arrays, answers and monotonic predicates.
pub mod binary_search;
/// Dynamic-programming patterns for one-dimensional, grid and string states.
pub mod dynamic_programming;
/// Computational geometry helpers and sweep-line examples.
pub mod geometry;
/// Unweighted graph traversals, topological ordering and Union-Find examples.
pub mod graphs;
/// Greedy algorithms where a local invariant drives the choice.
pub mod greedy;
/// Hash-table and counting patterns for lookup-heavy problems.
pub mod hashing;
/// Heap-based selection, streaming and merge patterns.
pub mod heaps;
/// Interval sorting and overlap-management patterns.
pub mod intervals;
/// Singly linked-list helpers and pointer-rewiring patterns.
pub mod linked_lists;
/// Number theory, bit manipulation and arithmetic helpers.
pub mod math_bit;
/// Matrix traversal, in-place mutation and randomized-set examples.
pub mod matrices;
/// Prefix sums, Fenwick trees, segment trees and range-query structures.
pub mod range_queries;
/// Backtracking, recursion and exhaustive-search patterns.
pub mod recursion_backtracking;
/// Sliding-window patterns over contiguous ranges.
pub mod sliding_window;
/// Stack, queue and monotonic-structure patterns.
pub mod stack_queue;
/// String matching, hashing and palindrome algorithms.
pub mod string_algorithms;
/// Binary-tree construction, traversal and serialization patterns.
pub mod trees;
/// Trie and prefix-tree based search structures.
pub mod tries;
/// Two-pointer patterns over sorted arrays and converging indexes.
pub mod two_pointers;
/// Weighted-graph shortest path, MST and connectivity algorithms.
pub mod weighted_graphs;
