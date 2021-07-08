use anyhow::{anyhow, Result as aResult};
use clap::{AppSettings, Clap};
use graph::Result;
use std::path::{Path, PathBuf};
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
    setting=AppSettings::ColoredHelp,
)]
struct CmdArgs {
    /// Task file with task data
    #[clap(long, short, parse(from_os_str), validator(file_exists))]
    pub task_file: PathBuf,
}

/// Checks if file exists
///
/// # Arguments
///
/// `p` - Path to file including it's name and format
fn file_exists(p: &str) -> aResult<()> {
    if Path::new(p).exists() {
        Ok(())
    } else {
        Err(anyhow!("the file does not exist: {}", p))
    }
}

/// Builds graph from given file and calculates weight of it's minimum spanning tree, returns [`anyhow::Result`]
fn run() -> Result<()> {
    let cmd_args: CmdArgs = CmdArgs::parse();

    let graph = graph::build_graph_from_file(cmd_args.task_file)?;
    let output = algorithms::calculate_min_total_weight(graph);
    println!("{}", output);

    Ok(())
}
