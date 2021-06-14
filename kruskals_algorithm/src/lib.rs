use std::error::Error;

mod data;

pub fn run(filename: &str) -> Result<data::structures::Graph, Box<dyn Error>> {
    data::reader::read_graph_data(filename)
}