use graph::Graph;

pub fn calculate_min_total_weight(mut graph: Graph) -> i32 {
    // create array of parental nodes
    // on initialization, all nodes have themselves as parent nodes
    // in this task, nodes are indexed from one, so index 0 of this array won't be used
    // that's why array has size n + 1
    let mut parents: Vec<u32> = Vec::with_capacity((graph.nodes_count + 1) as usize);

    for i in 0..=graph.nodes_count {
        parents.push(i);
    }

    // sort result_edges by ascending weight
    graph.edges.sort_by(|a, b| a.weight.cmp(&b.weight));

    // remove edges that connect nodes with the same parental node
    // when lambda expression returns false, edge will be removed
    graph.edges.retain(|&e| {
        let from_parent = find_parent(&parents, e.from_index);
        let to_parent = find_parent(&parents, e.to_index);
        if from_parent == to_parent {
            false
        } else {
            parents[to_parent as usize] = from_parent;
            true
        }
    });

    graph.edges.iter().map(|e| e.weight as i32).sum()
}

fn find_parent(parents: &[u32], index: u32) -> u32 {
    if parents[index as usize] != index {
        find_parent(parents, parents[index as usize])
    } else {
        index
    }
}

/*
fn _find_parent_no_recursion(parents: &[u32], index: u32) -> u32 {
    let mut current_index = index;
    while parents[current_index as usize] != current_index {
        current_index = parents[current_index as usize];
    }
    current_index
}
*/

#[cfg(test)]
#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_case::test_case;

    #[test_case(1 => 10; "last element is parent of the first element")]
    #[test_case(10 => 10; "last element is it's own parent")]
    fn find_parent_when_all_nodes_are_connected(index: u32) -> u32 {
        let parents = [0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 10];
        find_parent(&parents, index)
    }

    #[test_case(1 => 1)]
    #[test_case(4 => 1)]
    #[test_case(8 => 6)]
    #[test_case(6 => 6)]
    fn find_parent_disjoint_set(index: u32) -> u32 {
        let parents = [0, 1, 1, 2, 1, 7, 6, 6, 5];
        find_parent(&parents, index)
    }
}
