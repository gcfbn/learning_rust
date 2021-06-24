// extern these crates only when running tests
#[cfg(test)]
extern crate test_case;

mod algorithm;
mod data;
mod errors;

use std::path::Path;

use crate::algorithm::calculate_min_total_weight;
use crate::data::build_graph_from_file;
use crate::data::Graph;

pub use crate::errors::{CreatingEdgeError, KruskalsAlgorithmError};

pub fn run<P>(filename: P) -> Result<i32, KruskalsAlgorithmError>
where
    P: AsRef<Path>,
{
    let graph = build_graph_from_file(filename)?;
    Ok(calculate_min_total_weight(graph))
}
