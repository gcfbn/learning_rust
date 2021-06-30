use graph::build_graph_from_file;
use kruskal::calculate_min_total_weight;
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
    let graph = build_graph_from_file(format!("tests/data/passing{}.txt", dataset_number)).unwrap();
    calculate_min_total_weight(graph)
}
