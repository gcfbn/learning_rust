#![warn(missing_docs)]

use super::structures::{Edge, Graph, GraphBuilder, GraphParameters};
use crate::{BuildGraphError, GraphParametersParsingError, Result};
use std::convert::TryFrom;
use std::fs;
use std::path::Path;


/// Builds a directed graph from txt file with specific format
///
/// File should be formatted as in [`build_graph_from_string`]
///
/// # Arguments
///
/// * `filename` - path to file containing input
pub fn build_graph_from_file<P: AsRef<Path>>(filename: P) -> Result<Graph> {
    let filename = filename.as_ref();
    let input = fs::read_to_string(filename)?;
    build_graph_from_string(input)
}

/// Builds a directed graph from string with specific format
///
/// First line of string should contain two positive integers - number of nodes in the graph (`nodes_count`)
/// and number of edges in the graph (`edges_count`).
///
/// Then, every line describes one of the `edges_count` edges and contains three integers:
///
/// * First - Index of node where edge starts ([`Edge::from_index`])
/// * Second - Index of node where edge ends ([`Edge::to_index`])
/// * Third - weight of the edge ([`Edge::weight`])
///
/// # Example
/// ```
/// use graph::build_graph_from_string;
///
/// let input = String::from(
/// "4 3
/// 1 2 100
/// 2 3 200
/// 4 1 125");
///
/// let graph = build_graph_from_string(input).unwrap();
///
/// assert_eq!(graph.nodes_count, 4);
/// assert_eq!(graph.edges.len(), 3);
/// assert_eq!(graph.edges[0], "1 2 100".parse().unwrap());
/// assert_eq!(graph.edges[1], "2 3 200".parse().unwrap());
/// assert_eq!(graph.edges[2], "4 1 125".parse().unwrap());
///```
/// # Arguments
///
/// * `input` - string containing graph data
pub fn build_graph_from_string(input: String) -> Result<Graph> {
    let mut graph_file_reader = GraphFileReader::new(&input);

    let graph_parameters = graph_file_reader.graph_parameters()?;

    let mut graph_builder = GraphBuilder::new(graph_parameters);

    for (line_no, maybe_edge) in graph_file_reader.enumerate() {
        let add_edge = || -> Result<()> { graph_builder.add_edge(maybe_edge?) };

        add_edge().map_err(|error| BuildGraphError::ErrorInGraphDescriptionFile {
            line_no: line_no + 1,
            error:   Box::new(error),
        })?;
    }

    graph_builder.build()
}

type DataIter<'a> = std::str::Lines<'a>;

struct GraphFileReader<'a> {
    iter: DataIter<'a>,
}

impl<'a> GraphFileReader<'a> {
    pub fn new(input: &'a str) -> Self {
        Self { iter: input.lines() }
    }

    pub fn graph_parameters(&mut self) -> Result<GraphParameters> {
        let line = self
            .iter
            .next()
            .ok_or_else(|| BuildGraphError::from(GraphParametersParsingError::EmptyInput))?;
        GraphParameters::try_from(line)
    }
}

impl<'a> Iterator for GraphFileReader<'a> {
    type Item = Result<Edge>;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(str::parse)
    }
}
