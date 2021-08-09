use crate::{RunnerError, SubCommand};
use anyhow::{anyhow, Result as aResult};
use clap::{AppSettings, Clap};
use std::path::{Path, PathBuf};
use std::str::FromStr;
use utils::PositiveInteger;

/// Subcommand running Kruskal's algorithm for graph built from `task_file`
#[derive(Clap, Debug)]
#[clap(setting = AppSettings::ColoredHelp)]
pub struct RunAlgorithmArgs {
    /// Name of file containing graph data
    #[clap(long, short, parse(from_os_str), validator(file_exists))]
    pub task_file: PathBuf,

    #[clap(subcommand)]
    pub algorithm_args: AlgorithmArgs,
}

#[derive(Clap, Debug)]
pub enum AlgorithmArgs {
    Kruskals {},
    Dijkstras {
        #[clap(long, short)]
        start_node: PositiveInteger,
        #[clap(long, short)]
        end_node:   PositiveInteger,
    },
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

impl FromStr for RunAlgorithmArgs {
    type Err = RunnerError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match SubCommand::try_from_name_and_args("run-algorithm", s)? {
            SubCommand::RunAlgorithm(cmd) => Ok(cmd),
            // this should never happen, because if args aren't matching run-algorithm arguments,
            // error will be returned after calling `SubCommand::try_from_name_and_args`
            _ => panic!("this should never happen !"),
        }
    }
}
