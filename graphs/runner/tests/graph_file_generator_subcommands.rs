use runner_lib::*;

use anyhow::Result;

mod failing_tests {
    use super::*;
    use test_case::test_case;

    fn validate_args(args: &str) {
        let result = args.parse::<GenerateGraphFileArgs>();

        assert!(result.is_err());
    }

    #[test_case("3a"; "nodes count is not integer")]
    #[test_case("'-1'"; "nodes count is not a positive integer")]
    #[test_case("0"; "nodes count is 0")]
    fn invalid_nodes_count(nodes_count_arg: &str) {
        let args = format!(
            "--graph-file aaa.txt --nodes-count {} --edges-count 3 --max-weight 100",
            nodes_count_arg
        );
        validate_args(&args)
    }

    #[test_case("3a"; "edges count is not integer")]
    #[test_case("'-1'"; "edges count is not a positive integer")]
    #[test_case("0"; "edges count is 0")]
    fn invalid_edges_count(edges_count_arg: &str) {
        let args = format!(
            "--graph-file aaa.txt --nodes-count 3 --edges-count {} --max-weight 100",
            edges_count_arg
        );
        validate_args(&args)
    }

    #[test_case("100a"; "max weight is not integer")]
    #[test_case("'-100'"; "max weight is not a positive integer")]
    #[test_case("0"; "max weight is 0")]
    fn invalid_max_weight(max_weight_arg: &str) {
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
    fn missing_required_options(args: &str) {
        validate_args(&args)
    }
}

mod passing_tests {
    use super::*;
    use graph::build_graph;
    use std::path::PathBuf;
    use tempfile::tempdir_in;

    #[test]
    fn ok() -> Result<()> {
        let temp_dir = tempdir_in("")?;
        let mut temp_file = PathBuf::from(temp_dir.path());
        temp_file.push("test_graph_file.txt");

        let parameters = format!(
            "--graph-file {} --nodes-count 5 --edges-count 4 --max-weight 100",
            temp_file.to_str().unwrap()
        )
        .parse::<GenerateGraphFileArgs>()?;

        let result = generate_graph(&parameters);

        assert!(result.is_ok());
        assert!(temp_file.exists());

        let graph_result = build_graph(temp_file.as_path());

        // temp_dir is deleted even if the tests panics so I use this if statement to keep it
        if !graph_result.is_ok() {
            // keep temp_dir as a normal directory
            temp_dir.into_path();
            // panic because of an error
            panic!("{:?}", graph_result.unwrap_err());
        }

        Ok(())
    }
}
