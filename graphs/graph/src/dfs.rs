/// Recursive implementation of Depth First Search algorithm
///
/// # Arguments
///
/// * `index` - Algorithm starting index
/// * `adjacency_list` - Array slice of vectors containing data about graph edges
/// * `visited` - Vector containing data about visited nodes
pub fn dfs(index: usize, adjacency_list: &[Vec<usize>], visited: &mut Vec<bool>) {
    visited[index] = true;
    for &neighbour in &adjacency_list[index] {
        if !visited[neighbour] {
            dfs(neighbour, adjacency_list, visited)
        }
    }
}
