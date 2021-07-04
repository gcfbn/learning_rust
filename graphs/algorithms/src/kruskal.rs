use graph::Graph;
use std::cmp::Ordering;

#[derive(Debug, Clone)]
pub struct UnionFind {
    parent: Vec<u32>,
    rank:   Vec<u8>,
}

impl UnionFind {
    /// n - count of disjoint sets.
    pub fn new(n: u32) -> Self {
        let rank = vec![0; (n + 1) as usize];
        let mut parent = Vec::with_capacity(n as usize);

        for i in 0..=n {
            parent.push(i);
        }

        UnionFind { parent, rank }
    }

    pub fn find_parent(&self, index: u32) -> u32 {
        assert!(index < self.parent.len() as u32);

        let mut current_index = index;
        while self.parent[current_index as usize] != current_index {
            current_index = self.parent[current_index as usize];
        }
        current_index
    }

    pub fn find_mut_parent(&mut self, index: u32) -> u32 {
        assert!(index < self.parent.len() as u32);

        let mut index = index;

        let mut parent = self.parent[index as usize];
        while parent != index {
            let grandparent = self.parent[parent as usize];
            self.parent[index as usize] = grandparent;
            index = parent;
            parent = grandparent;
        }
        index
    }

    pub fn merge_parents(&mut self, x: u32, y: u32) -> bool {
        let x_parent = self.find_parent(x);
        let y_parent = self.find_parent(y);
        if x_parent != y_parent {
            self.union(x_parent, y_parent)
        } else {
            false
        }
    }

    pub fn union(&mut self, x: u32, y: u32) -> bool {
        if x == y {
            return false;
        }
        let xrep = self.find_mut_parent(x);
        let yrep = self.find_mut_parent(y);

        if xrep == yrep {
            return false;
        }

        let xrepu = xrep as usize;
        let yrepu = yrep as usize;

        let xrank = self.rank[xrepu];
        let yrank = self.rank[yrepu];

        match xrank.cmp(&yrank) {
            Ordering::Less => self.parent[xrepu] = yrep,
            Ordering::Greater => self.parent[yrepu] = xrep,
            Ordering::Equal => {
                self.parent[yrepu] = xrep;
                self.rank[xrepu] += 1;
            }
        }

        true
    }
}

/// Uses Kruskal's algorithm to calculate weight of graph minimum spanning tree
///
/// # Arguments
///
/// * 'graph' - connected graph that will be used to calculate weight of minimum spanning tree
pub fn calculate_min_total_weight(mut graph: Graph) -> i32 {
    let mut union_find = UnionFind::new(graph.nodes_count);

    graph.edges.sort_by(|a, b| a.weight.cmp(&b.weight));

    graph
        .edges
        .iter()
        .filter(|e| union_find.merge_parents(e.from_index, e.to_index))
        .map(|e| e.weight as i32)
        .sum()
}

pub fn old_calculate_min_total_weight(mut graph: Graph) -> i32 {
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

    // remove edges that connect nodes with the same parental node and map vector to weights
    // then calculate the weight by calling sum()
    graph.edges.iter().filter_map(|&e| {
        let from_parent = find_parent(&parents, e.from_index);
        let to_parent = find_parent(&parents, e.to_index);
        if from_parent != to_parent {
            parents[to_parent as usize] = from_parent;
            return Some(e.weight as i32);
        }
        None
    }).sum()
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
