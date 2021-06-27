use crate::data::dfs::dfs;
use crate::{
    errors::{CreatingEdgeError, EdgeDescriptionError},
    BuildGraphError,
    Result,
};
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

impl<'a> TryFrom<&'a str> for EdgeDescription<'a> {
    type Error = BuildGraphError;

    fn try_from(s: &'a str) -> Result<Self, Self::Error> {
        let mut iter = s.split_whitespace();

        let from_index = iter
            .next()
            .ok_or_else(|| BuildGraphError::from(EdgeDescriptionError::EmptyInput))?;
        let to_index = iter
            .next()
            .ok_or_else(|| BuildGraphError::from(EdgeDescriptionError::MissingToIndexField))?;
        let weight = iter
            .next()
            .ok_or_else(|| BuildGraphError::from(EdgeDescriptionError::MissingWeightField))?;

        Ok(EdgeDescription {
            from_index,
            to_index,
            weight,
        })
    }
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
        if self.edges.len() >= self.max_edges_count {
            return Err(BuildGraphError::TooManyEdges {
                max_edges_count: self.max_edges_count,
                edge,
            });
        }

        if edge.from_index > self.nodes_count {
            return Err(BuildGraphError::from(EdgeDescriptionError::WrongFromIndex(
                edge,
                self.nodes_count,
            )));
        }

        if edge.to_index > self.nodes_count {
            return Err(BuildGraphError::from(EdgeDescriptionError::WrongToIndex(
                edge,
                self.nodes_count,
            )));
        }

        self.edges.push(edge);
        Ok(())
    }

    // checks if there is a path from any node to any other node
    fn is_connected(&self) -> bool {
        let mut adjacency_list: Vec<Vec<usize>> = vec![Vec::new(); (self.nodes_count + 1) as usize];

        for edge in &self.edges {
            adjacency_list[edge.from_index as usize].push(edge.to_index as usize);
            adjacency_list[edge.to_index as usize].push(edge.from_index as usize);
        }

        let mut visited: Vec<bool> = vec![false; (self.nodes_count + 1) as usize];
        dfs(1, &adjacency_list, &mut visited);

        for value in visited.iter().skip(1) {
            if !value {
                return false;
            }
        }
        true
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
    use crate::test_case::test_case;
    use crate::{errors::CreatingEdgeError, BuildGraphError};
    use std::convert::TryFrom;

    #[test]
    fn create_edge_ok() {
        let edge_description = EdgeDescription::try_from("1 5 200").unwrap();
        let expected = Edge::new(1, 5, 200);
        let actual = Edge::try_from(edge_description).unwrap();
        assert_eq!(expected, actual);
    }

    #[test_case( "", EdgeDescriptionError::EmptyInput; "empty input")]
    #[test_case( "1", EdgeDescriptionError::MissingToIndexField; "missing to_index field" )]
    #[test_case( "1 2", EdgeDescriptionError::MissingWeightField; "missing weight field" )]
    fn create_edge_fails_because_of_invalid_edge_description(input: &str, expected_error: EdgeDescriptionError) {
        let match_expected = match EdgeDescription::try_from(input).unwrap_err() {
            BuildGraphError::InvalidEdgeDescription(actual_err) if actual_err == expected_error => true,
            _ => false,
        };

        assert_eq!(match_expected, true);
    }

    #[test]
    fn create_edge_fails_because_from_index_field_in_edge_description_is_invalid() {
        let edge_description = EdgeDescription::try_from("x 2 130").unwrap();
        let expected = BuildGraphError::from(CreatingEdgeError::from_edge_description_with_bad_from_index(
            &edge_description,
        ));

        let actual = Edge::try_from(edge_description).unwrap_err();
        assert_eq!(actual.to_string(), expected.to_string());
    }

    #[test]
    fn create_edge_fails_because_to_index_field_in_edge_description_is_invalid() {
        let edge_description = EdgeDescription::try_from("1 x 130").unwrap();
        let expected = BuildGraphError::from(CreatingEdgeError::from_edge_description_with_bad_to_index(
            &edge_description,
        ));

        let actual = Edge::try_from(edge_description).unwrap_err();
        assert_eq!(actual.to_string(), expected.to_string());
    }

    #[test]
    fn create_edge_fails_because_weight_field_in_edge_description_is_invalid() {
        let edge_description = EdgeDescription::try_from("1 2 xxx").unwrap();
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

        let expected = BuildGraphError::from(EdgeDescriptionError::WrongFromIndex(
            invalid_edge,
            TEST_GRAPH_PARAMETERS.nodes_count,
        ));

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

        let expected = BuildGraphError::from(EdgeDescriptionError::WrongToIndex(
            invalid_edge,
            TEST_GRAPH_PARAMETERS.nodes_count,
        ));

        let actual = graph_builder.add_edge(invalid_edge).unwrap_err();
        eprintln!("{}", actual);
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
