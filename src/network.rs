use petgraph;
use std::collections::HashMap;

#[derive(Copy, Clone, Debug)]
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

#[test]
fn test_network() {
    use self::EdgeType::*;
    let mut net = Network::new();
    let a = net.add_node("a".to_string());
    let b = net.add_node("b".to_string());
    let c = net.add_node("c".to_string());
    let d = net.add_node("d".to_string());
    let e = net.add_node("e".to_string());

    for u in &[a, b, c, d, e] {
        net.add_edge(*u, *u, Pos);
        for v in &[a, b, c, d, e] {
            if u.index() < v.index() {
                net.add_edge(*u, *v, Pos);
                net.add_edge(*v, *u, Neg);
            }
        }
    }
    let sub = net.subnet(&[]);
    assert_eq!(0, sub.node_count());
    assert_eq!(0, sub.edge_count());

    let sub = net.subnet(&[b]);
    assert_eq!(1, sub.node_count());
    assert_eq!(1, sub.edge_count());

    let sub = net.subnet(&[a, c, e]);
    assert_eq!(3, sub.node_count());
    assert_eq!(9, sub.edge_count());

    let sub = net.subnet(&[a, b, c, e]);
    assert_eq!(4, sub.node_count());
    assert_eq!(16, sub.edge_count());

    let sub = net.subnet(&[e, d, c, b, a]);
    assert_eq!(5, sub.node_count());
    assert_eq!(25, sub.edge_count());

}
