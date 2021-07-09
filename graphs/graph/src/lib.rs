//! Implementation of graph structure and few graph algorithms
//!
//! Result type for functions in this crate is [`crate::errors::Result`]

// extern these crates only when running tests
#[cfg(test)]
extern crate test_case;

mod dfs;
mod errors;
mod reader;
mod structures;

pub use crate::errors::{AddingEdgeError, BuildGraphError, GraphParametersParsingError, ParsingEdgeError, Result};
pub use crate::reader::{build_graph_from_file, build_graph_from_string};
pub use crate::structures::{Edge, EdgeDescription, Graph};
