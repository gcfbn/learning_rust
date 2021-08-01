use crate::errors::{GenerateGraphError, GenerateGraphResult as Result};
use crate::GenerateGraphFileArgs;
use rand::prelude::*;
use std::fs::{create_dir_all, File};
use std::io::{Result as ioResult, Write};
use std::path::Path;

/// Generates txt file containing multi-graph data using `parameters`
///
/// Graph file is generated using [`rand`], so two nodes might be connected by more than one edge
///
/// Most of the graph theory algorithms require the graph to be connected,
/// so function checks if it's possible to generate connected graph from given parameters.
/// If it's not, function returns an error.
///
/// First line in the file contains `nodes_count` and `edges_count`
/// Then, file contains `nodes_count - 1` lines describing edges
/// connecting first node with every other node, so graph is for sure connected
/// After that, the rest of the edges is generated randomly.
///
/// # Example
/// ```
/// use runner::{GenerateGraphFileArgs, generate_graph};
/// use tempfile::NamedTempFile;
/// use graph::build_graph;
///
/// # let output_graph_file = NamedTempFile::new().unwrap();
///
/// // `output_graph_file` is temporary graph file, deleted after running the test
/// let parameters = format!("--graph-file {} --nodes-count 3 --edges-count 5 --max-weight 20",
///     output_graph_file.path().to_str().unwrap())
///     .parse::<GenerateGraphFileArgs>().unwrap();
///
/// let result = generate_graph(&parameters);
///
/// assert!(result.is_ok());
/// assert!(output_graph_file.path().exists());
///
/// // check if generated graph is correct
/// let build_result = build_graph(output_graph_file.path());
/// assert!(build_result.is_ok());
/// ```
///
/// # Possible output
/// ```text
/// test_graph_file.txt:
/// 3 5
/// 1 2 13
/// 1 3 1
/// 2 3 1
/// 1 1 11
/// 1 3 2
/// ```
///
/// # Arguments
///
/// * `parameters` - parameters of the graph
pub fn generate_graph(parameters: &GenerateGraphFileArgs) -> Result<()> {
    // how many edges will left after connecting first node with all other nodes - if not enough an error is returned
    // beause we don't want to have a graph with not connected nodes
    let edges_left = try_calculate_edges_left(parameters.nodes_count.value(), parameters.edges_count.value())?;

    create_directory_if_necessary(&parameters.graph_file).map_err(GenerateGraphError::CreatingDirectoryError)?;

    let mut output = File::create(&parameters.graph_file).map_err(GenerateGraphError::CreatingFileError)?;

    // write `nodes_count` and `edges_count` to the first line of file
    output
        .write_all(format!("{} {}\n", parameters.nodes_count, parameters.edges_count).as_ref())
        .map_err(GenerateGraphError::WritingError)?;

    let mut rng = thread_rng();

    // add edges connecting first node with every other node so the graph will be connected
    for i in 2..=parameters.nodes_count.value() {
        output
            .write_all(format!("1 {} {}\n", i, rng.gen_range(1..=parameters.max_weight.value())).as_ref())
            .map_err(GenerateGraphError::WritingError)?;
    }

    // generate rest of edges using `rng`
    for _ in 0..edges_left {
        output
            .write_all(
                format!(
                    "{} {} {}\n",
                    rng.gen_range(1..=parameters.nodes_count.value()),
                    rng.gen_range(1..=parameters.nodes_count.value()),
                    rng.gen_range(1..=parameters.max_weight.value())
                )
                .as_ref(),
            )
            .map_err(GenerateGraphError::WritingError)?;
    }
    Ok(())
}

/// Takes a reference to a filepath and creates directory specified in the path if it doesn't exist
///
/// Clones `path`, because `pop()` removes filename from the path
///
/// # Arguments
///
/// * `path` - path to file
fn create_directory_if_necessary(path: &Path) -> ioResult<()> {
    let mut path = path.to_path_buf();
    path.pop();
    create_dir_all(path)
}

/// Calculates, how many edges left after connecting first graph node with other nodes
///
/// # Arguments
///
/// * `nodes_count` - number of nodes in the graph
/// * `edges_count` - total number of edges in the graph
fn try_calculate_edges_left(nodes_count: u32, edges_count: u32) -> Result<u32> {
    (1 + edges_count)
        .checked_sub(nodes_count)
        .ok_or(GenerateGraphError::TooFewEdgesForConnectedGraph {
            edges_count,
            nodes_count,
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fails_with_error_too_few_edges_for_connected_graph() {
        // nodes_count - edges_count > 1 - triggers an error
        let nodes_count = 5;
        let edges_count = 3;

        let args_str = format!(
            "--graph-file aaa.txt --nodes-count {} --edges-count {} --max-weight 100",
            nodes_count, edges_count
        );
        let parameters = args_str.parse::<GenerateGraphFileArgs>().unwrap();

        let actual_error = generate_graph(&parameters).unwrap_err();
        let expected_error = GenerateGraphError::TooFewEdgesForConnectedGraph {
            edges_count,
            nodes_count,
        };

        assert_eq!(actual_error.to_string(), expected_error.to_string());
    }
}
