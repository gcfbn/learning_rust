use runner::*;

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
    use tempfile::NamedTempFile;

    #[test]
    fn ok() -> Result<()> {
        let output_graph_file = NamedTempFile::new()?;

        let parameters = format!(
            "--graph-file {} --nodes-count 5 --edges-count 4 --max-weight 100",
            output_graph_file.path().to_str().unwrap()
        )
        .parse::<GenerateGraphFileArgs>()?;

        let result = generate_graph(&parameters);

        assert!(result.is_ok());
        assert!(output_graph_file.path().exists());

        let graph_result = build_graph(output_graph_file.path());

        // temp_dir is deleted even if the tests panics so I use this if statement to keep it
        if !graph_result.is_ok() {
            // keep temp_dir as a normal directory
            output_graph_file
                .into_temp_path()
                .persist("./passing_tests_ok_test_graph_file.txt")?;
            // panic because of an error
            panic!("{:?}", graph_result.unwrap_err());
        }

        Ok(())
    }
}
