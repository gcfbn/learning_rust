use graph::{build_graph, AddingEdgeError, BuildGraphError, Edge, GraphParametersParsingError, ParsingEdgeError};
use std::path::PathBuf;
use test_case::test_case;

// -----------------------------------------------------------------------------

fn build_path(dir: &str, graph_file: &str) -> PathBuf {
    let mut path = PathBuf::from("tests/data");
    path.push(dir);
    path.push(graph_file);
    path.set_extension("txt");

    path
}

fn validate_graph_file(dir_name: &str, graph_file: &str, expected_error: BuildGraphError) {
    let path = build_path(dir_name, graph_file);
    let actual_error = build_graph(&path).unwrap_err();

    assert_eq!(actual_error.to_string(), expected_error.to_string());
}

// Copied from: https://stackoverflow.com/questions/38088067/equivalent-of-func-or-function-in-rust
macro_rules! function {
    () => {{
        fn f() {}
        fn type_name_of<T>(_: T) -> &'static str {
            std::any::type_name::<T>()
        }
        let name = type_name_of(f);

        // Find and cut the rest of the path
        match &name[..name.len() - 3].rfind(':') {
            Some(pos) => &name[pos + 1..name.len() - 3],
            None => &name[..name.len() - 3],
        }
    }};
}

// -----------------------------------------------------------------------------
// When the first line in the graph description file is invalid
// -----------------------------------------------------------------------------

#[test_case("error_parsing_graph_parameters_empty_input",
            GraphParametersParsingError::EmptyInput;
            "error_parsing_graph_parameters_empty_input"
)]
#[test_case("error_parsing_graph_parameters_missing_edges_count",
            GraphParametersParsingError::MissingEdgesCountValue;
            "error_parsing_graph_parameters_missing_edges_count"
)]
#[test_case("error_parsing_graph_parameters_nodes_count",
            GraphParametersParsingError::NodesCountValueMustBeInteger("X".to_owned());
            "error_parsing_graph_parameters_nodes_count"
)]
#[test_case("error_parsing_graph_parameters_edges_count",
            GraphParametersParsingError::EdgesCountValueIsNotInteger("X".to_owned());
            "error_parsing_graph_parameters_edges_count"
)]
fn parsing_graph_parameters_errors(graph_file: &str, expected_error: GraphParametersParsingError) {
    validate_graph_file(function!(), graph_file, BuildGraphError::from(expected_error));
}

// -----------------------------------------------------------------------------
// When lines: 2..EOF in the graph description file are invalid
// -----------------------------------------------------------------------------

#[test_case("error_parsing_edge_non_integer_from_index", 2,
            ParsingEdgeError::FromIndexValueMustBeInteger("xyz".to_owned());
            "error_parsing_edge_non_integer_from_index"
)]
#[test_case("error_parsing_edge_non_integer_to_index", 2,
            ParsingEdgeError::ToIndexValueMustBeInteger("abc".to_owned());
            "error_parsing_edge_non_integer_to_index"
)]
#[test_case("error_parsing_edge_non_integer_weight", 2,
            ParsingEdgeError::WeightValueMustBeInteger("10a0".to_owned());
            "error_parsing_edge_non_integer_weight"
)]
#[test_case("error_parsing_edge_empty_line", 2,
            ParsingEdgeError::EmptyLine;
            "error_creating_edge_empty_line"
)]
#[test_case("error_parsing_edge_missing_to_index", 4,
            ParsingEdgeError::MissingToIndexField;
            "error_creating_edge_missing_to_index"
)]
#[test_case("error_parsing_edge_missing_weight", 3,
            ParsingEdgeError::MissingWeightField;
            "error_edge_description_missing_weight"
)]
#[test_case("error_adding_edge_too_many_edges", 4,
            AddingEdgeError::TooManyEdges{
                edges_count: 3,
                edge: Edge{ from_index: 1, to_index: 4, weight: 200 }
            };
            "error_adding_edge_too_many_edges"
)]
#[test_case("error_adding_edge_wrong_from_index", 3,
            AddingEdgeError::WrongFromIndex{
                nodes_count: 4,
                edge: Edge{ from_index: 5, to_index: 3, weight: 100, },
            };
            "error_adding_edge_wrong_from_index"
)]
#[test_case("error_adding_edge_wrong_to_index", 2,
            AddingEdgeError::WrongToIndex{
                nodes_count: 3,
                edge: Edge{ from_index: 1, to_index: 4, weight: 100, },
            };
            "error_adding_edge_wrong_to_index"
)]
fn edge_errors<E: Into<BuildGraphError>>(graph_file: &str, expected_line_no_with_error: usize, expected_error: E) {
    let expected_error = expected_error.into();

    let path = build_path(function!(), graph_file);
    let actual_error = build_graph(&path).unwrap_err();

    if let BuildGraphError::ErrorInGraphDescriptionFile {
        line_no: actual_line_no_with_error,
        error: actual_error,
    } = actual_error
    {
        assert_eq!(actual_line_no_with_error, expected_line_no_with_error);
        assert_eq!(actual_error.to_string(), expected_error.to_string());
    } else {
        panic!("unexpected error !")
    }
}

// -----------------------------------------------------------------------------

#[test_case("error_graph_not_connected", BuildGraphError::GraphNotConnected; "error_graph_not_connected")]
#[test_case("error_too_few_edges", BuildGraphError::TooFewEdges{current_count: 3, declared: 4}; "error_too_few_edges")]
fn graph_building_errors(graph_file: &str, expected_error: BuildGraphError) {
    validate_graph_file(function!(), graph_file, expected_error);
}
