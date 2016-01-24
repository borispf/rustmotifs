#![feature(alloc_system)]

extern crate alloc_system;
extern crate rustmotifs;

use rustmotifs::network::*;
use rustmotifs::motifs::*;

use std::io::prelude::*;
use std::iter::FromIterator;
use std::path::Path;
use std::str::FromStr;


fn arrowhead(w: EdgeType) -> &'static str {
    match w {
        1 => "onormal",
        3 => "diamond",
        _ => "circle",
    }
}

#[allow(unused_must_use)]
fn gen_dot(net: &Network) -> Vec<u8> {
    let mut graph = Vec::new();
    let n = net.node_count();
    {
        let g = &mut graph;
        writeln!(g, "digraph {{");
        for i in 0..n {
            let pi = std::f64::consts::PI;
            let t = 2. * pi * i as f64 / n as f64;
            writeln!(g, "  {} [pin=true,pos=\"{:.3},{:.3}\",shape=point]", i, t.sin(), t.cos());
        }
        for e in net.raw_edges() {
            writeln!(g, "  {} -> {} [arrowhead={}]", e.source().index(), e.target().index(), arrowhead(e.weight));
        }
        write!(g, "}}");
    }
    graph
}

fn is_interesting(motif: &Network) -> bool {
    let mut total_degree = 0;
    for n in 0..motif.node_count() {
        total_degree += std::collections::HashSet::<NodeIndex>::from_iter(motif.neighbors_undirected(NodeIndex::new(n))).len();
    }
    total_degree > 2 * (motif.node_count() - 1)
}

fn read_net<P: AsRef<Path>>(path: P) -> (Network, usize) {
    let mut file = std::fs::File::open(path).unwrap();
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
    (net, n)
}

fn main() {
    let args = Vec::from_iter(std::env::args());
    let (net, n) = read_net(&args[1]);
    println!("{}", n);
    let k = args.get(2).and_then(|k| usize::from_str(k).ok()).unwrap_or(3);
    let original_motifs = enumerate_subgraphs(k, &net);
    println!("{:?}", original_motifs);
    if args.len() <= 3 {
        for id in original_motifs.keys() {
            let dot = gen_dot(&id_to_network(k, *id));
            let mut file = std::fs::File::create(format!("graphs/{}.dot", id)).unwrap();
            file.write_all(dot.as_slice()).is_ok();
        }
        let mut motifs = Vec::from_iter(original_motifs.iter().map(|(id, count)| (*count, *id)));
        motifs.sort();
        motifs.reverse();
        {
            let mut path = Path::new("graphs").join(Path::new(&args[1]).file_stem().unwrap());
            path.set_extension("html");
            let mut html = std::fs::File::create(path).unwrap();
            write!(&mut html, "<html><body><table>").unwrap();
            for (count, id) in motifs {
                if is_interesting(&id_to_network(k, id)) {
                    write!(&mut html, r#"<tr><td><img src="{}.dot.png"></td><td>{}</td>"#, id, count).unwrap();
                }
            }
            write!(&mut html, "</table></body></html>").unwrap();
        }
    }

    print!("calculating ensemble motifs...");
    std::io::stdout().flush().unwrap();
    let mut ensemble_motifs = Vec::new();
    for (i, file) in args[3..].iter().enumerate() {
        if i % 10 == 0 {
            print!(" {}", i);
            std::io::stdout().flush().unwrap();
        }
        let (net, _) = read_net(file);
        ensemble_motifs.push(enumerate_subgraphs(k, &net));
    }
    println!(" done");
    print!("writing stats...");
    std::io::stdout().flush().unwrap();
    print_stats(&original_motifs, &ensemble_motifs).unwrap();
    println!("done");
}

fn print_stats(motifs: &MotifFreq, ensemble_motifs: &Vec<MotifFreq>) -> std::io::Result<()> {
    let mut stats = try!(std::fs::File::create("stats.csv"));
    try!(write!(&mut stats, "MotifId,Original"));
    for i in 0..ensemble_motifs.len() {
        try!(write!(&mut stats, ",R{}", i + 1));
    }
    try!(writeln!(&mut stats, ""));
    for (motif_id, freq) in motifs {
        try!(write!(&mut stats, "{},{}", *motif_id, *freq));
        for en in ensemble_motifs {
            try!(write!(&mut stats, ",{}", *en.get(motif_id).unwrap_or(&0)));
        }
        try!(writeln!(&mut stats, ""));
    }
    Ok(())
}
