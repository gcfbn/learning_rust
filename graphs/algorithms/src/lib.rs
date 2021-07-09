//! Implementation of few graph theory algorithms
//!
//! # Algorithms
//!
//! * Kruskal's algorithm ([`calculate_min_total_weight`])
//!
//! # Example
//! ```
//! use graph::build_graph_from_string;
//! use algorithms::calculate_min_total_weight;
//!
//! let input = String::from("3 4
//! 1 2 100
//! 2 1 80
//! 3 1 90
//! 1 3 110
//! ");
//!
//! let graph = build_graph_from_string(input).unwrap();
//!
//! let minimum_spanning_tree_weight = calculate_min_total_weight(graph);
//!
//! assert_eq!(minimum_spanning_tree_weight, 170);
//! ```

// extern these crates only when running tests
#[cfg(test)]
extern crate test_case;

mod kruskal;

pub use kruskal::calculate_min_total_weight;
