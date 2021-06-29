use crate::data::dfs::dfs;
use crate::{
    errors::{AddingEdgeError, GraphParametersParsingError, ParsingEdgeError},
    BuildGraphError,
    Result,
};
use std::convert::TryFrom;
use std::str::FromStr;

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

impl FromStr for Edge {
    type Err = BuildGraphError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let edge_description = EdgeDescription::try_from(s)?;
        Edge::try_from(edge_description)
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
            .ok_or_else(|| BuildGraphError::from(ParsingEdgeError::EmptyLine))?;
        let to_index = iter
            .next()
            .ok_or_else(|| BuildGraphError::from(ParsingEdgeError::MissingToIndexField))?;
        let weight = iter
            .next()
            .ok_or_else(|| BuildGraphError::from(ParsingEdgeError::MissingWeightField))?;

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
            BuildGraphError::from(ParsingEdgeError::FromIndexValueMustBeInteger(
                edge_description.from_index.to_owned(),
            ))
        })?;

        let parsed_to_index = edge_description.to_index.parse::<u32>().map_err(|_| {
            BuildGraphError::from(ParsingEdgeError::ToIndexValueMustBeInteger(
                edge_description.to_index.to_owned(),
            ))
        })?;

        let parsed_weight = edge_description.weight.parse::<i32>().map_err(|_| {
            BuildGraphError::from(ParsingEdgeError::WeightValueMustBeInteger(
                edge_description.weight.to_owned(),
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
            edges_count: max_edges_count,
        } = gp;

        GraphBuilder {
            nodes_count,
            max_edges_count,
            edges: Vec::with_capacity(max_edges_count),
        }
    }

    pub fn add_edge(&mut self, edge: Edge) -> Result<()> {
        if self.edges.len() >= self.max_edges_count {
            return Err(BuildGraphError::from(AddingEdgeError::TooManyEdges {
                edges_count: self.edges.len(),
                edge,
            }));
        }

        if edge.from_index > self.nodes_count {
            return Err(BuildGraphError::from(AddingEdgeError::WrongFromIndex {
                edge,
                nodes_count: self.nodes_count,
            }));
        }

        if edge.to_index > self.nodes_count {
            return Err(BuildGraphError::from(AddingEdgeError::WrongToIndex {
                edge,
                nodes_count: self.nodes_count,
            }));
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
    pub nodes_count: u32,
    pub edges_count: usize,
}

impl GraphParameters {
    pub fn new(nodes_count: u32, max_edges_count: usize) -> GraphParameters {
        GraphParameters {
            nodes_count,
            edges_count: max_edges_count,
        }
    }
}

impl TryFrom<&str> for GraphParameters {
    type Error = BuildGraphError;

    fn try_from(line: &str) -> Result<Self, Self::Error> {
        let mut inner_iter = line.split_whitespace();

        let nodes_count = inner_iter.next().unwrap(); // cannot fail !

        let nodes_count = nodes_count.parse::<u32>().map_err(|_| {
            BuildGraphError::from(GraphParametersParsingError::NodesCountValueMustBeInteger(
                nodes_count.to_owned(),
            ))
        })?;

        let edges_count = inner_iter
            .next()
            .ok_or_else(|| BuildGraphError::from(GraphParametersParsingError::MissingEdgesCountValue))?;

        let edges_count = edges_count.parse::<usize>().map_err(|_| {
            BuildGraphError::from(GraphParametersParsingError::EdgesCountValueIsNotInteger(
                edges_count.to_owned(),
            ))
        })?;

        Ok(GraphParameters::new(nodes_count, edges_count))
    }
}

#[cfg(test)]
#[macro_use]
mod tests {

    mod create_edge {
        use crate::data::EdgeDescription;
        use crate::errors::ParsingEdgeError;
        use crate::{BuildGraphError, Edge};
        use std::convert::TryFrom;
        use test_case::test_case;

        #[test]
        fn ok() {
            let edge_description = EdgeDescription::try_from("1 5 200").unwrap();
            let expected = Edge::new(1, 5, 200);
            let actual = Edge::try_from(edge_description).unwrap();
            assert_eq!(expected, actual);
        }

        #[test_case( "", ParsingEdgeError::EmptyLine; "empty line")]
        #[test_case( "1", ParsingEdgeError::MissingToIndexField; "missing to_index field" )]
        #[test_case( "1 2", ParsingEdgeError::MissingWeightField; "missing weight field" )]
        fn fails_because_of_invalid_edge_description(input: &str, expected_error: ParsingEdgeError) {
            let match_expected = match EdgeDescription::try_from(input).unwrap_err() {
                BuildGraphError::ParsingEdgeError(actual_err) if actual_err == expected_error => true,
                _ => false,
            };

            assert_eq!(match_expected, true);
        }

        #[test_case(
            "x 2 130",
            BuildGraphError::from(ParsingEdgeError::FromIndexValueMustBeInteger(String::from("x")));
            "from index is not an integer"
        )]
        #[test_case(
            "1 x 130",
            BuildGraphError::from(ParsingEdgeError::ToIndexValueMustBeInteger(String::from("x")));
            "to index is not an integer"
        )]
        #[test_case(
            "1 2 xxx",
            BuildGraphError::from(ParsingEdgeError::WeightValueMustBeInteger(String::from("xxx")));
            "weight is not an integer"
        )]
        fn fails_because_of_non_integer_value(line: &str, expected_error: BuildGraphError) {
            let edge_description = EdgeDescription::try_from(line).unwrap();

            let actual_error = Edge::try_from(edge_description).unwrap_err();
            assert_eq!(actual_error.to_string(), expected_error.to_string());
        }
    }

    // -----------------------------------------------------------------------------

    use crate::data::structures::{GraphBuilder, GraphParameters};

    const TEST_GRAPH_PARAMETERS: GraphParameters = GraphParameters {
        nodes_count: 3,
        edges_count: 2,
    };

    fn create_test_graph_builder() -> GraphBuilder {
        GraphBuilder::new(TEST_GRAPH_PARAMETERS)
    }

    // -----------------------------------------------------------------------------

    mod add_edge {
        use crate::data::structures::tests::{create_test_graph_builder, TEST_GRAPH_PARAMETERS};
        use crate::{AddingEdgeError, BuildGraphError};

        #[test]
        fn too_many_edges() {
            let mut graph_builder = create_test_graph_builder();
            graph_builder.add_edge("1 3 200".parse().unwrap()).unwrap();
            graph_builder.add_edge("2 1 50".parse().unwrap()).unwrap();

            let third_edge = "3 4 170".parse().unwrap();

            let expected = BuildGraphError::from(AddingEdgeError::TooManyEdges {
                edges_count: TEST_GRAPH_PARAMETERS.edges_count,
                edge:        third_edge,
            });

            let actual = graph_builder.add_edge(third_edge).unwrap_err();
            assert_eq!(actual.to_string(), expected.to_string());
        }

        #[test]
        fn invalid_from_index() {
            let mut graph_builder = create_test_graph_builder();
            let invalid_edge = "10 3 120".parse().unwrap();

            let expected = BuildGraphError::from(AddingEdgeError::WrongFromIndex {
                edge:        invalid_edge,
                nodes_count: TEST_GRAPH_PARAMETERS.nodes_count,
            });

            let actual = graph_builder.add_edge(invalid_edge).unwrap_err();
            assert_eq!(actual.to_string(), expected.to_string());
        }

        #[test]
        fn invalid_to_index() {
            let mut graph_builder = create_test_graph_builder();
            let invalid_edge = "2 7 120".parse().unwrap();

            let expected = BuildGraphError::from(AddingEdgeError::WrongToIndex {
                edge:        invalid_edge,
                nodes_count: TEST_GRAPH_PARAMETERS.nodes_count,
            });

            let actual = graph_builder.add_edge(invalid_edge).unwrap_err();
            eprintln!("{}", actual);
            assert_eq!(actual.to_string(), expected.to_string());
        }
    }

    // -----------------------------------------------------------------------------

    mod build_graph {
        use crate::data::structures::tests::create_test_graph_builder;
        use crate::data::Graph;
        use crate::BuildGraphError;

        #[test]
        fn ok() {
            let mut graph_builder = create_test_graph_builder();
            let first_edge = "1 3 100".parse().unwrap();
            let second_edge = "2 3 130".parse().unwrap();

            graph_builder.add_edge("1 3 100".parse().unwrap()).unwrap();
            graph_builder.add_edge("2 3 130".parse().unwrap()).unwrap();
            let expected = Graph {
                nodes_count: 3,
                edges:       vec![first_edge, second_edge],
            };
            let actual = graph_builder.build().unwrap();
            assert_eq!(format!("{:?}", actual), format!("{:?}", expected));
        }

        #[test]
        fn error_too_few_edges() {
            let mut graph_builder = create_test_graph_builder();

            graph_builder.add_edge("1 3 100".parse().unwrap()).unwrap();
            let expected = BuildGraphError::TooFewEdges {
                current_count: graph_builder.edges.len(),
                declared:      graph_builder.max_edges_count,
            };
            let actual = graph_builder.build().unwrap_err();
            assert_eq!(actual.to_string(), expected.to_string());
        }
    }
}
