use clap::Clap;
use runner::*;
use std::process;
use utils::write_colored_error_message;

type Result<T, E = RunnerError> = std::result::Result<T, E>;

/// Main function that is called when the app starts
///
/// Calls run() function and kills process if it returns an error
fn main() {
    if let Err(err) = run() {
        write_colored_error_message(&err.to_string()).unwrap();
        process::exit(1);
    }
}

/// Builds graph from given file and calculates weight of it's minimum spanning tree, returns [`anyhow::Result`]
fn run() -> Result<()> {
    let cmd_args: CmdArgs = CmdArgs::parse();

    match cmd_args.subcommand {
        SubCommand::RunAlgorithm(task_data) => {
            let graph = graph::build_graph(&task_data.task_file)?;
            let output = algorithms::calculate_min_total_weight(graph);
            println!("{}", output);
        }

        SubCommand::GenerateGraphFile(params) => {
            generate_graph(&params)?;
            println!("Graph file with path {:?} successfully generated!", params.graph_file);
        }
    }
    Ok(())
}
