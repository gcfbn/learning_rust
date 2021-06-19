use anyhow::{anyhow, Result as aResult};
use std::convert::TryFrom;

#[derive(Copy, Clone, Debug)]
pub struct Edge {
    pub from_index: u32,
    pub to_index: u32,
    pub weight: i32,
}

impl Edge {
    pub fn new(from_index: u32, to_index: u32, weight: i32) -> Edge {
        Edge {
            from_index,
            to_index,
            weight,
        }
    }
}

#[derive(Debug)]
pub struct EdgeDescription<'a> {
    pub from_index: &'a str,
    pub to_index: &'a str,
    pub weight: &'a str,
}

impl<'a> TryFrom<EdgeDescription<'a>> for Edge {
    type Error = String;

    fn try_from(edge_description: EdgeDescription<'a>) -> Result<Self, Self::Error> {
        fn build_error<'a>(
            edge_description: &EdgeDescription<'a>,
            field_name: &str,
            field_value: &str,
        ) -> String {
            format!(
                "creating graph edge from description `{:?}` has failed: {}={} is not an integer !",
                edge_description, field_name, field_value
            )
        }

        let parsed_from_index = edge_description.from_index.parse::<u32>().map_err(|_| {
            build_error(&edge_description, "from_index", edge_description.from_index)
        })?;
        let parsed_to_index = edge_description
            .to_index
            .parse::<u32>()
            .map_err(|_| build_error(&edge_description, "to_index", edge_description.to_index))?;
        let parsed_weight = edge_description
            .weight
            .parse::<i32>()
            .map_err(|_| build_error(&edge_description, "weight", edge_description.weight))?;

        Ok(Edge::new(parsed_from_index, parsed_to_index, parsed_weight))
    }
}

pub struct Graph {
    pub nodes_count: u32,
    pub edges: Vec<Edge>,
}

impl Graph {
    pub fn new(nodes_count: u32, edges: Vec<Edge>) -> Graph {
        Graph { nodes_count, edges }
    }
}

pub struct GraphBuilder {
    nodes_count: u32,
    max_edges_count: usize,
    edges: Vec<Edge>,
}

impl GraphBuilder {
    pub fn new(gp: GraphParameters) -> GraphBuilder {
        let GraphParameters {
            nodes_count,
            max_edges_count,
        } = gp;

        GraphBuilder {
            nodes_count,
            max_edges_count,
            edges: Vec::with_capacity(max_edges_count),
        }
    }

    pub fn add_edge(&mut self, edge: Edge) -> aResult<()> {
        if self.edges.len() < self.max_edges_count {
            if edge.from_index > self.nodes_count {
                return Err(anyhow!(
                    "add_edge has failed for edge number: {} - from_index {} is greater than {} !",
                    self.edges.len() + 1,
                    edge.from_index,
                    self.nodes_count
                ));
            }

            if edge.to_index > self.nodes_count {
                return Err(anyhow!(
                    "add_edge has failed for edge number: {} - to_index {} is greater than {} !",
                    self.edges.len() + 1,
                    edge.to_index,
                    self.nodes_count
                ));
            }

            self.edges.push(edge);
            Ok(())
        } else {
            Err(anyhow!(
                "max allowed count of edges is {} but you are trying to add a new edge {:?}",
                self.max_edges_count,
                edge
            ))
        }
    }

    pub fn build(self) -> aResult<Graph> {
        if self.edges.len() < self.max_edges_count {
            Err(anyhow!(
                "current count of edges {} is less than declared {}",
                self.edges.len(),
                self.max_edges_count
            ))
        } else {
            Ok(Graph::new(self.nodes_count, self.edges))
        }
    }
}

// -----------------------------------------------------------------------------

#[derive(Debug)]
pub struct GraphParameters {
    pub nodes_count: u32,
    pub max_edges_count: usize,
}

impl GraphParameters {
    pub fn new(nodes_count: u32, max_edges_count: usize) -> GraphParameters {
        GraphParameters {
            nodes_count,
            max_edges_count,
        }
    }
}
