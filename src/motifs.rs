use nauty::*;
use network::*;

// use arrayvec::ArrayVec;
pub use fixedbitset::FixedBitSet;
use petgraph::visit::GetAdjacencyMatrix;
use std::collections::{BTreeMap, BTreeSet};
use std::iter::FromIterator;

// const MAX_N: usize = 4;
// type SmallVec<T> = ArrayVec<[T; MAX_N]>;

pub fn all_motifs(k: usize, net: &Network) -> BTreeMap<FixedBitSet, usize> {
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

fn search_subset(k: usize,
                 net: &Network,
                 s: &mut BTreeSet<NodeIndex>,
                 hash: &mut BTreeSet<BTreeSet<NodeIndex>>,
                 out: &mut BTreeMap<FixedBitSet, usize>) {
    if s.len() == k && !hash.contains(s) {
        hash.insert(s.clone());
        let motif = canonicalize(net.subnet(&Vec::from_iter(s.iter().cloned())));
        let adj = motif.adjacency_matrix();
        *out.entry(adj).or_insert(0) += 1;
    } else {
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

#[test]
fn test_motifs_3() {
    let net = network_from_paper();
    let motifs = all_motifs(3, &net);

    let feedforward = canonical_subnet(&net, &[3, 12, 13]).adjacency_matrix();
    assert_eq!(Some(&5), motifs.get(&feedforward));

    let line = canonical_subnet(&net, &[1, 2, 16]).adjacency_matrix();
    assert_eq!(Some(&10), motifs.get(&line));

    let twofan = canonical_subnet(&net, &[5, 10, 13]).adjacency_matrix();
    assert_eq!(Some(&3), motifs.get(&twofan));

    let vee = canonical_subnet(&net, &[6, 9, 10]).adjacency_matrix();
    assert_eq!(Some(&3), motifs.get(&vee));

    assert_eq!(4, motifs.len());
}

#[test]
fn test_motifs_4() {
    let net = network_from_paper();
    let motifs = all_motifs(4, &net);

    let line = canonical_subnet(&net, &[1, 2, 15, 16]).adjacency_matrix();
    assert_eq!(Some(&5), motifs.get(&line));

    let feedforwardin = canonical_subnet(&net, &[1, 14, 15, 16]).adjacency_matrix();
    assert_eq!(Some(&2), motifs.get(&feedforwardin));

    let feedforwardout = canonical_subnet(&net, &[1, 2, 8, 16]).adjacency_matrix();
    assert_eq!(Some(&2), motifs.get(&feedforwardout));

    let feedforwardsidein = canonical_subnet(&net, &[3, 5, 12, 13]).adjacency_matrix();
    assert_eq!(Some(&3), motifs.get(&feedforwardsidein));

    let feedforwardsideout = canonical_subnet(&net, &[5, 6, 9, 10]).adjacency_matrix();
    assert_eq!(Some(&1), motifs.get(&feedforwardsideout));

    let branch = canonical_subnet(&net, &[5, 6, 9, 13]).adjacency_matrix();
    assert_eq!(Some(&5), motifs.get(&branch));

    let n = canonical_subnet(&net, &[13, 10, 3, 5]).adjacency_matrix();
    assert_eq!(Some(&4), motifs.get(&n));

    let feedforwardendin = canonical_subnet(&net, &[4, 5, 6, 10]).adjacency_matrix();
    assert_eq!(Some(&1), motifs.get(&feedforwardendin));

    let branch_feedforward = canonical_subnet(&net, &[5, 6, 10, 13]).adjacency_matrix();
    assert_eq!(Some(&1), motifs.get(&branch_feedforward));

    println!("{:?}", motifs);

    assert_eq!(9, motifs.len());
}

#[cfg(test)]
fn canonical_subnet(net: &Network, ns: &[usize]) -> Network {
    canonicalize(net.subnet(&Vec::from_iter(ns.iter().map(|n| NodeIndex::new(*n - 1)))))
}
