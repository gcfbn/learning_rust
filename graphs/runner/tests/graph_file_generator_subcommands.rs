use runner_lib::*;

use super::*;
use anyhow::Result;

mod failing_tests {
    use super::*;
    use test_case::test_case;

    #[test]
    fn fails_because_edges_count_is_to_small() -> Result<()> {
        let parameters =
            GraphFileGenerator::try_from_args("--graph-file aaa.txt --nodes-count 5 --edges-count 3 --max-weight 100")
                .unwrap();

        let actual_error = generate_graph(&parameters).unwrap_err();
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
        let parameters =
            GraphFileGenerator::try_from_args("--graph-file aaa.txt --nodes-count 5 --edges-count 4 --max-weight 100")
                .unwrap();

        let result = generate_graph(&parameters);

        assert!(result.is_ok());

        Ok(())
    }
}
