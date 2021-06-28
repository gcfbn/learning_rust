use crate::data::structures::{Edge, EdgeDescription, Graph, GraphBuilder, GraphParameters};
use crate::{BuildGraphError, Result};
use std::convert::TryFrom;
use std::fs;
use std::path::Path;

pub fn build_graph_from_file<P: AsRef<Path>>(filename: P) -> Result<Graph> {
    let filename = filename.as_ref();
    let input = fs::read_to_string(filename)?;

    /*
        let mut iter = input.lines();

        let first_line = iter.next().ok_or(BuildGraphError::NotEnoughData)?;
        let graph_parameters = GraphParameters::try_from(first_line)?;

        let mut graph_builder = GraphBuilder::new(graph_parameters);

        for line in iter {
            let edge_description = EdgeDescription::try_from(line)?;
            let edge = Edge::try_from(edge_description)?;

            graph_builder.add_edge(edge)?;
        }
    */

    let mut graph_file_reader = GraphFileReader::new(&input);

    let graph_parameters = graph_file_reader.graph_parameters()?;

    let mut graph_builder = GraphBuilder::new(graph_parameters);

    for maybe_edge in graph_file_reader {
        let edge = maybe_edge?;
        graph_builder.add_edge(edge)?;
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
        /*
        let n = self.iter.next().ok_or(BuildGraphError::NotEnoughData)?;
        let m = self.iter.next().ok_or(BuildGraphError::NotEnoughData)?;

        let n = n.parse::<u32>().map_err(|_| BuildGraphError::ParsingError {
            parameter_name: "n".to_owned(),
            value:          n.to_owned(),
        })?;
        let m = m.parse::<usize>().map_err(|_| BuildGraphError::ParsingError {
            parameter_name: "m".to_owned(),
            value:          m.to_owned(),
        })?;
        */

        Ok(GraphParameters::new(1, 2))
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
