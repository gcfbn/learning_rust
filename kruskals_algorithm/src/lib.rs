extern crate assert_matches;

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
