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
    use algorithms::DijkstrasError;
    use algorithms::{find_shortest_path_length, AlgorithmError};
    use test_case::test_case;
    use utils::PositiveInteger;

    fn test_error(dataset_number: u32, start_node: u32, end_node: u32, expected_error: AlgorithmError) {
        let graph = build_graph_from_dataset_number(dataset_number);
        let actual_error =
            find_shortest_path_length(&graph, PositiveInteger::new(start_node), PositiveInteger::new(end_node))
                .unwrap_err();

        assert_eq!(actual_error.to_string(), expected_error.to_string())
    }

    // -----------------------------------------------------------------------------

    #[test_case(1, 1, 2 => 50)]
    #[test_case(1, 1, 4 => 120)]
    #[test_case(1, 5, 4 => 160)]
    #[test_case(3, 5, 6 => 300)]
    #[test_case(3, 1, 8 => 350)]
    #[test_case(6, 1, 3 => 400)]
    #[test_case(6, 5, 2 => 300)]
    fn passing(dataset_number: u32, start_node: u32, end_node: u32) -> u32 {
        let graph = build_graph_from_dataset_number(dataset_number);
        find_shortest_path_length(&graph, PositiveInteger::new(start_node), PositiveInteger::new(end_node)).unwrap()
    }

    #[test_case(1, 6, 1, DijkstrasError::InvalidStartNode {
        start_node: 6,
        nodes_count: 5,
    })]
    #[test_case(7, 6, 7, DijkstrasError::InvalidEndNode {
    end_node: 7,
    nodes_count: 6,
    })]
    fn failing(dataset_number: u32, start_node: u32, end_node: u32, expected_error: DijkstrasError) {
        test_error(
            dataset_number,
            start_node,
            end_node,
            AlgorithmError::from(expected_error),
        )
    }
}
