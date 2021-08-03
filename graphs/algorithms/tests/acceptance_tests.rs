use graph::{build_graph, Graph};
use std::path::PathBuf;

// -----------------------------------------------------------------------------

fn build_graph_from_dataset_number(dataset_number: u32) -> Graph {
    let mut path = PathBuf::from("tests/data");
    path.push(format!("passing{}", dataset_number));
    path.set_extension("txt");

    build_graph(&path).unwrap()
}

mod kruskal {
    use super::*;
    use algorithms::calculate_min_total_weight;
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
        let graph = build_graph_from_dataset_number(dataset_number);

        calculate_min_total_weight(graph)
    }
}

mod dijkstra {
    use super::*;
    use algorithms::{find_shortest_path_length, DijkstraAlgorithmError};
    use test_case::test_case;
    use utils::PositiveInteger;

    #[test_case(1, 1, 2 => 50)]
    fn passing(dataset_number: u32, start_node: u32, end_node: u32) -> u32 {
        let graph = build_graph_from_dataset_number(dataset_number);
        find_shortest_path_length(&graph, PositiveInteger::new(start_node), PositiveInteger::new(end_node)).unwrap()
    }
}
