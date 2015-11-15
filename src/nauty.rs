use nauty_bindings;
use nauty_bindings::{graph, optionblk};

use network::*;
use std::iter::FromIterator;

pub const MAXN: usize = 32;
pub const WORDSIZE: usize = 64;

fn default_options_graph() -> optionblk {
    optionblk {
        tc_level: 100,
        maxinvarlevel: 1,
        defaultptn: 1, // bool
        linelength: 78,
        dispatch: unsafe {&mut nauty_bindings::dispatch_graph},
        ..optionblk::default()
    }
}

pub fn default_options_digraph() -> optionblk {
    optionblk {
        digraph: 1, // bool
        maxinvarlevel: 999,
        .. default_options_graph()
    }
}

fn add_one_arc(g: &mut [graph], v: usize, w: usize) {
    g[v] |= bit(w);
}

// fn add_one_edge(g: &mut [graph], v: usize, w: usize) {
//     add_one_arc(g,v,w);
//     add_one_arc(g,w,v);
// }

fn bit(n: usize) -> nauty_bindings::setword {
    1 << (WORDSIZE - 1 - n)
}

pub fn canonical_labelling(net: &Network) -> Vec<NodeIndex> {
    let mut g = [0; MAXN];
    let mut cg = [0; MAXN];
    let mut lab = [0; MAXN];
    let mut ptn = [0; MAXN];
    let mut orbits = [0; MAXN];
    let mut options = default_options_digraph();
    let mut stats = nauty_bindings::statsblk::default();

    options.getcanon = 1;

    let n = net.node_count();
    assert!(n <= MAXN, "number of nodes greater than MAXN ({}): {}", MAXN, n);

    for e in net.raw_edges() {
        add_one_arc(&mut g, e.source().index(), e.target().index());
    }

    unsafe {
        nauty_bindings::densenauty(
            g.as_mut_ptr(),
            lab.as_mut_ptr(),
            ptn.as_mut_ptr(),
            orbits.as_mut_ptr(),
            &mut options,
            &mut stats,
            1, // m
            n as ::libc::c_int,
            cg.as_mut_ptr());
    }
    Vec::from_iter(lab[..n].iter().map(|idx| NodeIndex::new(*idx as usize)))
}

pub fn canonicalize(net: Network) -> Network {
    let lab = canonical_labelling(&net);
    net.subnet(&lab)
}

#[test]
fn test_canon() {
    use network::EdgeType::*;
    let mut net1 = Network::new();
    let a1 = net1.add_node("1".to_string());
    let b1 = net1.add_node("2".to_string());
    let c1 = net1.add_node("3".to_string());
    let d1 = net1.add_node("4".to_string());
    let e1 = net1.add_node("5".to_string());

    // The different order with change the internal numbering for net2.
    let mut net2 = Network::new();
    let a2 = net2.add_node("1".to_string());
    let d2 = net2.add_node("4".to_string());
    let c2 = net2.add_node("3".to_string());
    let b2 = net2.add_node("2".to_string());
    let e2 = net2.add_node("5".to_string());
    // We test using a feedforward involving a, b, c; and one with c, d, e. And e inhibiting a.
    net1.add_edge(a1, b1, Pos);
    net2.add_edge(a2, b2, Pos);
    net1.add_edge(b1, c1, Pos);
    net2.add_edge(b2, c2, Pos);
    net1.add_edge(a1, c1, Pos);
    net2.add_edge(a2, c2, Pos);

    net1.add_edge(c1, d1, Pos);
    net2.add_edge(c2, d2, Pos);
    net1.add_edge(d1, e1, Pos);
    net2.add_edge(d2, e2, Pos);
    net1.add_edge(c1, e1, Neg);
    net2.add_edge(c2, e2, Neg);

    net1.add_edge(e1, a1, Neg);
    net2.add_edge(e2, a2, Neg);

    let lab1 = canonical_labelling(&net1);
    let lab2 = canonical_labelling(&net2);

    for (l1, l2) in lab1.into_iter().zip(lab2.into_iter()) {
        assert_eq!(net1[l1], net2[l2]);
    }
}
