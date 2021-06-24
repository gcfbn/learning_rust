use crate::data::structures::Edge;

pub fn is_connected(edges: &[Edge], nodes_count: u32) -> bool {
    let mut adjacency_list: Vec<Vec<usize>> = vec![Vec::new(); (nodes_count + 1) as usize];

    for edge in edges {
        adjacency_list[edge.from_index as usize].push(edge.to_index as usize);
        adjacency_list[edge.to_index as usize].push(edge.from_index as usize);
    }

    let mut visited: Vec<bool> = vec![false; (nodes_count + 1) as usize];
    dfs(1, &adjacency_list, &mut visited);

    for value in visited.iter().skip(1) {
        if !value {
            return false;
        }
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

#[cfg(test)]
mod tests {
    use crate::data::dfs::is_connected;
    use crate::data::structures::Edge;

    #[test]
    fn graph_not_connected() {
        let nodes_count = 5;
        let mut edges: Vec<Edge> = Vec::new();
        edges.push(Edge {
            from_index: 1,
            to_index:   2,
            weight:     100,
        });
        edges.push(Edge {
            from_index: 3,
            to_index:   4,
            weight:     100,
        });
        edges.push(Edge {
            from_index: 4,
            to_index:   5,
            weight:     100,
        });
        edges.push(Edge {
            from_index: 5,
            to_index:   3,
            weight:     100,
        });
        assert_eq!(false, is_connected(&edges, nodes_count));
    }

    #[test]
    fn graph_connected() {
        let nodes_count = 5;
        let mut edges: Vec<Edge> = Vec::new();
        edges.push(Edge {
            from_index: 1,
            to_index:   2,
            weight:     100,
        });
        edges.push(Edge {
            from_index: 4,
            to_index:   5,
            weight:     100,
        });
        edges.push(Edge {
            from_index: 3,
            to_index:   5,
            weight:     100,
        });
        edges.push(Edge {
            from_index: 1,
            to_index:   4,
            weight:     100,
        });
        assert_eq!(true, is_connected(&edges, nodes_count));
    }
}
