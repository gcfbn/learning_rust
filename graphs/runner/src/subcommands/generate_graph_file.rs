use clap::{AppSettings, Clap};
use core::result::Result::Ok;
use std::path::PathBuf;

use crate::{RunnerError, SubCommand};
use std::str::FromStr;

use utils::PositiveInteger;

/// Subcommand generating random graph file, which could be used in algorithms
#[derive(Clap, Debug)]
#[clap(setting = AppSettings::ColoredHelp)]
pub struct GenerateGraphFileArgs {
    /// Output filename
    #[clap(long, short)]
    pub graph_file: PathBuf,

    /// Number of nodes in graph (indexed from 1 to `nodes_count`, so must be positive)
    #[clap(long, short)]
    pub nodes_count: PositiveInteger,

    /// Number of edges in graph
    #[clap(long, short)]
    pub edges_count: PositiveInteger,

    /// Maximum weight of an edge in graph (must be an positive integer)
    #[clap(long, short)]
    pub max_weight: PositiveInteger,
}

impl FromStr for GenerateGraphFileArgs {
    type Err = RunnerError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match SubCommand::try_from_name_and_args("generate-graph-file", s)? {
            SubCommand::GenerateGraphFile(cmd) => Ok(cmd),
            // this should never happen, because if args aren't matching graph-file-generator arguments,
            // error will be returned after calling `SubCommand::try_from_name_and_args`
            _ => panic!("this should never happen !"),
        }
    }
}
