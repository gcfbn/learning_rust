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

    #[error(
        "add_edge has failed for edge number: {edge_number} - from_index {from_index} is greater than {nodes_count} !"
    )]
    WrongFromIndex {
        edge_number: usize,
        from_index:  u32,
        nodes_count: u32,
    },

    #[error(
        "add_edge has failed for edge number: {edge_number} - to_index {to_index} is greater than {nodes_count} !"
    )]
    WrongToIndex {
        edge_number: usize,
        to_index:    u32,
        nodes_count: u32,
    },

    #[error("max allowed count of edges is {max_edges_count} but you are trying to add a new edge {edge:?}")]
    TooManyEdges {
        max_edges_count: usize,
        edge:            Edge,
    },

    #[error("not enough data in input file")]
    NotEnoughData,

    #[error("{0}")]
    CreatingEdgeError(CreatingEdgeError),

    #[error("parsing graph parameters has failed: {parameter_name}={value} is not an integer!")]
    ParsingError {
        parameter_name: String,
        value:          String,
    },

    #[error(transparent)]
    StandardError(#[from] std::io::Error),
}

impl From<CreatingEdgeError> for BuildGraphError {
    fn from(e: CreatingEdgeError) -> Self {
        BuildGraphError::CreatingEdgeError(e)
    }
}

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
    pub fn from_edge_description(edge_description: &EdgeDescription, field_name: &str, field_value: &str) -> Self {
        Self {
            edge_description: format!("{:?}", edge_description),
            field_name:       field_name.to_owned(),
            field_value:      field_value.to_owned(),
        }
    }
}
