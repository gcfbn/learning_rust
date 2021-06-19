mod data;
mod kruskal;

use anyhow::Error;
use std::path::Path;

use crate::data::build_graph_from_file;
use crate::data::Graph;

pub fn run<P>(filename: P) -> Result<i32, Error>
where
    P: AsRef<Path>,
{
    let graph = build_graph_from_file(filename)?;
    Ok(kruskal::algorithm::calculate_min_total_weight(graph))
}
