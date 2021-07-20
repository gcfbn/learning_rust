use crate::errors::{GenerateGraphError, Result};
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
/// use runner_lib::{GenerateGraphFileArgs, generate_graph};
/// use std::path::PathBuf;
///
/// let parameters = GenerateGraphFileArgs::try_from_args( "--graph-file test_graph.file.txt --nodes-count 3 --edges-count 5 --max-weight 20").unwrap();
/// let output = generate_graph(&parameters);
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
    if !is_possible_to_create_connected_graph(parameters) {
        return Err(GenerateGraphError::TooFewEdgesForConnectedGraph {
            edges_count: parameters.edges_count,
            nodes_count: parameters.nodes_count,
        });
    }

    create_directory_if_necessary(&parameters.graph_file).map_err(GenerateGraphError::CreatingDirectoryError)?;

    let mut output = File::create(&parameters.graph_file).map_err(GenerateGraphError::CreatingFileError)?;

    // write `nodes_count` and `edges_count` to the first line of file
    output
        .write_all(format!("{} {}\n", parameters.nodes_count, parameters.edges_count).as_ref())
        .map_err(GenerateGraphError::WritingError)?;

    let mut rng = thread_rng();

    // add edges connecting first node with every other node so the graph will be connected
    for i in 2..=parameters.nodes_count {
        output
            .write_all(format!("1 {} {}\n", i, rng.gen_range(1..=parameters.max_weight)).as_ref())
            .map_err(GenerateGraphError::WritingError)?;
    }

    // edges left after connecting first node with other nodes
    let edges_left = calculate_edges_left(parameters.nodes_count, parameters.edges_count);

    // generate rest of edges using `rng`
    for _ in 0..=edges_left {
        output
            .write_all(
                format!(
                    "{} {} {}\n",
                    rng.gen_range(1..=parameters.nodes_count),
                    rng.gen_range(1..=parameters.nodes_count),
                    rng.gen_range(1..=parameters.max_weight)
                )
                .as_ref(),
            )
            .map_err(GenerateGraphError::WritingError)?;
    }
    Ok(())
}

/// Check if it's possible to generate connected graph with given parameters
///
/// # Arguments
///
/// * `parameters` - parameters of the graph
fn is_possible_to_create_connected_graph(parameters: &GenerateGraphFileArgs) -> bool {
    parameters.edges_count + 1 >= parameters.nodes_count
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
fn calculate_edges_left(nodes_count: u32, edges_count: u32) -> u32 {
    edges_count - (nodes_count - 1)
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(98 => false; "too small edges_count")]
    #[test_case(99 => true; "ok")]
    fn is_possible_to_create_connected_graph(edges_count: u32) -> bool {
        let args = format!(
            "--graph-file test_file.txt --nodes-count 100 --edges-count {} --max-weight 100",
            edges_count
        );
        let parameters = GenerateGraphFileArgs::try_from_args(&args).unwrap();

        crate::generate_graph::is_possible_to_create_connected_graph(&parameters)
    }

    #[test]
    fn generating_graph_fails_because_number_of_edges_was_too_small() {
        let parameters = GenerateGraphFileArgs::try_from_args(
            "--graph-file test_file.txt --nodes-count 30 --edges-count 28 --max-weight 100",
        )
        .unwrap();

        let expected_error = GenerateGraphError::TooFewEdgesForConnectedGraph {
            edges_count: 28,
            nodes_count: 30,
        };
        let actual_error = generate_graph(&parameters).unwrap_err();

        assert_eq!(actual_error.to_string(), expected_error.to_string());
    }
}
