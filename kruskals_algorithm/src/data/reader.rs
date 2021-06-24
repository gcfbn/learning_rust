use crate::data::structures::{Edge, EdgeDescription, Graph, GraphBuilder, GraphParameters};
use crate::KruskalsAlgorithmError;
use std::convert::TryFrom;
use std::fs;
use std::path::Path;

pub fn build_graph_from_file<P: AsRef<Path>>(filename: P) -> Result<Graph, KruskalsAlgorithmError<'static>> {
    let filename = filename.as_ref();
    let input = fs::read_to_string(filename)?;
    let mut task_file_reader = TaskFileReader::new(&input);

    // compiler error: returns a value referencing data owned by the current function
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

struct TaskFileReader<'a> {
    iter: DataIter<'a>,
}

impl<'a> TaskFileReader<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            iter: input.split_whitespace(),
        }
    }

    pub fn graph_parameters(&mut self) -> Result<GraphParameters, KruskalsAlgorithmError<'a>> {
        let n = self.iter.next().ok_or(KruskalsAlgorithmError::NotEnoughData)?;
        let m = self.iter.next().ok_or(KruskalsAlgorithmError::NotEnoughData)?;

        let n = n.parse::<u32>().map_err(|_| KruskalsAlgorithmError::ParsingError {
            parameter_name: "n",
            value:          n,
        })?;
        let m = m.parse::<usize>().map_err(|_| KruskalsAlgorithmError::ParsingError {
            parameter_name: "m",
            value:          m,
        })?;

        Ok(GraphParameters::new(n, m))
    }
}

impl<'a> Iterator for TaskFileReader<'a> {
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
