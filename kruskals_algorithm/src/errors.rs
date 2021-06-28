use crate::data::{Edge, EdgeDescription};
use parse_display::Display;
use thiserror::Error;

pub type Result<T, E = BuildGraphError> = std::result::Result<T, E>;

#[derive(Error, Debug)]
pub enum BuildGraphError {
    #[error("graph is not connected!")]
    GraphNotConnected,

    #[error("current count of edges {current_count} is less than declared {declared}")]
    TooFewEdges { current_count: usize, declared: usize },

    #[error("invalid edge descritpion - {0}")]
    InvalidEdgeDescription(EdgeDescriptionError),

    #[error("max allowed count of edges is {edges_count} but you are trying to add a new edge {edge:?}")]
    TooManyEdges { edges_count: usize, edge: Edge },

    #[error("not enough data in input file")]
    NotEnoughData,

    #[error("{0}")]
    CreatingEdgeError(CreatingEdgeError),

    #[error("{0}")]
    GraphParametersParsingError(GraphParametersParsingError),

    #[error(transparent)]
    StandardError(#[from] std::io::Error),
}

impl From<CreatingEdgeError> for BuildGraphError {
    fn from(e: CreatingEdgeError) -> Self {
        BuildGraphError::CreatingEdgeError(e)
    }
}

impl From<EdgeDescriptionError> for BuildGraphError {
    fn from(e: EdgeDescriptionError) -> Self {
        BuildGraphError::InvalidEdgeDescription(e)
    }
}

impl From<GraphParametersParsingError> for BuildGraphError {
    fn from(e: GraphParametersParsingError) -> Self {
        BuildGraphError::GraphParametersParsingError(e)
    }
}

// -----------------------------------------------------------------------------

#[derive(Display, PartialEq, Debug)]
pub enum EdgeDescriptionError {
    #[display("empty input")]
    EmptyInput,
    #[display("missing `to_index` field")]
    MissingToIndexField,
    #[display("missing `weight` field")]
    MissingWeightField,
    #[display("`{edge:?}` from_index field value is greater than nodes count `{nodes_count}` in graph !")]
    WrongFromIndex { edge: Edge, nodes_count: u32 },
    #[display("`{edge:?}` to_index field value is greater than nodes count `{nodes_count}` in graph !")]
    WrongToIndex { edge: Edge, nodes_count: u32 },
}

// -----------------------------------------------------------------------------

#[derive(Debug, Display)]
#[display(
    "creating graph edge from description `{edge_description}` has failed: {field_name}={field_value} is not an \
     integer!"
)]
pub struct CreatingEdgeError {
    edge_description: String,
    field_name:       String,
    field_value:      String,
}

impl CreatingEdgeError {
    fn from_edge_description(edge_description: &EdgeDescription, field_name: &str, field_value: &str) -> Self {
        Self {
            edge_description: format!("{:?}", edge_description),
            field_name:       field_name.to_owned(),
            field_value:      field_value.to_owned(),
        }
    }

    pub fn from_edge_description_with_bad_from_index(edge_description: &EdgeDescription) -> Self {
        Self::from_edge_description(edge_description, "from_index", edge_description.from_index)
    }

    pub fn from_edge_description_with_bad_to_index(edge_description: &EdgeDescription) -> Self {
        Self::from_edge_description(edge_description, "to_index", edge_description.to_index)
    }

    pub fn from_edge_description_with_bad_weight(edge_description: &EdgeDescription) -> Self {
        Self::from_edge_description(edge_description, "weight", edge_description.weight)
    }
}

// -----------------------------------------------------------------------------

#[derive(Debug, Display)]
#[display("parsing graph parameters has failed: {parameter_name}={value} is not an integer!")]
pub struct GraphParametersParsingError {
    parameter_name: String,
    value:          String,
}

impl GraphParametersParsingError {
    fn from_non_integer_value(field_name: &str, value: &str) -> Self {
        Self {
            parameter_name: field_name.to_owned(),
            value:          value.to_owned(),
        }
    }

    pub fn from_non_integer_nodes_count(value: &str) -> Self {
        Self::from_non_integer_value("nodes_count", value)
    }

    pub fn from_non_integer_edges_count(value: &str) -> Self {
        Self::from_non_integer_value("edges_count", value)
    }
}
