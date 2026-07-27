//! Educational catalog of algorithms and data structures in Rust.
//!
//! The crate groups problems by solving family under [`patterns`]. Each module
//! keeps the implementation small enough to study and documents the core idea,
//! complexity, and representative examples.
//!
//! # Example
//!
//! ```
//! use rust_algorithms::patterns::binary_search::binary_search;
//!
//! assert_eq!(binary_search(vec![1, 3, 5, 8], 5), Some(2));
//! ```

/// Algorithm families organized by the pattern used to solve each problem.
pub mod patterns;
