//! Implementation of directed graph structure
//!
//! Result type for functions in this crate is [`crate::errors::BuildGraphResult`]
//!
//! # Example
//! ```
//! use graph::Graph;
//!
//! let graph: Graph = "4 3
//!     1 2 150
//!     1 3 220
//!     4 2 140"
//!     .parse()
//!     .unwrap();
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

mod adjacency_list;
mod dfs;
mod errors;
mod reader;
mod structures;

pub use crate::adjacency_list::{adjacency_list, AdjacencyList};
pub use crate::errors::{
    AddingEdgeError,
    BuildGraphError,
    BuildGraphResult,
    GraphParametersParsingError,
    ParsingEdgeError,
};
pub use crate::reader::build_graph;
pub use crate::structures::{Edge, EdgeDescription, Graph, GraphBuilder, GraphParameters};
