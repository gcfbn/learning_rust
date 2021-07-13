use anyhow::{anyhow, Context, Result as aResult};
use clap::{AppSettings, Clap};
use graph_file_generator::generate_graph;
use std::path::{Path, PathBuf};
use std::process;

mod graph_file_generator;

const APP_NAME: &str = "kruskal_algorithm";

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
name = APP_NAME,
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

impl SubCommand {
    pub fn try_from_name_and_args(command_name: &str, args: &str) -> aResult<Self> {
        let cli_string = format!(
            "{app} {command} {args}",
            app = APP_NAME,
            command = command_name,
            args = args
        );

        let cmd_args = CmdArgs::try_parse_from(cli_string.split_whitespace()).with_context(|| {
            format!(
                "failed creating command with command_name='{}' and args={}",
                command_name, args
            )
        })?;

        Ok(cmd_args.subcommand)
    }
}

/// Subcommand generating random graph file, which could be used in algorithms
#[derive(Clap, Debug)]
#[clap(setting = AppSettings::ColoredHelp)]
pub struct GraphFileGenerator {
    /// Output filename
    #[clap(long, short)]
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

impl GraphFileGenerator {
    /// Constructor
    pub fn new(graph_file: PathBuf, nodes_count: u32, edges_count: u32, max_weight: u32) -> GraphFileGenerator {
        GraphFileGenerator {
            graph_file,
            nodes_count,
            edges_count,
            max_weight,
        }
    }

    #[cfg(test)]
    pub fn try_from_args(args: &str) -> aResult<Self> {
        match SubCommand::try_from_name_and_args("graph-file-generator", args)? {
            SubCommand::GraphFileGenerator(cmd) => Ok(cmd),
            _ => panic!("this should never happen !"),
        }
    }
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

/// Checks if given number of nodes is correct (has to be a positive integer)
///
/// # Arguments
///
/// `nodes_count` - Number of nodes given by user
fn nodes_count_valid(nodes_count: &str) -> aResult<()> {
    let nodes_count = nodes_count
        .parse::<u32>()
        .with_context(|| format!("'{}' has to be a not negative integer", nodes_count))?;
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
    let max_weight = max_weight
        .parse::<u32>()
        .with_context(|| format!("'{}' has to be a not negative integer", max_weight))?;
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

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    mod subcommand_graph_file_generator {
        use super::*;

        use anyhow::Result;

        mod failing_tests {
            use super::*;

            #[test]
            fn fails_because_edges_count_is_to_small() -> Result<()> {
                let result = GraphFileGenerator::try_from_args(
                    "--graph-file aaa.txt --nodes-count 5 --edges-count 3 --max-weight 100",
                );

                assert!(result.is_err());
                Ok(())
            }

            #[test]
            fn fails_because_edges_count_is_not_integer() -> Result<()> {
                let result = GraphFileGenerator::try_from_args(
                    "--graph-file aaa.txt --nodes-count 5 --edges-count 3a --max-weight 100",
                );

                assert!(result.is_err());
                Ok(())
            }
        }
    }

    #[test]
    fn nodes_count_ok() {
        let nodes_count = "200";
        let actual = nodes_count_valid(nodes_count).unwrap();
        assert_eq!(actual, ());
    }

    #[test_case("200a", "'200a' has to be a not negative integer"; "not_a_number")]
    #[test_case("0", "given number of nodes has to be a positive integer, but is: 0"; "zero")]
    #[test_case("-123", "'-123' has to be a not negative integer"; "negative")]
    #[test_case("3,5", "'3,5' has to be a not negative integer"; "not an integer")]
    fn nodes_count_invalid(nodes_count: &str, expected: &str) {
        let actual = nodes_count_valid(nodes_count).unwrap_err().to_string();
        assert_eq!(actual, expected);
    }

    #[test]
    fn max_weight_ok() {
        let max_weight = "300";
        let actual = nodes_count_valid(max_weight).unwrap();
        assert_eq!(actual, ());
    }

    #[test_case("100a0", "'100a0' has to be a not negative integer"; "not_a_number")]
    #[test_case("0", "given max edge weight has to be a positive integer, but is: 0"; "zero")]
    #[test_case("-3", "'-3' has to be a not negative integer"; "negative")]
    #[test_case("7,25", "'7,25' has to be a not negative integer"; "not an integer")]
    fn max_weight_invalid(max_weight: &str, expected: &str) {
        let actual = max_weight_valid(max_weight).unwrap_err().to_string();
        assert_eq!(actual, expected);
    }
}
