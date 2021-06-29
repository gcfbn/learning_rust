use crate::data::structures::{Edge, EdgeDescription, Graph, GraphBuilder, GraphParameters};
use crate::{BuildGraphError, Result};
use std::convert::TryFrom;
use std::fs;
use std::path::Path;

pub fn build_graph_from_file<P: AsRef<Path>>(filename: P) -> Result<Graph> {
    let filename = filename.as_ref();
    let input = fs::read_to_string(filename)?;

    let mut graph_file_reader = GraphFileReader::new(&input);

    let graph_parameters = graph_file_reader.graph_parameters()?;

    let mut graph_builder = GraphBuilder::new(graph_parameters);

    for (line_no, maybe_edge) in graph_file_reader.enumerate() {
        let add_edge = || -> Result<()> { Ok(graph_builder.add_edge(maybe_edge?)?) };

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
        let line = self.iter.next().ok_or(BuildGraphError::NotEnoughData)?;
        GraphParameters::try_from(line)
    }
}

impl<'a> Iterator for GraphFileReader<'a> {
    type Item = Result<Edge>;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(line2edge)
    }
}

fn line2edge(line: &str) -> Result<Edge> {
    let edge_description = EdgeDescription::try_from(line)?;
    Edge::try_from(edge_description)
}
