mod cli;
mod errors;
mod graph_file_generator;

pub use cli::*;
pub use errors::{GraphFileGeneratorError, RunnerError};
pub use graph_file_generator::generate_graph;

pub fn test() {
    println!("TEST");
}
