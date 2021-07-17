#![warn(missing_docs)]
use graph::BuildGraphError;
use parse_display::Display;
use std::io::Error as ioError;
use thiserror::Error;

/// Result returned by graph generator
pub type Result<T, E = GenerateGraphError> = std::result::Result<T, E>;

/// Enum containing variants of errors that could occur in bin/main.rs  
#[derive(Error, Debug)]
pub enum RunnerError {
    /// Error connected with building graph from file or string
    #[error("graph builder error - {0}")]
    BuildGraphError(BuildGraphError),

    /// Error connected with generating graph file
    #[error("graph file generator error - {0}")]
    GraphFileGeneratorError(GenerateGraphError),

    /// Error with processing command line arguments
    #[error("error processing command line arguments")]
    CommandLineArgsError(#[from] clap::Error),
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

/// Enum containing variants of errors that might occur during generating graph
#[derive(Debug, Display)]
pub enum GenerateGraphError {
    /// Connected graph must contain at least n-1 edges (where `n` is number of nodes in the graph)
    #[display("{edges_count} edges is not enough to generate connected graph with {nodes_count} nodes")]
    TooFewEdgesForConnectedGraph {
        /// number of edges in the graph
        edges_count: u32,

        /// number of nodes in the graph
        nodes_count: u32,
    },

    /// Couldn't create directory specified in output filepath
    #[display("creating directory for output file failed with error - {0}")]
    CreatingDirectoryError(ioError),

    /// Couldn't create file containing graph data
    #[display("creating output file failed with error - {0}")]
    CreatingFileError(ioError),

    /// Couldn't write to output file
    #[display("writing error - {0}")]
    WritingError(ioError),
}
