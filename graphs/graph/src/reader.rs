use super::structures::{Edge, Graph, GraphBuilder, GraphParameters};
use crate::{BuildGraphError, BuildGraphResult as Result, GraphParametersParsingError};
use std::convert::From;
use std::convert::TryFrom;
use std::fs;
use std::path::Path;
use std::str::FromStr;

pub enum DataSource<'r> {
    String(&'r str),
    File(&'r Path),
}

impl<'r> From<&'r str> for DataSource<'r> {
    fn from(s: &'r str) -> Self {
        DataSource::String(s)
    }
}

impl<'r> From<&'r Path> for DataSource<'r> {
    fn from(filename: &'r Path) -> Self {
        DataSource::File(filename)
    }
}

impl<'r, T> From<&'r T> for DataSource<'r>
where
    T: AsRef<Path>,
{
    fn from(filename: &'r T) -> Self {
        DataSource::File(filename.as_ref())
    }
}

/// Builds a directed graph from the data source with specific format
///
/// Data source: string or file with graph description
///
/// # Arguments
///
/// * `data_source` - a reference to String, Path or PathBuf
pub fn build_graph<'r, DS>(data_source: DS) -> Result<Graph>
where
    DS: Into<DataSource<'r>>,
{
    let data_source: DataSource = data_source.into();
    match data_source {
        DataSource::String(s) => build_graph_from_string(s),
        DataSource::File(filename) => build_graph_from_file(filename),
    }
}

/// Builds a directed graph from txt file with specific format
///
/// File should be formatted as in [`build_graph_from_string`]
///
/// # Arguments
///
/// * `filename` - path to file containing input
fn build_graph_from_file<P: AsRef<Path>>(filename: P) -> Result<Graph> {
    let filename = filename.as_ref();
    let input = fs::read_to_string(filename)?;
    build_graph_from_string(input.as_str())
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
/// use graph::Graph;
///
/// let graph: Graph = "4 3
///     1 2 100
///     2 3 200
///     4 1 125"
///     .parse()
///     .unwrap();
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
fn build_graph_from_string(input: &str) -> Result<Graph> {
    let mut graph_file_reader = GraphDescriptionReader::new(input);

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

impl FromStr for Graph {
    type Err = BuildGraphError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        build_graph_from_string(s)
    }
}

type DataIter<'a> = std::str::Lines<'a>;

struct GraphDescriptionReader<'a> {
    iter: DataIter<'a>,
}

impl<'a> GraphDescriptionReader<'a> {
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

impl<'a> Iterator for GraphDescriptionReader<'a> {
    type Item = Result<Edge>;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(str::parse)
    }
}
