use crate::Graph;

pub fn calculate_min_total_weight(mut graph: Graph) -> i32 {
    // create array of parental nodes
    // on initialization, all nodes have themselves as parent nodes
    // in this task, nodes are indexed from one, so index 0 of this array won't be used
    // that's why array has size n + 1
    let mut parents: Vec<u32> = Vec::with_capacity((graph.nodes_count + 1) as usize);

    for i in 0..=graph.nodes_count {
        parents.push(i);
    }

    // copy all edges from the graph to result vector of edges

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
