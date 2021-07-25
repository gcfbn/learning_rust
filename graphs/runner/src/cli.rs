#![warn(missing_docs)]

use clap::{AppSettings, Clap};

use crate::errors::RunnerError;
use crate::subcommands::*;

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
    #[clap(visible_alias = "ggf")]
    GenerateGraphFile(GenerateGraphFileArgs),
    /// Runs algorithm using data from chosen file
    #[clap(visible_alias = "t")]
    RunAlgorithm(RunAlgorithmArgs),
}

impl SubCommand {
    /// Tries to build [`SubCommand`] variant from command line arguments
    ///
    /// Returns [`RunnerError::CommandLineArgsError`] on fail
    ///
    /// # Arguments
    ///
    /// * `command_name` - name of command used in command line
    /// * `args` - arguments passed with command
    ///
    /// # Example
    /// ```
    /// use runner::SubCommand;
    ///
    /// let command_name = "generate-graph-file";
    /// let args = "--graph-file aaa.txt --nodes-count 5 --edges-count 6 --max-weight 100";
    ///
    /// let ggf_subcommand = SubCommand::try_from_name_and_args(command_name, args);
    ///
    /// assert!(ggf_subcommand.is_ok());
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
