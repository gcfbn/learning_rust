use runner::*;
use utils::ApplicationRunner;

/// Main function that is called when the app starts
///
/// Calls run() function and kills process if it returns an error
fn main() {
    App.main();
}

struct App;

impl ApplicationRunner for App {
    type CmdArgs = CmdArgs;
    type Error = RunnerError;

    /// Builds graph from given file and calculates weight of it's minimum spanning tree, returns [`RunnerResult`]
    fn run(&self, cmd_args: CmdArgs) -> RunnerResult<()> {
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
}
