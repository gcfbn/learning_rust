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

    #[test]
    fn ok() -> Result<()> {
        let parameters =
            "--graph-file aaa.txt --nodes-count 5 --edges-count 4 --max-weight 100".parse::<GenerateGraphFileArgs>()?;

        let result = generate_graph(&parameters);

        assert!(result.is_ok());

        Ok(())
    }
}
