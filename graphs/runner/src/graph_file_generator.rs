use crate::GraphFileGenerator;
use anyhow::{bail, Result as aResult};
use rand::prelude::*;
use std::fs::File;
use std::io::Write;

pub fn generate_graph(parameters: &GraphFileGenerator) -> aResult<()> {
    // check if it's possible to generate connected graph with given parameters
    if parameters.edges_count + 1 < parameters.nodes_count {
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
