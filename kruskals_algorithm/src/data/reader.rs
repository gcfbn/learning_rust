use crate::data::structures::{Edge, EdgeDescription, Graph, GraphBuilder, GraphParameters};
use crate::{BuildGraphError, Result};
use std::convert::TryFrom;
use std::fs;
use std::path::Path;

pub fn build_graph_from_file<P: AsRef<Path>>(filename: P) -> Result<Graph> {
    let filename = filename.as_ref();
    let input = fs::read_to_string(filename)?;

    let mut iter = input.lines();

    let first_line = iter.next().ok_or(BuildGraphError::NotEnoughData)?;
    let graph_parameters = GraphParameters::try_from(first_line)?;

    let mut graph_builder = GraphBuilder::new(graph_parameters);

    for line in iter {
        let edge_description = EdgeDescription::try_from(line)?;
        let edge = Edge::try_from(edge_description)?;

        graph_builder.add_edge(edge)?;
    }

    graph_builder.build()
}
