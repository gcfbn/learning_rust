pub fn dfs(index: usize, adjacency_list: &[Vec<usize>], visited: &mut Vec<bool>) {
    visited[index] = true;
    for &neighbour in &adjacency_list[index] {
        if !visited[neighbour] {
            dfs(neighbour, adjacency_list, visited)
        }
    }
}
