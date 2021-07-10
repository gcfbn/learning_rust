use algorithms::calculate_min_total_weight;
use graph::build_graph;
use std::path::PathBuf;
use test_case::test_case;

// -----------------------------------------------------------------------------

#[test_case(1 => 280)]
#[test_case(2 => 0)]
#[test_case(3 => 600)]
#[test_case(4 => 9500)]
#[test_case(5 => 2500)]
#[test_case(6 => 2700)]
#[test_case(7 => 1500)]
#[test_case(8 => 400)]
fn passing(dataset_number: u32) -> i32 {
    let mut path = PathBuf::from("tests/data/passing");
    path.push(format!("{}", dataset_number));
    path.set_extension("txt");

    let graph = build_graph(&path).unwrap();

    calculate_min_total_weight(graph)
}
