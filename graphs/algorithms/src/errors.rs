use thiserror::Error;

/// Type returned by functions in this crate
pub type AlgorithmResult<T, E = AlgorithmError> = std::result::Result<T, E>;

/// Enum containing possible errors returned by algorithms
///
/// # Example
/// ```
/// use graph::Graph;
/// use algorithms::{find_shortest_path_length, AlgorithmError, DijkstrasError};
/// use utils::PositiveInteger;
///
/// let graph: Graph = "3 2
/// 1 2 100
/// 1 3 200".parse().unwrap();
///
/// let actual_error = find_shortest_path_length(&graph, PositiveInteger::new(1), PositiveInteger::new(4)).unwrap_err();
/// let expected_error = AlgorithmError::from(DijkstrasError::InvalidEndNode{
/// end_node: 4,
/// nodes_count: 3,
/// });
///
/// assert_eq!(actual_error.to_string(), expected_error.to_string());
/// ```
#[derive(Error, Debug)]
pub enum AlgorithmError {
    /// Error returned by Dijkstra's algorithm
    #[error("Dijkstra's algorithm error - {0}")]
    DijkstrasError(DijkstrasError),
}

impl From<DijkstrasError> for AlgorithmError {
    fn from(e: DijkstrasError) -> Self {
        Self::DijkstrasError(e)
    }
}

/// Errors returned by Dijkstra's algorithm
#[derive(Error, Debug)]
pub enum DijkstrasError {
    /// There is no node with given start index in the graph
    #[error("start_node `{start_node}` is greater than nodes_count `{nodes_count}`")]
    InvalidStartNode {
        /// Given index of start node
        start_node: u32,

        /// Number of nodes in the graph
        nodes_count: u32,
    },

    /// There is no node with given end index in the graph
    #[error("end_node `{end_node}` is greater than nodes_count `{nodes_count}`")]
    InvalidEndNode {
        /// Given index of end node
        end_node: u32,

        /// Number of nodes in the graph
        nodes_count: u32,
    },
}
