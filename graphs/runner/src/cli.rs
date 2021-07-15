#![warn(missing_docs)]
use crate::errors::RunnerError;
use anyhow::{anyhow, Context, Result as aResult};
use clap::{AppSettings, Clap};
use std::path::{Path, PathBuf};

const APP_NAME: &str = "kruskal_algorithm";

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
pub struct CmdArgs {
    /// Program subcommand
    #[clap(subcommand)]
    pub subcommand: SubCommand,
}

/// All available subcommands
#[derive(Clap, Debug)]
pub enum SubCommand {
    /// Generates file containing random graph data
    #[clap(visible_alias = "gfg")]
    GraphFileGenerator(GraphFileGenerator),
    /// Runs algorithm using data from chosen file
    #[clap(visible_alias = "t")]
    Task(Task),
}

impl SubCommand {
    /// Tries to build [`SubCommand`] variant from command line arguments
    ///
    /// Returns [`CommandLineSubcommandCreatingError`] on fail
    ///
    /// # Arguments
    ///
    /// * `command_name` - name of command used in command line
    /// * `args` - arguments passed with command
    pub fn try_from_name_and_args(command_name: &str, args: &str) -> Result<Self, RunnerError> {
        let cli_string = format!(
            "{app} {command} {args}",
            app = APP_NAME,
            command = command_name,
            args = args
        );

        let cmd_args = CmdArgs::try_parse_from(cli_string.split_whitespace()).map_err({
            |_| RunnerError::SubcommandCreatingError {
                command_name: String::from(command_name),
                args:         String::from(args),
            }
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
    pub graph_file: PathBuf,

    /// Number of nodes in graph (indexed from 1 to `nodes_count`, so must be positive)
    #[clap(long, short, validator(nodes_count_valid))]
    pub nodes_count: u32,

    /// Number of edges in graph
    #[clap(long, short)]
    pub edges_count: u32,

    /// Maximum weight of an edge in graph (must be an positive integer)
    #[clap(long, short, validator(max_weight_valid))]
    pub max_weight: u32,
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

    pub fn try_from_args(args: &str) -> aResult<Self> {
        match SubCommand::try_from_name_and_args("graph-file-generator", args)? {
            SubCommand::GraphFileGenerator(cmd) => Ok(cmd),
            // temporary solution
            _ => Err(anyhow!(
                "Invalid arguments: `{}` for command `{}`",
                args,
                "graph_file_generator"
            )),
        }
    }
}

/// Subcommand running Kruskal's algorithm for graph built from `task_file`
#[derive(Clap, Debug)]
#[clap(setting = AppSettings::ColoredHelp)]
pub struct Task {
    /// Name of file containing graph data
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

/// Checks if given number of nodes is correct (has to be a positive integer)
///
/// # Arguments
///
/// `nodes_count` - Number of nodes given by user
pub fn nodes_count_valid(nodes_count: &str) -> aResult<()> {
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
pub fn max_weight_valid(max_weight: &str) -> aResult<()> {
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
