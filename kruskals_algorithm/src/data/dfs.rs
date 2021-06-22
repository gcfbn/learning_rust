use crate::data::structures::Edge;

pub fn is_connected(edges: &[Edge], nodes_count: u32) -> bool {
    let mut adjacency_list: Vec<Vec<usize>> = vec!(Vec::new(); (nodes_count + 1) as usize);

    for edge in edges {
        adjacency_list[edge.from_index as usize].push(edge.to_index as usize);
        adjacency_list[edge.to_index as usize].push(edge.from_index as usize);
    }

    let mut visited: Vec<bool> = vec![false; (nodes_count + 1) as usize];
    dfs(1, &adjacency_list, &mut visited);

    for value in visited.iter().skip(1) {
        if !value { return false; }
    }
    true
}

fn dfs(index: usize, adjacency_list: &[Vec<usize>], visited: &mut Vec<bool>) {
    visited[index] = true;
    for &neighbour in &adjacency_list[index] {
        if !visited[neighbour] {
            dfs(neighbour, adjacency_list, visited)
        }
    }
}