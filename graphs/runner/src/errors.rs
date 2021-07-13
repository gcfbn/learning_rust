use graph::BuildGraphError;
use parse_display::Display;
use std::io::Error as ioError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum RunnerError {
    #[error("graph builder error - {0}")]
    BuildGraphError(BuildGraphError),

    #[error("graph file generator error - {0}")]
    GraphFileGeneratorError(GraphFileGeneratorError),
}

impl From<BuildGraphError> for RunnerError {
    fn from(e: BuildGraphError) -> Self {
        RunnerError::BuildGraphError(e)
    }
}

impl From<GraphFileGeneratorError> for RunnerError {
    fn from(e: GraphFileGeneratorError) -> Self {
        RunnerError::GraphFileGeneratorError(e)
    }
}

// -----------------------------------------------------------------------------

#[derive(Debug, Display)]
pub enum GraphFileGeneratorError {
    #[display(
        "given number of edges `{edges_count}` is to small to generate connected graph containing given number of \
         nodes `{nodes_count}`"
    )]
    TooFewEdgesForConnectedGraph { edges_count: u32, nodes_count: u32 },

    #[display("creating directory for output file failed with error - {0}")]
    CreatingDirectoryError(ioError),

    #[display("creating output file failed with error - {0}")]
    CreatingFileError(ioError),

    #[display("writing error - {0}")]
    WritingError(ioError),
}
