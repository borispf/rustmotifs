use petgraph;
use std::collections::HashMap;

#[derive(Copy, Clone)]
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
        let mut subnet = Network::with_capacity(ns.len(), ns.len() * 2);
        let old_to_new: HashMap<NodeIndex, NodeIndex> =
            ns.iter().map(|n| (*n, subnet.add_node(self[*n].clone()))).collect();
        for u in ns {
            for (v, e) in self.edges(*u) {
                match (old_to_new.get(u), old_to_new.get(&v)) {
                    (Some(u), Some(v)) => {subnet.add_edge(*u, *v, *e);},
                    (Some(_), None) => {},
                    _ => {panic!("Huh!??!??!");},
                }
            }
        }
        subnet
    }
}
