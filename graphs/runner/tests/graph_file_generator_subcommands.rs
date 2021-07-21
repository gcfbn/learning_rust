use runner_lib::*;

use anyhow::Result;

mod failing_tests {
    use super::*;
    use test_case::test_case;

    #[test]
    fn fails_because_edges_count_is_to_small() -> Result<()> {
        let parameters = "--graph-file aaa.txt --nodes-count 5 --edges-count 3 --max-weight 100"
            .parse::<GenerateGraphFileArgs>()
            .unwrap();

        let actual_error = generate_graph(&parameters).unwrap_err();
        let expected_error = GenerateGraphError::TooFewEdgesForConnectedGraph {
            edges_count: 3,
            nodes_count: 5,
        };

        assert_eq!(actual_error.to_string(), expected_error.to_string());
        Ok(())
    }

    fn validate_args(args: &str) -> Result<()> {
        let result = args.parse::<GenerateGraphFileArgs>();

        assert!(result.is_err());
        Ok(())
    }

    #[test_case("3a"; "nodes count is not integer")]
    #[test_case("'-1'"; "nodes count is not a positive integer")]
    #[test_case("0"; "nodes count is 0")]
    fn invalid_nodes_count(nodes_count_arg: &str) -> Result<()> {
        let args = format!(
            "--graph-file aaa.txt --nodes-count {} --edges-count 3 --max-weight 100",
            nodes_count_arg
        );
        validate_args(&args)
    }

    #[test_case("3a"; "edges count is not integer")]
    #[test_case("'-1'"; "edges count is not a positive integer")]
    #[test_case("0"; "edges count is 0")]
    fn invalid_edges_count(edges_count_arg: &str) -> Result<()> {
        let args = format!(
            "--graph-file aaa.txt --nodes-count 3 --edges-count {} --max-weight 100",
            edges_count_arg
        );
        validate_args(&args)
    }

    #[test_case("100a"; "max weight is not integer")]
    #[test_case("'-100'"; "max weight is not a positive integer")]
    #[test_case("0"; "max weight is 0")]
    fn invalid_max_weight(max_weight_arg: &str) -> Result<()> {
        let args = format!(
            "--graph-file aaa.txt --nodes-count 3 --edges-count 3 --max-weight {}",
            max_weight_arg
        );
        validate_args(&args)
    }

    #[test_case("--graph-file aaa.txt --nodes-count 5 --edges-count 3"; "1")]
    #[test_case("--graph-file aaa.txt --nodes-count 5"; "2")]
    #[test_case("--graph-file aaa.txt"; "3")]
    #[test_case(""; "4")]
    fn missing_required_options(args: &str) -> Result<()> {
        validate_args(&args)
    }
}

mod passing_tests {
    use super::*;
    use std::path::PathBuf;
    use tempfile::tempdir_in;

    #[test]
    fn ok() -> Result<()> {
        let temp_dir = tempdir_in(".")?;
        let file_path = PathBuf::from(temp_dir.path()).push("test_graph_file.txt");

        let parameters = format!(
            "--graph-file {:?} --nodes-count 5 --edges-count 4 --max-weight 100",
            file_path
        )
        .parse::<GenerateGraphFileArgs>()
        .unwrap();

        let result = generate_graph(&parameters);

        assert!(result.is_ok());

        temp_dir.close()?;

        Ok(())
    }
}
