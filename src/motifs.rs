use nauty::*;
use network::*;

pub use fixedbitset::FixedBitSet;
use std::collections::{BTreeMap, BTreeSet};
use std::iter::FromIterator;

pub type MotifId = u64;
pub const MOTIF_BASE: u64 = 4;
pub type MotifFreq = BTreeMap<MotifId, usize>;

pub fn all_motifs(k: usize, net: &Network) -> BTreeMap<MotifId, usize> {
    let mut hash = BTreeSet::new();
    let mut out = BTreeMap::new();
    for e in net.raw_edges() {
        let mut s = BTreeSet::new();
        s.insert(e.source());
        s.insert(e.target());
        search_subset(k, net, &mut s, &mut hash, &mut out);
    }
    out
}

pub fn motif_id(motif: &Network) -> MotifId {
    let mut id = 0;
    let n = motif.node_count() as u32;
    for e in motif.raw_edges() {
        let edge_id = MOTIF_BASE.pow(n * e.source().index() as u32 + e.target().index() as u32) *
                      e.weight as u64;
        id += edge_id;
    }
    id
}

pub fn id_to_network(n: usize, mut id: MotifId) -> Network {
    let mut net = Network::with_capacity(n, 0);
    for _ in 0..n {
        net.add_node(String::new());
    }
    for i in (0..n).map(NodeIndex::new) {
        for j in (0..n).map(NodeIndex::new) {
            let e = id % MOTIF_BASE;
            id /= MOTIF_BASE;
            if e != 0 {
                net.add_edge(i, j, e as EdgeType);
            }
        }
    }
    net
}

fn search_subset(k: usize,
                 net: &Network,
                 s: &mut BTreeSet<NodeIndex>,
                 hash: &mut BTreeSet<BTreeSet<NodeIndex>>,
                 out: &mut BTreeMap<MotifId, usize>) {
    if s.len() == k && !hash.contains(s) {
        hash.insert(s.clone());
        let motif = canonicalize(net.subnet(&Vec::from_iter(s.iter().cloned())));
        let motif_id = motif_id(&motif);
        *out.entry(motif_id).or_insert(0) += 1;
    } else if s.len() < k {
        hash.insert(s.clone());
        for u in s.clone() {
            for v in net.neighbors_undirected(u) {
                if !s.contains(&v) {
                    s.insert(v);
                    if !hash.contains(s) {
                        search_subset(k, net, s, hash, out);
                    }
                    s.remove(&v);
                }
            }
        }
    }
}

pub fn enumerate_subgraphs(k: usize, net: &Network) -> BTreeMap<MotifId, usize> {
    let mut out = BTreeMap::new();
    let n = net.node_count();
    for v in (0..n).map(NodeIndex::new) {
        let v_subgraph = vec![v].into_iter().collect();
        let v_subgraph_neighbours = net.neighbors_undirected(v).collect();
        let v_extension = net.neighbors_undirected(v).filter(|u| *u > v).collect();
        extend_subgraph(k, net, v, v_subgraph, v_subgraph_neighbours, v_extension, &mut out);
    }
    out
}

pub fn extend_subgraph(k: usize,
                       net: &Network,
                       v: NodeIndex,
                       v_subgraph: BTreeSet<NodeIndex>,
                       v_subgraph_neighbours: BTreeSet<NodeIndex>,
                       mut v_extension: BTreeSet<NodeIndex>,
                       out: &mut BTreeMap<MotifId, usize>) {
    if v_subgraph.len() == k {
        let motif = canonicalize(net.subnet(Vec::from_iter(v_subgraph)));
        *out.entry(motif_id(&motif)).or_insert(0) += 1;
    } else {
        while let Some(w) = {
            let maybe_w = v_extension.iter().cloned().next();
            maybe_w.map(|w| v_extension.remove(&w));
            maybe_w
        } {
            let w_neighbours: BTreeSet<_> = net.neighbors_undirected(w).filter(|u| u > &v).collect();
            let v_extension_prime = &v_extension | &(&w_neighbours - &v_subgraph_neighbours);
            extend_subgraph(
                k,
                net,
                v,
                &v_subgraph | &BTreeSet::from_iter(vec![w]),
                &v_subgraph_neighbours | &BTreeSet::from_iter(net.neighbors_undirected(w)),
                v_extension_prime,
                out
            )
        }
    }
}

#[test]
fn test_shared_ff() {
    let mut net = Network::new();
    let source = net.add_node("source".to_string());
    let sink = net.add_node("sink".to_string());
    net.add_edge(source, sink, 1);
    let n = 10;
    for i in 0..n {
        let node = net.add_node(format!("{}", i));
        net.add_edge(source, node, 1);
        net.add_edge(node, sink, 1);
    }
    assert_eq!(Some(&n), all_motifs(3, &net).get(&motif_str("011001000")));
}

#[test]
fn test_motifs_3() {
    let net = network_from_paper();
    let motifs = all_motifs(3, &net);

    let feedforward = motif_id(&canonical_subnet(&net, &[3, 12, 13]));
    assert_eq!(Some(&5), motifs.get(&feedforward));

    let line = motif_id(&canonical_subnet(&net, &[1, 2, 16]));
    assert_eq!(Some(&10), motifs.get(&line));

    let twofan = motif_id(&canonical_subnet(&net, &[5, 10, 13]));
    assert_eq!(Some(&3), motifs.get(&twofan));

    let vee = motif_id(&canonical_subnet(&net, &[3, 5, 13]));
    assert_eq!(Some(&3), motifs.get(&vee));

    assert_eq!(4, motifs.len());

    assert_eq!(enumerate_subgraphs(3, &net), motifs);
}

#[test]
fn test_motifs_4() {
    let net = network_from_paper();
    let motifs = all_motifs(4, &net);

    let line = motif_id(&canonical_subnet(&net, &[1, 2, 15, 16]));
    assert_eq!(Some(&5), motifs.get(&line));

    let feedforwardin = motif_id(&canonical_subnet(&net, &[1, 14, 15, 16]));
    assert_eq!(Some(&2), motifs.get(&feedforwardin));

    let feedforwardout = motif_id(&canonical_subnet(&net, &[1, 2, 8, 16]));
    assert_eq!(Some(&2), motifs.get(&feedforwardout));

    let feedforwardsidein = motif_id(&canonical_subnet(&net, &[3, 5, 12, 13]));
    assert_eq!(Some(&3), motifs.get(&feedforwardsidein));

    let feedforwardsideout = motif_id(&canonical_subnet(&net, &[5, 6, 9, 10]));
    assert_eq!(Some(&1), motifs.get(&feedforwardsideout));

    let branch = motif_id(&canonical_subnet(&net, &[5, 6, 9, 13]));
    assert_eq!(Some(&5), motifs.get(&branch));

    let n = motif_id(&canonical_subnet(&net, &[13, 10, 3, 5]));
    assert_eq!(Some(&4), motifs.get(&n));

    let feedforwardendin = motif_id(&canonical_subnet(&net, &[4, 5, 6, 10]));
    assert_eq!(Some(&1), motifs.get(&feedforwardendin));

    let branch_feedforward = motif_id(&canonical_subnet(&net, &[5, 6, 10, 13]));
    assert_eq!(Some(&1), motifs.get(&branch_feedforward));

    assert_eq!(9, motifs.len());
    assert_eq!(enumerate_subgraphs(4, &net), motifs);
}

#[test]
fn test_motif_id() {
    {
        let mut net = Network::new();
        assert_eq!(0, motif_id(&net));
        net.add_node(String::new());
        assert_eq!(0, motif_id(&net));
        net.add_node(String::new());
        assert_eq!(0, motif_id(&net));
        net.add_node(String::new());
        assert_eq!(0, motif_id(&net));
    }

    let net = network_from_paper();
    println!("{:?}", canonical_subnet(&net, &[1, 2, 3, 4, 5, 6]));
    assert_eq!(0, motif_id(&canonical_subnet(&net, &[1])));
    assert_eq!(motif_str("0100"), motif_id(&canonical_subnet(&net, &[1, 2])));
    assert_eq!(motif_str("001000000"), motif_id(&canonical_subnet(&net, &[1, 2, 3])));
    assert_eq!(motif_str("0001000000000000"), motif_id(&canonical_subnet(&net, &[1, 2, 3, 4])));
    assert_eq!(motif_str("0000100000000000000000000"),
               motif_id(&canonical_subnet(&net, &[1, 2, 3, 4, 5])));
    assert_eq!(motif_str("000010000001000000000000000000000000"),
               motif_id(&canonical_subnet(&net, &[1, 2, 3, 4, 5, 6])));
}

#[test]
fn test_motif_id_roundtrip() {
    for id in 0..128 {
        assert_eq!(motif_id(&id_to_network(5, id)), id);
    }
}

#[cfg(test)]
fn canonical_subnet(net: &Network, ns: &[usize]) -> Network {
    canonicalize(net.subnet(&Vec::from_iter(ns.iter().map(|n| NodeIndex::new(*n - 1)))))
}

#[cfg(test)]
fn motif_str(s: &str) -> MotifId {
    MotifId::from_str_radix(s, MOTIF_BASE as u32).unwrap()
}
