// extern these crates only when running tests
#[cfg(test)]
extern crate test_case;

mod algorithm;
mod data;
mod errors;

use std::path::Path;

use crate::algorithm::calculate_min_total_weight;
use crate::data::build_graph_from_file;

pub use crate::data::{Edge, EdgeDescription};
pub use crate::errors::{AddingEdgeError, BuildGraphError, GraphParametersParsingError, ParsingEdgeError, Result};

pub fn run<P>(filename: P) -> Result<i32>
where
    P: AsRef<Path>,
{
    let graph = build_graph_from_file(filename)?;
    Ok(calculate_min_total_weight(graph))
}
