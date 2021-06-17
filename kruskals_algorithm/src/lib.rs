use anyhow::Error;
use std::path::Path;

mod data;
mod kruskal;

pub fn run(filename: &Path) -> Result<i32, Error> {
    let graph = data::reader::build_graph_from_input(filename)?;
    Ok(kruskal::algorithm::calculate_min_total_weight(graph))
}