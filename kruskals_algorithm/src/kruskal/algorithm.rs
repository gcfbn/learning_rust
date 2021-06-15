use crate::data::structures;

pub fn calculate_min_total_weight(graph: structures::Graph) -> i32 {

    // create array of parental nodes
    // on initialization, all nodes have themself as their parental nodes
    // in this task, nodes are indexed from one, so index 0 of this array won't be used
    // that's why array has size n + 1
    let mut parents: Vec<u32> = Vec::with_capacity((graph.nodes_count + 1) as usize);

    for i in 0..graph.nodes_count + 1 {
        parents.push(i);
    }

    // copy all edges from the graph to result vector of edges
    let mut result_edges = graph.edges;

    // sort result_edges by ascending weight
    result_edges.sort_by(|a, b| a.weight.cmp(&b.weight));

    let mut from_parent: u32 = 0;
    let mut to_parent: u32 = 0;

    // remove edges that connect nodes with the same parental node
    // when lambda expression returns false, edge will be removed
    result_edges.retain(|&e| {
        from_parent = find_parent(&parents, e.from_index);
        to_parent = find_parent(&parents, e.to_index);
        if from_parent == to_parent {
            false
        } else {
            parents[to_parent as usize] = from_parent;
            true
        }
    });

    result_edges.iter().map(|&e| e.weight as i32).sum()
}

fn find_parent(parents: &[u32], index: u32) -> u32 {
    if parents[index as usize] != index { find_parent(parents, parents[index as usize]) } else { index }
}