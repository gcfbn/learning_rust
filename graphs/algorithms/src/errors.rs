use thiserror::Error;

/// Type returned by functions in this crate
pub type AlgorithmResult<T, E = AlgorithmError> = std::result::Result<T, E>;

/// Enum containing possible errors returned by algorithms
#[derive(Error, Debug)]
pub enum AlgorithmError {
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
    #[error("start_node `{start_node}` is greater than nodes_count `{nodes_count}`")]
    InvalidStartNode { start_node: u32, nodes_count: u32 },

    #[error("end_node `{end_node}` is greater than nodes_count `{nodes_count}`")]
    InvalidEndNode { end_node: u32, nodes_count: u32 },
}
