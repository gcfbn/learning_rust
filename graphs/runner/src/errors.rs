use algorithms::AlgorithmError;
use graph::BuildGraphError;
use parse_display::Display;
use std::io::Error as ioError;
use thiserror::Error;

/// Result returned by the application
pub type RunnerResult<T, E = RunnerError> = std::result::Result<T, E>;

/// Result returned by graph generator
pub type GenerateGraphResult<T, E = GenerateGraphError> = std::result::Result<T, E>;

/// Enum containing variants of errors that could occur in bin/main.rs
/// Derives [`thiserror::Error`] and [`core::fmt::Debug`], so errors could be easily printed out (using [`parse_display::Display`]).
///
/// # Example
/// Building graph from non-existing file should return an error. Clap checks if source file exists, so returned variant
/// should be [`RunnerError::CommandLineArgsError`]. We use `run-algorithm` subcommand, because it builds graph from given path and then
/// runs Kruskal's algoritm.
///
/// ```
/// use runner::{SubCommand, RunnerError};
/// use anyhow::anyhow;
///
/// let non_existing_file = "non_existing_file.txt";
///
/// let command_name = "run-algorithm";
/// let args = format!("graph-file {}", non_existing_file);
///
/// let result = SubCommand::try_from_name_and_args(command_name, &args);
/// assert!(result.is_err());
///
/// let expected_error = RunnerError::from(
///     clap::Error::with_description(format!("the file does not exist: {}", non_existing_file),
///     clap::ErrorKind::ValueValidation));
/// assert_eq!(result.unwrap_err().to_string(), expected_error.to_string());
/// ```
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

    #[error("algorithm error - {0}")]
    AlgorithmError(AlgorithmError),
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

impl From<AlgorithmError> for RunnerError {
    fn from(e: AlgorithmError) -> Self {
        RunnerError::AlgorithmError(e)
    }
}

// -----------------------------------------------------------------------------

/// Enum containing variants of errors that might occur during generating graph
///
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
