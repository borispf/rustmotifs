use petgraph;
use std::collections::HashMap;

pub type EdgeType = u8;
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
    let mut net = Network::new();
    let a = net.add_node("a".to_string());
    let b = net.add_node("b".to_string());
    let c = net.add_node("c".to_string());
    let d = net.add_node("d".to_string());
    let e = net.add_node("e".to_string());

    for u in &[a, b, c, d, e] {
        net.add_edge(*u, *u, 1);
        for v in &[a, b, c, d, e] {
            if u.index() < v.index() {
                net.add_edge(*u, *v, 1);
                net.add_edge(*v, *u, 2);
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

// The real network in figure 2 from R. Milo, et. al 2002; Network Motifs: Simple Building
// Blocks of Complex Networks. Has many feedforwards.
#[cfg(test)]
pub fn network_from_paper() -> Network {
    let mut net = Network::new();
    let a = net.add_node("1".to_string());
    let b = net.add_node("2".to_string());
    let c = net.add_node("3".to_string());
    let d = net.add_node("4".to_string());
    let e = net.add_node("5".to_string());
    let f = net.add_node("6".to_string());
    let g = net.add_node("7".to_string());
    let h = net.add_node("8".to_string());
    let i = net.add_node("9".to_string());
    let j = net.add_node("10".to_string());
    let k = net.add_node("11".to_string());
    let l = net.add_node("12".to_string());
    let m = net.add_node("13".to_string());
    let n = net.add_node("14".to_string());
    let o = net.add_node("15".to_string());
    let p = net.add_node("16".to_string());

    // 1
    net.add_edge(a, p, 1);

    // 2
    net.add_edge(b, a, 1);

    // 3
    net.add_edge(c, l, 1);
    net.add_edge(c, m, 1);

    // 4
    net.add_edge(d, j, 1);
    net.add_edge(d, k, 1);

    // 5
    net.add_edge(e, f, 1);
    net.add_edge(e, j, 1);
    net.add_edge(e, m, 1);

    // 6
    net.add_edge(f, i, 1);
    net.add_edge(f, j, 1);

    // 7
    net.add_edge(g, h, 1);

    // 8
    net.add_edge(h, a, 1);
    net.add_edge(h, b, 1);

    // 9 (none outgoing)
    // 10
    net.add_edge(j, k, 1);

    // 11 (none outgoing)
    // 12 (none outgoing)
    // 13
    net.add_edge(m, l, 1);

    // 14 (none outgoing)
    // 15
    net.add_edge(o, n, 1);

    // 16
    net.add_edge(p, n, 1);
    net.add_edge(p, o, 1);

    net
}
