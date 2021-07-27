#![warn(missing_docs)]
use graph::BuildGraphError;
use parse_display::Display;
use std::io::Error as ioError;
use thiserror::Error;

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

/// Result returned by graph generator
pub type Result<T, E = GenerateGraphError> = std::result::Result<T, E>;

/// Enum containing variants of errors that might occur during generating graph
/// Derives [`thiserror::Error`] and [`core::fmt::Debug`], so errors could be easily printed out (using [`parse_display::Display`]).
/// Some errors contain variant of more specific enums.
///
/// # Example
/// Input contains invalid argument - edges_count is to small to create connected graph.
///
/// > runner::generate_graph() should return [`GenerateGraphError::TooFewEdgesForConnectedGraph`]
/// ```
/// use runner::{SubCommand, RunnerError, GenerateGraphError, GenerateGraphFileArgs};
/// use SubCommand::GenerateGraphFile;
///
/// let args = "--graph-file aaa.txt --nodes-count 5 --edges-count 3 --max-weight 100".parse::<GenerateGraphFileArgs>().unwrap();
/// let result = runner::generate_graph(&args);
///
/// assert!(result.is_err());
///
/// let expected_error = GenerateGraphError::TooFewEdgesForConnectedGraph {
///     edges_count: 3,
///     nodes_count: 5,
/// };
///
/// assert_eq!(result.unwrap_err().to_string(), expected_error.to_string());
/// ```
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
