// extern these crates only when running tests
#[cfg(test)]
extern crate test_case;

mod dfs;
mod errors;
mod reader;
mod structures;

pub use crate::errors::{AddingEdgeError, BuildGraphError, GraphParametersParsingError, ParsingEdgeError, Result};
pub use crate::reader::build_graph_from_file;
pub use crate::structures::{Edge, EdgeDescription, Graph};
