use nauty::*;
use network::*;

pub use fixedbitset::FixedBitSet;
use std::collections::{BTreeMap, BTreeSet};
use std::iter::FromIterator;

pub type MotifId = u64;

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
        let edge_id = 3u64.pow(n * e.source().index() as u32 + e.target().index() as u32) + e.weight as u64;
        id += edge_id;
    }
    id
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

    let feedforward = motif_id(&canonical_subnet(&net, &[3, 12, 13]));
    assert_eq!(Some(&5), motifs.get(&feedforward));

    let line = motif_id(&canonical_subnet(&net, &[1, 2, 16]));
    assert_eq!(Some(&10), motifs.get(&line));

    let twofan = motif_id(&canonical_subnet(&net, &[5, 10, 13]));
    assert_eq!(Some(&3), motifs.get(&twofan));

    let vee = motif_id(&canonical_subnet(&net, &[3, 5, 13]));
    assert_eq!(Some(&3), motifs.get(&vee));

    assert_eq!(4, motifs.len());
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
    assert_eq!(0, motif_id(&canonical_subnet(&net, &[1])));
    assert_eq!(10, motif_id(&canonical_subnet(&net, &[1, 2])));
    assert_eq!(730, motif_id(&canonical_subnet(&net, &[1, 2, 3])));
    assert_eq!(531442, motif_id(&canonical_subnet(&net, &[1, 2, 3, 4])));
    assert_eq!(3486784402, motif_id(&canonical_subnet(&net, &[1, 2, 3, 4, 5])));
    assert_eq!(617955825820430, motif_id(&canonical_subnet(&net, &[1, 2, 3, 4, 5, 6])));
}

#[cfg(test)]
fn canonical_subnet(net: &Network, ns: &[usize]) -> Network {
    canonicalize(net.subnet(&Vec::from_iter(ns.iter().map(|n| NodeIndex::new(*n - 1)))))
}
