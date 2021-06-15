use std::error::Error;

mod data;
mod kruskal;

pub fn run(filename: &str) -> Result<i32, Box<dyn Error>> {
    let graph = data::reader::build_graph_from_input(filename)?;
    Ok(kruskal::algorithm::calculate_min_total_weight(graph))
}