use crate::Edge;

pub type AdjacencyList = Vec<Vec<(usize, usize)>>;

pub fn adjacency_list(edges: &[Edge], nodes_count: u32) -> AdjacencyList {
    let mut adjacency_list: AdjacencyList = vec![Vec::new(); (nodes_count + 1) as usize];

    for edge in edges {
        adjacency_list[edge.from_index as usize].push((edge.to_index as usize, edge.weight as usize));
        adjacency_list[edge.to_index as usize].push((edge.from_index as usize, edge.weight as usize));
    }

    adjacency_list
}
