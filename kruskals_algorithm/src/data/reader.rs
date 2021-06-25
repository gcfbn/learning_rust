use crate::data::structures::{Edge, EdgeDescription, Graph, GraphBuilder, GraphParameters};
use crate::{BuildGraphError, Result};
use std::convert::TryFrom;
use std::fs;
use std::path::Path;

pub fn build_graph_from_file<P: AsRef<Path>>(filename: P) -> Result<Graph> {
    let filename = filename.as_ref();
    let input = fs::read_to_string(filename)?;
    let mut task_file_reader = GraphFileReader::new(&input);

    let graph_parameters = task_file_reader.graph_parameters()?;

    let mut graph_builder = GraphBuilder::new(graph_parameters);

    for edge_description in task_file_reader {
        let current_edge = Edge::try_from(edge_description)?;

        graph_builder.add_edge(current_edge)?;
    }

    graph_builder.build()
}

// -----------------------------------------------------------------------------

type DataIter<'a> = std::str::SplitWhitespace<'a>;

struct GraphFileReader<'a> {
    iter: DataIter<'a>,
}

impl<'a> GraphFileReader<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            iter: input.split_whitespace(),
        }
    }

    pub fn graph_parameters(&mut self) -> Result<GraphParameters> {
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

        Ok(GraphParameters::new(n, m))
    }
}

impl<'a> Iterator for GraphFileReader<'a> {
    type Item = EdgeDescription<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let from_index = self.iter.next()?;
        let to_index = self.iter.next()?;
        let weight = self.iter.next()?;
        Some(EdgeDescription {
            from_index,
            to_index,
            weight,
        })
    }
}
