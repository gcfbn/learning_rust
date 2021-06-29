use kruskals_algorithm::{
    run,
    BuildGraphError,
    CreatingEdgeError,
    Edge,
    EdgeDescription,
    EdgeDescriptionError,
    GraphParametersParsingError,
};
use test_case::test_case;

#[test_case(1 => 280)]
#[test_case(2 => 0)]
#[test_case(3 => 600)]
#[test_case(4 => 9500)]
#[test_case(5 => 2500)]
#[test_case(6 => 2700)]
#[test_case(7 => 1500)]
#[test_case(8 => 400)]
fn passing(dataset_number: u32) -> i32 {
    run(format!("tests/data/passing_tests/passing{}.txt", dataset_number)).unwrap()
}

#[test_case("error_graph_not_connected", BuildGraphError::GraphNotConnected)]
#[test_case("error_too_few_edges", BuildGraphError::TooFewEdges{current_count: 3, declared: 4})]
#[test_case("error_not_enough_data", BuildGraphError::NotEnoughData)]
#[test_case(
    "error_parsing_graph_parameters_nodes_count",
    BuildGraphError::from(GraphParametersParsingError::from_non_integer_nodes_count("X"))
)]
#[test_case(
    "error_parsing_graph_parameters_edges_count",
    BuildGraphError::from(GraphParametersParsingError::from_non_integer_edges_count("X"))
)]
fn test_graph_building_errors(graph_file: &str, expected_error: BuildGraphError) {
    let actual_error = run(format!(
        "tests/data/error_tests/graph_building_errors/{}.txt",
        graph_file
    ))
    .unwrap_err();
    assert_eq!(actual_error.to_string(), expected_error.to_string());
}

#[test_case("error_edge_description_bad_from_index", 2,
            BuildGraphError::from(CreatingEdgeError::from_edge_description_with_bad_from_index(
                &EdgeDescription { from_index: "xyz", to_index: "3", weight: "100" }
            )); "error_edge_description_bad_from_index"
 )]
#[test_case("error_edge_description_bad_to_index", 2,
            BuildGraphError::from(CreatingEdgeError::from_edge_description_with_bad_to_index(
                &EdgeDescription { from_index: "1", to_index: "abc", weight: "150" }
            )); "error_edge_description_bad_to_index"
)]
#[test_case("error_edge_description_bad_weight", 2,
            BuildGraphError::from(CreatingEdgeError::from_edge_description_with_bad_weight(
                &EdgeDescription { from_index: "1", to_index: "2", weight: "10a0" }
            )); "error_edge_description_bad_weight"
)]
#[test_case(
"error_edge_description_empty_input", 2,
BuildGraphError::from(EdgeDescriptionError::EmptyInput); "error_edge_description_empty_input")]
#[test_case(
"error_edge_description_missing_to_index", 4,
BuildGraphError::from(EdgeDescriptionError::MissingToIndexField); "error_edge_description_missing_to_index")]
#[test_case(
"error_edge_description_missing_weight", 3,
BuildGraphError::from(EdgeDescriptionError::MissingWeightField); "error_edge_description_missing_weight"
)]
#[test_case("error_too_many_edges", 4, 
    BuildGraphError::TooManyEdges{
        edges_count: 3,
        edge: Edge{
            from_index: 1,
            to_index: 4,
            weight: 200
        }
})]
#[test_case("error_edge_description_wrong_from_index", 3, 
    BuildGraphError::from(EdgeDescriptionError::WrongFromIndex{
        edge: Edge{
            from_index: 5,
            to_index: 3,
            weight: 100,
        },
        nodes_count: 4,
}))]
#[test_case("error_edge_description_wrong_to_index", 2, 
    BuildGraphError::from(EdgeDescriptionError::WrongToIndex{
        edge: Edge{
            from_index: 1,
            to_index: 4,
            weight: 100,
        },
        nodes_count: 3,
}))]
fn test_creating_edge_errors(graph_file: &str, expected_line_no_with_error: usize, expected_error: BuildGraphError) {
    let result = run(format!(
        "tests/data/error_tests/creating_edge_errors/{}.txt",
        graph_file
    ))
    .unwrap_err();

    if let BuildGraphError::ErrorInGraphDescriptionFile {
        line_no: actual_line_no_with_error,
        error: actual_error,
    } = result
    {
        assert_eq!(actual_line_no_with_error, expected_line_no_with_error);
        assert_eq!(actual_error.to_string(), expected_error.to_string());
    } else {
        panic!("invalid error !")
    }
}
