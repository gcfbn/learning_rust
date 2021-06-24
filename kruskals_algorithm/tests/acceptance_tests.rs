use kruskals_algorithm::{run, KruskalsAlgorithmError};
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

#[test]
fn failing_graph_not_connected() {
    let actual = run("tests/data/failing_graph_not_connected.txt").unwrap_err();
    let expected = KruskalsAlgorithmError::GraphNotConnected;
    assert_eq!(actual.to_string(), expected.to_string());
}
