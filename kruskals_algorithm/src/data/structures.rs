use crate::data::dfs::is_connected as dfs_is_connected;
use crate::{errors::CreatingEdgeError, BuildGraphError, Result};
use std::convert::TryFrom;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Edge {
    pub from_index: u32,
    pub to_index:   u32,
    pub weight:     i32,
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
    pub to_index:   &'a str,
    pub weight:     &'a str,
}

impl<'a> TryFrom<EdgeDescription<'a>> for Edge {
    type Error = BuildGraphError;

    fn try_from(edge_description: EdgeDescription<'a>) -> Result<Self, Self::Error> {
        let parsed_from_index = edge_description.from_index.parse::<u32>().map_err(|_| {
            let err: BuildGraphError =
                CreatingEdgeError::from_edge_description_with_bad_from_index(&edge_description).into();
            err
        })?;

        let parsed_to_index = edge_description.to_index.parse::<u32>().map_err(|_| {
            BuildGraphError::from(CreatingEdgeError::from_edge_description_with_bad_to_index(
                &edge_description,
            ))
        })?;

        let parsed_weight = edge_description.weight.parse::<i32>().map_err(|_| {
            BuildGraphError::from(CreatingEdgeError::from_edge_description_with_bad_weight(
                &edge_description,
            ))
        })?;

        Ok(Edge::new(parsed_from_index, parsed_to_index, parsed_weight))
    }
}

#[derive(Debug)]
pub struct Graph {
    pub nodes_count: u32,
    pub edges:       Vec<Edge>,
}

impl Graph {
    pub fn new(nodes_count: u32, edges: Vec<Edge>) -> Graph {
        Graph { nodes_count, edges }
    }
}

pub struct GraphBuilder {
    nodes_count:     u32,
    max_edges_count: usize,
    edges:           Vec<Edge>,
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

    pub fn add_edge(&mut self, edge: Edge) -> Result<()> {
        if self.edges.len() < self.max_edges_count {
            if edge.from_index > self.nodes_count {
                return Err(BuildGraphError::WrongFromIndex {
                    edge_number: self.edges.len() + 1,
                    from_index:  edge.from_index,
                    nodes_count: self.nodes_count,
                });
            }

            if edge.to_index > self.nodes_count {
                return Err(BuildGraphError::WrongToIndex {
                    edge_number: self.edges.len() + 1,
                    to_index:    edge.to_index,
                    nodes_count: self.nodes_count,
                });
            }

            self.edges.push(edge);
            Ok(())
        } else {
            Err(BuildGraphError::TooManyEdges {
                max_edges_count: self.max_edges_count,
                edge,
            })
        }
    }

    // checks if there is a path from any node to any other node
    fn is_connected(&self) -> bool {
        dfs_is_connected(&self.edges, self.nodes_count)
    }

    pub fn build(self) -> Result<Graph> {
        if self.edges.len() < self.max_edges_count {
            return Err(BuildGraphError::TooFewEdges {
                current_count: self.edges.len(),
                declared:      self.max_edges_count,
            });
        }

        if !self.is_connected() {
            return Err(BuildGraphError::GraphNotConnected);
        }

        Ok(Graph::new(self.nodes_count, self.edges))
    }
}

// -----------------------------------------------------------------------------

#[derive(Debug)]
pub struct GraphParameters {
    pub nodes_count:     u32,
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
    use super::*;
    use crate::data::Graph;
    use crate::{errors::CreatingEdgeError, BuildGraphError};
    use std::convert::TryFrom;

    #[test]
    fn create_edge_ok() {
        let edge_description = EdgeDescription {
            from_index: "1",
            to_index:   "5",
            weight:     "200",
        };
        let expected = Edge::new(1, 5, 200);
        let actual = Edge::try_from(edge_description).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn create_edge_with_err_because_from_index_field_in_edge_description_is_invalid() {
        let edge_description = EdgeDescription {
            from_index: "xxx",
            to_index:   "2",
            weight:     "130",
        };
        let expected = BuildGraphError::from(CreatingEdgeError::from_edge_description_with_bad_from_index(
            &edge_description,
        ));

        let actual = Edge::try_from(edge_description).unwrap_err();
        assert_eq!(actual.to_string(), expected.to_string());
    }

    #[test]
    fn create_edge_with_err_because_to_index_field_in_edge_description_is_invalid() {
        let edge_description = EdgeDescription {
            from_index: "1",
            to_index:   "xxx",
            weight:     "130",
        };
        let expected = BuildGraphError::from(CreatingEdgeError::from_edge_description_with_bad_to_index(
            &edge_description,
        ));

        let actual = Edge::try_from(edge_description).unwrap_err();
        assert_eq!(actual.to_string(), expected.to_string());
    }

    #[test]
    fn create_edge_with_err_because_weight_field_in_edge_description_is_invalid() {
        let edge_description = EdgeDescription {
            from_index: "1",
            to_index:   "2",
            weight:     "xxx",
        };
        let expected = BuildGraphError::from(CreatingEdgeError::from_edge_description_with_bad_weight(
            &edge_description,
        ));

        let actual = Edge::try_from(edge_description).unwrap_err();
        assert_eq!(actual.to_string(), expected.to_string());
    }

    const TEST_GRAPH_PARAMETERS: GraphParameters = GraphParameters {
        nodes_count:     3,
        max_edges_count: 2,
    };

    fn create_test_graph_builder() -> GraphBuilder {
        GraphBuilder::new(TEST_GRAPH_PARAMETERS)
    }

    #[test]
    fn too_many_edges() {
        let mut graph_builder = create_test_graph_builder();
        graph_builder
            .add_edge(Edge {
                from_index: 1,
                to_index:   3,
                weight:     200,
            })
            .unwrap();
        graph_builder
            .add_edge(Edge {
                from_index: 2,
                to_index:   1,
                weight:     50,
            })
            .unwrap();

        let third_edge = Edge {
            from_index: 3,
            to_index:   4,
            weight:     170,
        };
        let expected = BuildGraphError::TooManyEdges {
            max_edges_count: TEST_GRAPH_PARAMETERS.max_edges_count,
            edge:            third_edge,
        };

        let actual = graph_builder.add_edge(third_edge).unwrap_err();
        assert_eq!(actual.to_string(), expected.to_string());
    }

    #[test]
    fn invalid_from_index() {
        let mut graph_builder = create_test_graph_builder();
        let invalid_edge = Edge {
            from_index: 10,
            to_index:   3,
            weight:     120,
        };

        let expected = BuildGraphError::WrongFromIndex {
            edge_number: 1,
            from_index:  10,
            nodes_count: TEST_GRAPH_PARAMETERS.nodes_count,
        };
        let actual = graph_builder.add_edge(invalid_edge).unwrap_err();
        assert_eq!(actual.to_string(), expected.to_string());
    }

    #[test]
    fn invalid_to_index() {
        let mut graph_builder = create_test_graph_builder();
        let invalid_edge = Edge {
            from_index: 2,
            to_index:   7,
            weight:     120,
        };

        let expected = BuildGraphError::WrongToIndex {
            edge_number: 1,
            to_index:    7,
            nodes_count: 3,
        };

        let actual = graph_builder.add_edge(invalid_edge).unwrap_err();
        assert_eq!(actual.to_string(), expected.to_string());
    }

    #[test]
    fn build_graph_too_few_edges() {
        let mut graph_builder = create_test_graph_builder();
        let first_edge = Edge {
            from_index: 1,
            to_index:   3,
            weight:     100,
        };

        graph_builder.add_edge(first_edge).unwrap();
        let expected = BuildGraphError::TooFewEdges {
            current_count: graph_builder.edges.len(),
            declared:      graph_builder.max_edges_count,
        };
        let actual = graph_builder.build().unwrap_err();
        assert_eq!(actual.to_string(), expected.to_string());
    }

    #[test]
    fn build_graph_ok() {
        let mut graph_builder = create_test_graph_builder();
        let first_edge = Edge {
            from_index: 1,
            to_index:   3,
            weight:     100,
        };
        let second_edge = Edge {
            from_index: 2,
            to_index:   3,
            weight:     130,
        };
        graph_builder.add_edge(first_edge).unwrap();
        graph_builder.add_edge(second_edge).unwrap();
        let expected = Graph {
            nodes_count: 3,
            edges:       vec![first_edge, second_edge],
        };
        let actual = graph_builder.build().unwrap();
        assert_eq!(format!("{:?}", actual), format!("{:?}", expected));
    }
}
