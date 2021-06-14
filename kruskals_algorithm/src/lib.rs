use std::error::Error;

mod data;

pub fn run(filename: &str) -> Result<data::structures::Graph, Box<dyn Error>> {
    graph = data::reader::build_graph_from_input(filename)
}