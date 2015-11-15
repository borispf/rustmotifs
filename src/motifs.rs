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
            for v in net.neighbors(u) {
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
fn test_motifs() {
    let net = network_from_paper();
    let motifs = all_motifs(3, &net);

    let feedforward = canonicalize(net.subnet(&[NodeIndex::new(2),
                                                NodeIndex::new(11),
                                                NodeIndex::new(12)]))
                          .adjacency_matrix();
    assert_eq!(Some(&5), motifs.get(&feedforward));

    let line = canonicalize(net.subnet(&[NodeIndex::new(0),
                                         NodeIndex::new(1),
                                         NodeIndex::new(15)]))
                   .adjacency_matrix();
    assert_eq!(Some(&10), motifs.get(&line));

    let twofan = canonicalize(net.subnet(&[NodeIndex::new(4),
                                           NodeIndex::new(9),
                                           NodeIndex::new(12)]))
                     .adjacency_matrix();
    assert_eq!(Some(&3), motifs.get(&twofan));


    assert_eq!(3, motifs.len());
}
