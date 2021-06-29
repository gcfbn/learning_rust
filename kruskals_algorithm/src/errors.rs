use crate::data::Edge;
use parse_display::Display;
use thiserror::Error;

pub type Result<T, E = BuildGraphError> = std::result::Result<T, E>;

#[derive(Error, Debug)]
pub enum BuildGraphError {
    #[error("graph is not connected!")]
    GraphNotConnected,

    #[error("current count of edges {current_count} is less than declared {declared}")]
    TooFewEdges { current_count: usize, declared: usize },

    #[error("error parsing edge - {0}")]
    ParsingEdgeError(ParsingEdgeError),

    #[error("error adding edge - {0}")]
    AddingEdgeError(AddingEdgeError),

    #[error("error parsing graph parameters - {0}")]
    GraphParametersParsingError(GraphParametersParsingError),

    #[error("error in line {line_no}: {error}")]
    ErrorInGraphDescriptionFile {
        line_no: usize,
        error:   Box<BuildGraphError>,
    },

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

#[derive(Debug, Display, PartialEq)]
pub enum ParsingEdgeError {
    #[display("empty line")]
    EmptyLine,

    #[display("missing `to_index` field")]
    MissingToIndexField,

    #[display("missing `weight` field")]
    MissingWeightField,

    #[display("from_index must be an integer, but it is: `{0}`")]
    FromIndexValueMustBeInteger(String),

    #[display("to_index must be an integer, but it is: `{0}`")]
    ToIndexValueMustBeInteger(String),

    #[display("weight must be an integer, but it is: `{0}`")]
    WeightValueMustBeInteger(String),
}

// -----------------------------------------------------------------------------

#[derive(Debug, Display)]
pub enum AddingEdgeError {
    #[display("max allowed count of edges is {edges_count} but you are trying to add a new edge {edge:?}")]
    TooManyEdges { edges_count: usize, edge: Edge },

    #[display("{edge:?} from_index field value is greater than nodes count `{nodes_count}` in graph !")]
    WrongFromIndex { edge: Edge, nodes_count: u32 },

    #[display("{edge:?} to_index field value is greater than nodes count `{nodes_count}` in graph !")]
    WrongToIndex { edge: Edge, nodes_count: u32 },
}

// -----------------------------------------------------------------------------

#[derive(Debug, Display)]
pub enum GraphParametersParsingError {
    #[display("empty input")]
    EmptyInput,

    #[display("missing edges count value")]
    MissingEdgesCountValue,

    #[display("nodes count must be an integer, but it is: `{0}`")]
    NodesCountValueMustBeInteger(String),

    #[display("edges count must be an integer, but it is: `{0}`")]
    EdgesCountValueIsNotInteger(String),
}
