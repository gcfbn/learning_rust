use crate::Edge;
use parse_display::Display;
use thiserror::Error;

/// Type returned by functions in this crate
pub type Result<T, E = BuildGraphError> = std::result::Result<T, E>;

/// Enum containing all possible variants of errors returned by functions in this crate.
///
/// Derives [`thiserror::Error`] and [`core::fmt::Debug`], so errors could be easily printed out (using [`parse_display::Display`]).
/// Some errors contain variant of more specific enums.
///
/// # Examples
/// 1. Input contains edge that connects nodes '2' and '4', but declared number of nodes is 3
///
/// > "...".parse() should return an [`BuildGraphError::AddingEdgeError`]
///
/// ```
/// use graph::{Graph, BuildGraphError, AddingEdgeError, Result};
///
/// let maybe_graph: Result<Graph> = "3 2
///     1 2 100
///     2 4 100"
///     .parse();
///
/// assert!(maybe_graph.is_err());
/// assert_eq!(maybe_graph.unwrap_err().to_string(),
///     BuildGraphError::ErrorInGraphDescriptionFile {
///     line_no: 2,
///     error: Box::from(BuildGraphError::from(AddingEdgeError::WrongToIndex {
///         edge: "2 4 100".parse().unwrap(),
///         nodes_count: 3
///     })),
/// }.to_string());
/// ```
///
/// 2. Input contains only 2 edges, but declared number of edges is 3
///
/// > "...".parse() should return [`BuildGraphError::TooFewEdges`]
///
/// ```
/// use graph::{Graph, BuildGraphError, Result};
///
/// let maybe_graph: Result<Graph> = "4 3
///     1 2 500
///     1 4 350"
///     .parse();
///
/// assert!(maybe_graph.is_err());
/// assert_eq!(maybe_graph.unwrap_err().to_string(),
///     BuildGraphError::TooFewEdges {
///     current_count: 2,
///     declared: 3,
/// }.to_string());
/// ```
#[derive(Error, Debug)]
pub enum BuildGraphError {
    /// Kruskal's algorithm finds minimum spanning tree for connected graphs -
    /// containing path from any point to any other point in the graph
    #[error("graph is not connected!")]
    GraphNotConnected,

    /// It is possible to build a graph only if it's number of edges is equal to declared number of edges
    #[error("current count of edges {current_count} is less than declared {declared}")]
    TooFewEdges {
        /// Number of edges already inserted to the graph
        current_count: usize,

        /// Declared number of edges in the graph
        declared: usize,
    },

    /// Line containing edge data is invalid (missing or non-integer values)
    #[error("error parsing edge - {0}")]
    ParsingEdgeError(ParsingEdgeError),

    /// Can't add line to graph (index out of bounds or too many edges)
    #[error("error adding edge - {0}")]
    AddingEdgeError(AddingEdgeError),

    /// Graph parameters are missing or non-integer
    #[error("error parsing graph parameters - {0}")]
    GraphParametersParsingError(GraphParametersParsingError),

    /// Indicates, which line in input file is invalid and what's the error
    #[error("error in line {line_no}: {error}")]
    ErrorInGraphDescriptionFile {
        ///Number of line that caused the error
        line_no: usize,

        /// Variant of the error
        error: Box<BuildGraphError>,
    },

    /// Other standard input/output errors
    #[error(transparent)]
    StandardError(#[from] std::io::Error),
}

impl From<ParsingEdgeError> for BuildGraphError {
    fn from(e: ParsingEdgeError) -> Self {
        BuildGraphError::ParsingEdgeError(e)
    }
}

impl From<AddingEdgeError> for BuildGraphError {
    fn from(e: AddingEdgeError) -> Self {
        BuildGraphError::AddingEdgeError(e)
    }
}

impl From<GraphParametersParsingError> for BuildGraphError {
    fn from(e: GraphParametersParsingError) -> Self {
        BuildGraphError::GraphParametersParsingError(e)
    }
}

// -----------------------------------------------------------------------------

/// Enum with errors related to parsing graph edge parameters
#[derive(Debug, Display, PartialEq)]
pub enum ParsingEdgeError {
    /// Line is empty
    #[display("empty line")]
    EmptyLine,

    /// Line doesn't contain `to_index` value, which should be second value in the line
    #[display("missing `to_index` field")]
    MissingToIndexField,

    /// Line doesn't contain `weight` value, which should be third value in the line
    #[display("missing `weight` field")]
    MissingWeightField,

    /// Non-integer value as `from_index`
    #[display("from_index must be an integer, but it is: `{0}`")]
    FromIndexValueMustBeInteger(String),

    /// Non-integer value as `to_index`
    #[display("to_index must be an integer, but it is: `{0}`")]
    ToIndexValueMustBeInteger(String),

    /// Non-integer value as `weight`
    #[display("weight must be an integer, but it is: `{0}`")]
    WeightValueMustBeInteger(String),
}

// -----------------------------------------------------------------------------

/// Enum with errors related to adding an edge to the graph
#[derive(Debug, Display)]
pub enum AddingEdgeError {
    /// Graph already contains declared number of edges and can't add new edge
    #[display("max allowed count of edges is {edges_count} but you are trying to add a new edge {edge:?}")]
    TooManyEdges {
        /// Declared number of edges
        edges_count: usize,

        /// [`crate::Edge`] user is trying to add
        edge: Edge,
    },

    /// `from_index` field value is greater than number of nodes in the graph
    #[display("{edge:?} from_index field value is greater than nodes count `{nodes_count}` in graph !")]
    WrongFromIndex {
        /// [`crate::Edge`] user is trying to add
        edge: Edge,

        /// Declared number of nodes in the graph
        nodes_count: u32,
    },

    /// `to_index` field value is greater than number of nodes in the graph
    #[display("{edge:?} to_index field value is greater than nodes count `{nodes_count}` in graph !")]
    WrongToIndex {
        /// [`crate::Edge`] user is trying to add
        edge: Edge,

        /// Declared number of nodes in the graph
        nodes_count: u32,
    },
}

// -----------------------------------------------------------------------------

/// Enum with errors related to parsing graph parameters
#[derive(Debug, Display)]
pub enum GraphParametersParsingError {
    /// Input file is empty
    #[display("empty input")]
    EmptyInput,

    /// First line doesn't contain `edges_count` value, which should be second value in first line
    #[display("missing edges count value")]
    MissingEdgesCountValue,

    /// Non-integer value as `nodes_count`
    #[display("nodes count must be an integer, but it is: `{0}`")]
    NodesCountValueMustBeInteger(String),

    /// Non-integer value as `edges_count`
    #[display("edges count must be an integer, but it is: `{0}`")]
    EdgesCountValueIsNotInteger(String),
}
