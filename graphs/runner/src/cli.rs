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
    /// Returns [`RunnerError::SubcommandCreatingError`] on fail
    ///
    /// # Arguments
    ///
    /// * `command_name` - name of command used in command line
    /// * `args` - arguments passed with command
    ///
    /// # Example
    /// ```
    /// use runner_lib::SubCommand;
    ///
    /// let command_name = "graph-file-generator";
    /// let args = "--graph-file aaa.txt --nodes-count 5 --edges-count 6 --max-weight 100";
    ///
    /// let gfg_subcommand = SubCommand::try_from_name_and_args(command_name, args);
    ///
    /// assert!(gfg_subcommand.is_ok());
    /// ```
    pub fn try_from_name_and_args(command_name: &str, args: &str) -> Result<Self, RunnerError> {
        let cli_string = format!(
            "{app} {command} {args}",
            app = APP_NAME,
            command = command_name,
            args = args
        );

        let cmd_args = CmdArgs::try_parse_from(cli_string.split_whitespace())?;

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
    #[clap(long, short, validator(edges_count_valid))]
    pub edges_count: u32,

    /// Maximum weight of an edge in graph (must be an positive integer)
    #[clap(long, short, validator(max_weight_valid))]
    pub max_weight: u32,
}

impl GraphFileGenerator {
    /// Tries to build [`GraphFileGenerator`] from command line args
    ///
    /// # Arugments
    ///
    /// * `args` - command line arguments for graph-file-generator subcommand
    ///
    /// # Example
    /// ```
    /// use runner_lib::GraphFileGenerator;
    ///
    /// let args = "--graph-file aaa.txt --nodes-count 5 --edges-count 3 --max-weight 100";
    /// let gfg = GraphFileGenerator::try_from_args(args);
    ///
    /// assert!(gfg.is_ok());
    /// ```
    pub fn try_from_args(args: &str) -> aResult<Self> {
        match SubCommand::try_from_name_and_args("graph-file-generator", args)? {
            SubCommand::GraphFileGenerator(cmd) => Ok(cmd),
            // this should never happen, because if args aren't matching graph-file-generator arguments,
            // error will be returned after calling `SubCommand::try_from_name_and_args`
            _ => panic!("this should never happen !"),
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

fn must_be_positive_integer(count: &str) -> aResult<()> {
    let count = count
        .parse::<u32>()
        .with_context(|| format!("cannot be negative '{}'", count))?;

    if count == 0 {
        return Err(anyhow!("must be a positive integer '{}'", count));
    }

    Ok(())
}

/// Checks if number of nodes is correct (has to be a positive integer)
///
/// # Arguments
///
/// `nodes_count` - Number of nodes given by user
pub fn nodes_count_valid(nodes_count: &str) -> aResult<()> {
    must_be_positive_integer(nodes_count)
}

/// Checks if number of edges is correct (has to be a positive integer)
///
/// # Arguments
///
/// `edges_count` - Number of edges given by user
pub fn edges_count_valid(edges_count: &str) -> aResult<()> {
    must_be_positive_integer(edges_count)
}

/// Checks if maximum edge weight is correct (has to be a positive integer)
///
/// # Arguments
///
/// `max_weight` - Max weight given by user
pub fn max_weight_valid(max_weight: &str) -> aResult<()> {
    must_be_positive_integer(max_weight)
}

#[cfg(test)]
mod tests {

    use super::*;
    use test_case::test_case;

    #[test]
    fn nodes_count_ok() {
        let nodes_count = "200";
        let result = nodes_count_valid(nodes_count);
        assert!(result.is_ok());
    }

    #[test_case("200a"; "not_a_number")]
    #[test_case("0"; "zero")]
    #[test_case("-123"; "negative")]
    #[test_case("3,5"; "not an integer")]
    fn nodes_count_invalid(nodes_count: &str) {
        let result = nodes_count_valid(nodes_count);
        assert!(result.is_err());
    }

    #[test]
    fn max_weight_ok() {
        let max_weight = "300";
        let result = max_weight_valid(max_weight);
        assert!(result.is_ok());
    }

    #[test_case("100a0"; "not_a_number")]
    #[test_case("0"; "zero")]
    #[test_case("-3"; "negative")]
    #[test_case("7,25"; "not an integer")]
    fn max_weight_invalid(max_weight: &str) {
        let result = max_weight_valid(max_weight);
        assert!(result.is_err());
    }
}
