use std::error::Error;

#[derive(Copy, Clone, Debug)]
pub struct Edge {
    from_index: i32,
    to_index: i32,
    weight: i32,
}

impl Edge {
    pub fn new(from_index: &str, to_index: &str, weight: &str) -> Result<Edge, Box<dyn Error>> {
        let parsed_from_index = from_index.parse::<i32>()?;
        let parsed_to_index = to_index.parse::<i32>()?;
        let parsed_weight = weight.parse::<i32>()?;

        return Ok(Edge::new_using_i32(parsed_from_index, parsed_to_index, parsed_weight));
    }

    pub fn new_using_i32(from_index: i32, to_index: i32, weight: i32) -> Edge {
        Edge {
            from_index,
            to_index,
            weight,
        }
    }
}

pub struct Graph {
    pub nodes_count: i32,
    pub edges: Vec<Edge>,
}

impl Graph {
    pub fn new(nodes_count: i32, edges: Vec<Edge>) -> Graph {
        Graph {
            nodes_count,
            edges,
        }
    }
}