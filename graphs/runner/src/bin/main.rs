use clap::Clap;
use std::process;

use runner_lib::*;

type Result<T, E = RunnerError> = std::result::Result<T, E>;

/// Main function that is called when the app starts
///
/// Calls run() function and kills process if it returns an error
fn main() {
    if let Err(err) = run() {
        println!("Error: {}", err);
        process::exit(1);
    }
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
    // use crate::cli::{max_weight_valid, nodes_count_valid};
    use test_case::test_case;

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
