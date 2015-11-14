use petgraph;
use std::collections::HashSet;
use std::iter::FromIterator;

pub enum EdgeType {
    Pos,
    Neg,
}

pub type EdgeIndex = petgraph::graph::EdgeIndex;
pub type NodeIndex = petgraph::graph::NodeIndex;

pub type Network = petgraph::Graph<String, EdgeType>;

pub trait SubNetwork {
    fn subnet<N: AsRef<[NodeIndex]>>(&self, ns: N) -> Network;
}

impl SubNetwork for Network {
    fn subnet<N: AsRef<[NodeIndex]>>(&self, ns: N) -> Network {
        let ns = ns.as_ref();
        let n_set: HashSet<NodeIndex> = HashSet::from_iter(ns.iter().cloned());
        let subnet = Network::with_capacity(ns.len(), ns.len() * 2);
        // IMPLEMENTATION

        subnet
    }
}
