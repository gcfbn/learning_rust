// extern these crates only when running tests
#[cfg(test)]
extern crate assert_matches;

#[cfg(test)]
extern crate test_case;

mod algorithm;
mod data;

use anyhow::Result as aResult;
use std::path::Path;

use crate::algorithm::calculate_min_total_weight;
use crate::data::build_graph_from_file;
use crate::data::Graph;

pub fn run<P>(filename: P) -> aResult<i32>
    where
        P: AsRef<Path>,
{
    let graph = build_graph_from_file(filename)?;
    Ok(calculate_min_total_weight(graph))
}
