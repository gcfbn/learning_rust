use std::error::Error;

mod data;

pub fn run(filename: &str) -> Result<data::reader::Parameters, Box<dyn Error>> {
    data::reader::read_graph_data(filename)
}