use kruskals_algorithm::{run, BuildGraphError, Edge, EdgeDescriptionError};
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
    run(format!("tests/data/passing{}.txt", dataset_number)).unwrap()
}

#[test_case("error_graph_not_connected", BuildGraphError::GraphNotConnected)]
#[test_case("error_too_few_edges", BuildGraphError::TooFewEdges{current_count: 3, declared: 4})]
#[test_case("error_wrong_from_index", BuildGraphError::from(EdgeDescriptionError::WrongFromIndex{
    edge: Edge{
        from_index: 5,
        to_index: 3,
        weight: 100,
        },
    nodes_count: 4,
    })
)]
#[test_case("error_wrong_to_index", BuildGraphError::from(EdgeDescriptionError::WrongToIndex{
    edge: Edge{
        from_index: 1,
        to_index: 4,
        weight: 100,
        },
    nodes_count: 3,
}))]
#[test_case("error_too_many_edges", BuildGraphError::TooManyEdges{
    max_edges_count: 3,
    edge: Edge{
        from_index: 1,
        to_index: 4,
        weight: 200
        }
})]
#[test_case("error_not_enough_data", BuildGraphError::NotEnoughData)]
fn test_graph_building_errors(graph_file: &str, expected_error: BuildGraphError) {
    let actual_error = run(format!("tests/data/{}.txt", graph_file)).unwrap_err();
    assert_eq!(actual_error.to_string(), expected_error.to_string());
}
