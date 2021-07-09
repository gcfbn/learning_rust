//! Implementation of directed graph structure
//!
//! Result type for functions in this crate is [`crate::errors::Result`]
//!
//! # Example
//! ```
//! use graph::build_graph_from_string;
//!
//! let input = String::from("4 3
//! 1 2 150
//! 1 3 220
//! 4 2 140
//! ");
//!
//! let graph = build_graph_from_string(input).unwrap();
//!
//! assert_eq!(graph.nodes_count, 4);
//! assert_eq!(graph.edges.len(), 3);
//!
//! let second_edge = graph.edges[1];
//!
//! assert_eq!(second_edge.from_index, 1);
//! assert_eq!(second_edge.to_index, 3);
//! assert_eq!(second_edge.weight, 220);
//! ```

// extern these crates only when running tests
#[cfg(test)]
extern crate test_case;

mod dfs;
mod errors;
mod reader;
mod structures;

pub use crate::errors::{AddingEdgeError, BuildGraphError, GraphParametersParsingError, ParsingEdgeError, Result};
pub use crate::reader::{build_graph_from_file, build_graph_from_string};
pub use crate::structures::{Edge, EdgeDescription, Graph, GraphBuilder, GraphParameters};
