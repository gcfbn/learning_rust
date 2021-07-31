use runner::*;

/// Main function that is called when the app starts
///
/// Calls run() function and kills process if it returns an error
fn main() {
    if let Err(err) = run() {
        write_app_error_message(&err.to_string());
        std::process::exit(1);
    }
}

fn write_app_error_message(error_message: &String) {
    use clap::{AppSettings, IntoApp};
    use utils::write_colored_error_message;

    if CmdArgs::into_app().is_set(AppSettings::ColoredHelp) {
        write_colored_error_message(error_message).unwrap();
    } else {
        println!("{}", error_message);
    }
}

/// Builds graph from given file and calculates weight of it's minimum spanning tree, returns [`RunnerResult`]
fn run() -> RunnerResult<()> {
    use clap::Clap;

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
