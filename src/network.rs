use petgraph;

pub enum EdgeType {
    Pos,
    Neg,
}

pub type EdgeIndex = petgraph::graph::EdgeIndex;
pub type NodeIndex = petgraph::graph::NodeIndex;

pub type Network = petgraph::Graph<String, EdgeType>;
