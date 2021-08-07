/// Non-recursive implementation of Depth First Search algorithm
///
/// # Arguments
///
/// * `index` - Algorithm starting index
/// * `adjacency_list` - Array slice of vectors containing data about graph edges
pub fn dfs(start_index: usize, adjacency_list: &[Vec<(usize, usize)>]) -> Vec<bool> {
    let mut visited = vec![false; adjacency_list.len()];
    let mut stack = vec![];

    visited[start_index] = true;
    stack.push(start_index);

    while !stack.is_empty() {
        // there is at least one element on the stack, so `unwrap` is safe
        let popped_element = stack.pop().unwrap();
        visited[popped_element] = true;

        for neighbour in &adjacency_list[popped_element] {
            if !visited[neighbour.0] {
                stack.push(neighbour.0);
            }
        }
    }

    visited
}
