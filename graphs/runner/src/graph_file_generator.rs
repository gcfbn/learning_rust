use crate::GraphFileGenerator;
use anyhow::{bail, Result as aResult};
use rand::prelude::*;
use std::fs::File;
use std::io::Write;

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
/// let parameters = GraphFileGenerator {
///     test_graph_file.txt,
///     3,
///     5,
///     20,
/// }
///
///  generate_graph(&parameters)
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

pub fn generate_graph(parameters: &GraphFileGenerator) -> aResult<()> {
    if impossible_to_generate_connected_graph(parameters) {
        bail!(
            "`edges_count` must be at least {}, because `nodes_count` is {}, otherwise graph won't be connected",
            parameters.nodes_count - 1,
            parameters.nodes_count
        );
    }

    let mut output = File::create(&parameters.graph_file)?;

    // write `nodes_count` and `edges_count` to the first line of file
    output.write_all(format!("{} {}\n", parameters.nodes_count, parameters.edges_count).as_ref())?;

    let mut rng = thread_rng();

    // add edges connecting first node with every other node so the graph will be connected
    for i in 2..=parameters.nodes_count {
        output.write_all(format!("1 {} {}\n", i, rng.gen_range(1..=parameters.max_weight)).as_ref())?;
    }

    // calculate how many edges left
    let edges_left = parameters.edges_count - parameters.nodes_count;

    // generate rest of edges using `rng`
    for _ in 0..=edges_left {
        output.write_all(
            format!(
                "{} {} {}\n",
                rng.gen_range(1..=parameters.nodes_count),
                rng.gen_range(1..=parameters.nodes_count),
                rng.gen_range(1..=parameters.max_weight)
            )
            .as_ref(),
        )?;
    }
    Ok(())
}

/// Check if it's possible to generate connected graph with given parameters
///
/// # Arguments
///
/// * `parameters` - parameters used for generating graph file
fn impossible_to_generate_connected_graph(parameters: &GraphFileGenerator) -> bool {
    parameters.edges_count + 1 < parameters.nodes_count
}
