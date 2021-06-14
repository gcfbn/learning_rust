pub struct Edge {
    from_index: i32,
    to_index: i32,
    length: i32,
}

pub struct Graph {
    nodes: i32,
    edges: Vec<Edge>,
}