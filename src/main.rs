extern crate bruteforce;

use bruteforce::network::*;
use bruteforce::motifs::*;

use std::io::prelude::*;
use std::iter::FromIterator;
use std::str::FromStr;

fn main() {
    let args = Vec::from_iter(std::env::args());
    let mut file = std::fs::File::open(&args[1]).unwrap();
    let mut s = String::new();
    file.read_to_string(&mut s).unwrap();
    let adj_mat = Vec::from_iter(s.split_whitespace().map(|f| match f {
        "1" => 1,
        "2" => 2,
        "-1" => 3,
        "0" => 0,
        s => panic!("{:?}", s),
    }));
    let n = (adj_mat.len() as f64 + 1.0).sqrt() as usize;
    let mut net = Network::with_capacity(n, adj_mat.iter().filter(|v| **v != 0).count());
    for i in 0..n {
        net.add_node(format!("{}", i + 1));
    }
    for i in (0..n).map(NodeIndex::new) {
        for j in (0..n).map(NodeIndex::new) {
            let w = adj_mat[n * i.index() + j.index()];
            if i != j && w != 0 {
                net.add_edge(i, j, w);
            }
        }
    }
    println!("{}", n);
    let k = args.get(2).and_then(|k| usize::from_str(k).ok()).unwrap_or(3);
    println!("{:?}", all_motifs(k, &net));
}
