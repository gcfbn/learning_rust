use anyhow::{anyhow, Result as aResult};
use clap::{AppSettings, Clap};
use graph_file_generator::generate_graph;
use std::path::{Path, PathBuf};
use std::process;

mod graph_file_generator;

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
    #[clap(visible_alias = "gfg")]
    GraphFileGenerator(GraphFileGenerator),
    #[clap(visible_alias = "t")]
    Task(Task),
}

/// Subcommand generating random graph file, which could be used in algorithms
#[derive(Clap, Debug)]
#[clap(setting = AppSettings::ColoredHelp)]
pub struct GraphFileGenerator {
    /// Output filename
    #[clap(long, short, validator(is_txt))]
    graph_file: PathBuf,

    /// Number of nodes in graph (indexed from 1 to `nodes_count`, so must be positive)
    #[clap(long, short, validator(nodes_count_valid))]
    nodes_count: u32,

    /// Number of edges in graph
    #[clap(long, short)]
    edges_count: u32,

    /// Maximum weight of an edge in graph (must be an positive integer)
    #[clap(long, short, validator(max_weight_valid))]
    max_weight: u32,
}

/// Subcommand running Kruskal's algorithm for graph built from `task_file`
#[derive(Clap, Debug)]
#[clap(setting = AppSettings::ColoredHelp)]
struct Task {
    /// Name of file containing graph data
    #[clap(long, short, parse(from_os_str), validator(file_exists))]
    task_file: PathBuf,
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

/// Checks if file is a txt file
///
/// # Arguments
///
/// `p` - Path to file including it's name and format
fn is_txt(p: &str) -> aResult<()> {
    if Path::new(p)
        .extension()
        .ok_or_else(|| anyhow!("missing final '.' in filename: {}", p))?
        == "txt"
    {
        Ok(())
    } else {
        Err(anyhow!("the file isn't a txt file: {}", p))
    }
}

/// Checks if given number of nodes is correct (has to be a positive integer)
///
/// # Arguments
///
/// `nodes_count` - Number of nodes given by user
fn nodes_count_valid(nodes_count: &str) -> aResult<()> {
    let nodes_count = nodes_count.parse::<u32>()?;
    if nodes_count > 0 {
        Ok(())
    } else {
        Err(anyhow!(
            "given number of nodes has to be a positive integer, but is: {}",
            nodes_count
        ))
    }
}

/// Checks if given edge weight is correct (has to be a positive integer)
///
/// # Arguments
///
/// `max_weight` - Max weight given by user
fn max_weight_valid(max_weight: &str) -> aResult<()> {
    let max_weight = max_weight.parse::<u32>()?;
    if max_weight > 0 {
        Ok(())
    } else {
        Err(anyhow!(
            "given max edge weight has to be a positive integer, but is: {}",
            max_weight
        ))
    }
}

/// Builds graph from given file and calculates weight of it's minimum spanning tree, returns [`anyhow::Result`]
fn run() -> aResult<()> {
    let cmd_args: CmdArgs = CmdArgs::parse();

    match cmd_args.subcommand {
        SubCommand::Task(task_data) => {
            let graph = graph::build_graph(&task_data.task_file)?;
            let output = algorithms::calculate_min_total_weight(graph);
            println!("{}", output);
        }

        SubCommand::GraphFileGenerator(params) => {
            generate_graph(&params)?;
            println!("Graph file with path {:?} successfully generated!", params.graph_file);
        }
    }
    Ok(())
}
