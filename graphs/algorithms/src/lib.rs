//! Implementation of few graph theory algorithms
//!
//! # Algorithms:
//!
//! * Kruskal's algorithm

// extern these crates only when running tests
#[cfg(test)]
extern crate test_case;

mod kruskal;

pub use kruskal::calculate_min_total_weight;
