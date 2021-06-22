use anyhow::{anyhow, Result as aResult};
use std::convert::TryFrom;
use crate::data::dfs::is_connected as dfs_is_connected;

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

#[derive(Debug)]
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

    // checks if there is a path from any node to any other node
    fn is_connected(&self) -> bool {
        dfs_is_connected(&self.edges, self.nodes_count)
    }

    pub fn build(self) -> aResult<Graph> {
        if self.edges.len() < self.max_edges_count {
            Err(anyhow!(
                "current count of edges {} is less than declared {}",
                self.edges.len(),
                self.max_edges_count
            ))
        } else if !self.is_connected() {
            Err(anyhow!("graph is not connected!"))
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

#[cfg(test)]
#[macro_use]
mod tests {
    use crate::data::structures::{EdgeDescription, Edge, GraphParameters, GraphBuilder};
    use std::convert::TryFrom;
    use assert_matches::assert_matches;
    use anyhow::{anyhow, Result as aResult};
    use crate::data::Graph;

    #[test]
    fn create_edge_ok() {
        let edge_description = EdgeDescription {
            from_index: "1",
            to_index: "5",
            weight: "200",
        };
        let expected = Edge::new(1, 5, 200);
        let actual = Edge::try_from(edge_description).unwrap();
        assert_matches!(expected, actual);
    }

    #[test]
    fn create_edge_err() {
        let edge_description = EdgeDescription {
            from_index: "1",
            to_index: "a",
            weight: "130",
        };
        let expected: Result<Edge, String> = Err(format!("creating graph edge from description `{:?}` has failed: \
        to_index=a is not an integer !", edge_description));

        let actual = Edge::try_from(edge_description);
        assert_matches!(expected,actual );
    }

    fn create_test_graph_builder() -> GraphBuilder {
        let graph_parameters = GraphParameters {
            nodes_count: 3,
            max_edges_count: 2,
        };
        GraphBuilder::new(graph_parameters)
    }

    #[test]
    fn too_many_edges() {
        let mut graph_builder = create_test_graph_builder();
        graph_builder.add_edge(Edge { from_index: 1, to_index: 3, weight: 200 });
        graph_builder.add_edge(Edge { from_index: 2, to_index: 1, weight: 50 });

        let third_edge = Edge { from_index: 3, to_index: 4, weight: 170 };
        let expected: aResult<()> =
            Err(anyhow!("max allowed count of edges is 2 but you are trying to add a new edge {:?}",
                third_edge));
        let actual = graph_builder.add_edge(third_edge);
        assert_matches!(expected,actual );
    }

    #[test]
    fn invalid_from_index() {
        let mut graph_builder = create_test_graph_builder();
        let invalid_edge = Edge { from_index: 10, to_index: 3, weight: 120 };

        let expected: aResult<()> =
            Err(anyhow!("add_edge has failed for edge number: 1 - from_index 10 is greater than 5 !"));
        let actual = graph_builder.add_edge(invalid_edge);
        assert_matches!(expected,actual );
    }

    #[test]
    fn invalid_to_index() {
        let mut graph_builder = create_test_graph_builder();
        let invalid_edge = Edge { from_index: 2, to_index: 7, weight: 120 };

        let expected: aResult<()> =
            Err(anyhow!("add_edge has failed for edge number: 1 - to_index 7 is greater than 5 !"));
        let actual = graph_builder.add_edge(invalid_edge);
        assert_matches!(expected,actual );
    }

    #[test]
    fn build_graph_too_few_edges() {
        let mut graph_builder = create_test_graph_builder();
        let first_edge = Edge { from_index: 1, to_index: 3, weight: 100 };
        graph_builder.add_edge(first_edge);
        let expected: aResult<()> = Err(anyhow!("current count of edges 1 is less than declared 2"));
        let actual = graph_builder.build();
        assert_matches!(expected,actual );
    }

    #[test]
    fn build_graph_ok() {
        let mut graph_builder = create_test_graph_builder();
        let first_edge = Edge { from_index: 1, to_index: 3, weight: 100 };
        let second_edge = Edge { from_index: 2, to_index: 3, weight: 130 };
        graph_builder.add_edge(first_edge);
        graph_builder.add_edge(second_edge);
        let expected = Graph {
            nodes_count: 3,
            edges: vec![first_edge, second_edge],
        };
        let actual = graph_builder.build().unwrap();
        assert_matches!(expected,actual );
    }
}
