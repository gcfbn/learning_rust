use clap::Clap;
use std::process;

use graph_file_generator::generate_graph;

mod cli;
mod errors;
mod graph_file_generator;

use crate::cli::*;
use crate::errors::RunnerError;

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
    use test_case::test_case;

    use super::*;

    mod subcommand_graph_file_generator {
        use anyhow::Result;

        use super::*;

        mod failing_tests {
            use test_case::test_case;

            use crate::errors::GraphFileGeneratorError;

            use super::*;

            #[test]
            fn fails_because_edges_count_is_to_small() -> Result<()> {
                let parameters = GraphFileGenerator::try_from_args(
                    "--graph-file aaa.txt --nodes-count 5 --edges-count 3 --max-weight 100",
                )
                .unwrap();

                let actual_error = graph_file_generator::generate_graph(&parameters).unwrap_err();
                let expected_error = GraphFileGeneratorError::TooFewEdgesForConnectedGraph {
                    edges_count: 3,
                    nodes_count: 5,
                };

                assert_eq!(actual_error.to_string(), expected_error.to_string());
                Ok(())
            }

            #[test_case("--graph-file aaa.txt --nodes-count 5a --edges-count 3 --max-weight 100"; "nodes count is not integer")]
            #[test_case("--graph-file aaa.txt --nodes-count 5 --edges-count 3a --max-weight 100"; "edges count is not integer")]
            #[test_case("--graph-file aaa.txt --nodes-count 5 --edges-count 3 --max-weight 100a"; "max weight is not integer")]
            #[test_case("--graph-file aaa.txt --nodes-count 5 --edges-count 3a"; "missing max weight")]
            #[test_case("--graph-file aaa.txt --nodes-count 0 --edges-count 3 --max-weight 100"; "nodes count is zero")]
            #[test_case("--graph-file aaa.txt --nodes-count -5 --edges-count 3 --max-weight 100"; "nodes count is negative")]
            fn fails_because_of_invalid_input(args: &str) -> Result<()> {
                let result = GraphFileGenerator::try_from_args(args);

                // probably should check error type
                assert!(result.is_err());
                Ok(())
            }
        }

        mod passing_tests {
            use super::*;

            #[test]
            fn ok() -> Result<()> {
                let parameters = GraphFileGenerator::try_from_args(
                    "--graph-file aaa.txt --nodes-count 5 --edges-count 4 --max-weight 100",
                )
                .unwrap();

                let result = generate_graph(&parameters);

                assert!(result.is_ok());

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
