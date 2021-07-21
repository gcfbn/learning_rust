use anyhow::Result as aResult;
use anyhow::{anyhow, Context};
use clap::{AppSettings, Clap};
use core::result::Result::Ok;
use std::path::PathBuf;

use crate::{RunnerError, SubCommand};
use std::str::FromStr;

/// Subcommand generating random graph file, which could be used in algorithms
#[derive(Clap, Debug)]
#[clap(setting = AppSettings::ColoredHelp)]
pub struct GenerateGraphFileArgs {
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
