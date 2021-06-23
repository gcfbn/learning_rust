// extern these crates only when running tests
#[cfg(test)]
extern crate assert_matches;

#[cfg(test)]
extern crate test_case;

mod algorithm;
mod data;

use anyhow::Result as aResult;
use std::path::Path;
use thiserror::Error;

use crate::algorithm::calculate_min_total_weight;
use crate::data::build_graph_from_file;
use crate::data::{Graph, Edge, EdgeDescription};

pub fn run<P>(filename: P) -> aResult<i32>
    where
        P: AsRef<Path>,
{
    let graph = build_graph_from_file(filename)?;
    Ok(calculate_min_total_weight(graph))
}

#[derive(Error, Debug)]
pub enum KruskalsAlgorithmError<'a> {
    #[error("graph is not connected!")]
    GraphNotConnected,

    #[error("current count of edges {current_count} is less than declared {declared}")]
    TooFewEdges { current_count: usize, declared: usize },

    #[error("add_edge has failed for edge number: {edge_number} - \
    from_index {from_index} is greater than {nodes_count} !")]
    WrongFromIndex { edge_number: usize, from_index: u32, nodes_count: u32 },

    #[error("add_edge has failed for edge number: {edge_number} - \
    to_index {to_index} is greater than {nodes_count} !")]
    WrongToIndex { edge_number: usize, to_index: u32, nodes_count: u32 },

    #[error("max allowed count of edges is {max_edges_count} but you are trying to add a new edge {edge:?}")]
    TooManyEdges { max_edges_count: usize, edge: Edge },

    #[error("not enough data in input file")]
    NotEnoughData,

    #[error("creating graph edge from description `{:?}` has failed: \
    {field_name}={field_value} is not an integer!")]
    CreatingEdgeError { edge_description: &'a EdgeDescription<'a>, field_name: &'a str, field_value: &'a str },

    #[error("parsing graph parameters has failed: {parameter_name}={value} is not an integer!")]
    ParsingError { parameter_name: &'a str, value: &'a str },

    #[error(transparent)]
    StandardError(#[from] std::io::Error),
}
