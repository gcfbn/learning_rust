mod cli;
mod errors;
mod generate_graph;

pub use cli::*;
pub use errors::{GraphFileGeneratorError, RunnerError};
pub use generate_graph::generate_graph;

pub fn test() {
    println!("TEST");
}
