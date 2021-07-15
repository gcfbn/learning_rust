use graph::BuildGraphError;
use parse_display::Display;
use std::io::Error as ioError;
use thiserror::Error;

pub type Result<T, E = GenerateGraphError> = std::result::Result<T, E>;

#[derive(Error, Debug)]
pub enum RunnerError {
    #[error("graph builder error - {0}")]
    BuildGraphError(BuildGraphError),

    #[error("graph file generator error - {0}")]
    GraphFileGeneratorError(GenerateGraphError),

    #[error("failed creating command with command_name='{command_name}' and args={args}")]
    SubcommandCreatingError { command_name: String, args: String },
}

impl From<BuildGraphError> for RunnerError {
    fn from(e: BuildGraphError) -> Self {
        RunnerError::BuildGraphError(e)
    }
}

impl From<GenerateGraphError> for RunnerError {
    fn from(e: GenerateGraphError) -> Self {
        RunnerError::GraphFileGeneratorError(e)
    }
}

// -----------------------------------------------------------------------------

#[derive(Debug, Display)]
pub enum GenerateGraphError {
    #[display("{edges_count} edges is not enough to generate connected graph with {nodes_count} nodes")]
    TooFewEdgesForConnectedGraph { edges_count: u32, nodes_count: u32 },

    #[display("creating directory for output file failed with error - {0}")]
    CreatingDirectoryError(ioError),

    #[display("creating output file failed with error - {0}")]
    CreatingFileError(ioError),

    #[display("writing error - {0}")]
    WritingError(ioError),
}
