use clap::{AppSettings, Clap};
use graph::Result;
use std::path::{PathBuf};
use std::process;

/// Main function that is called when the app starts
///
/// Calls run() function and kills process if it returns an error
fn main() {
    if let Err(err) = run() {
        println!("Error: {:?}", err);
        process::exit(1);
    }
}

/// Arguments read from console by Clap
#[derive(Debug, Clap)]
#[clap(
name = "kruskal_algorithm",
version = "1.0",
about = "Algorithms & Data structures task from graph theory",
author = "Bartek M. <bmekarski@interia.pl>",
setting = AppSettings::ColoredHelp,
setting = AppSettings::ArgRequiredElseHelp,
)]
struct CmdArgs {
    #[clap(subcommand)]
    pub subcommand: SubCommand,
}

#[derive(Clap, Debug)]
enum SubCommand {
    #[clap()]
    GraphFileGenerator(GraphFileGeneratorParameters),
    #[clap()]
    Task(TaskData),
}

#[derive(Clap, Debug)]
struct GraphFileGeneratorParameters {
    output: PathBuf,
    nodes_count: u32,
    edges_count: u32,
    max_weight: u32,
}

#[derive(Clap, Debug)]
struct TaskData {
    task_file: PathBuf,
}

/// Builds graph from given file and calculates weight of it's minimum spanning tree, returns [`anyhow::Result`]
fn run() -> Result<()> {
    let cmd_args: CmdArgs = CmdArgs::parse();

    match cmd_args.subcommand {
        SubCommand::Task(task_data) => {
            let graph = graph::build_graph(&task_data.task_file)?;
            let output = algorithms::calculate_min_total_weight(graph);
            println!("{}", output);
        }

        SubCommand::GraphFileGenerator(params) => generate_graph(params),
    }
    Ok(())
}

fn generate_graph(parameters: GraphFileGeneratorParameters) {
    println!("RANDOM_GRAPH");
}

