extern crate gcc;

fn main() {
    gcc::Config::new()
        .file("nauty/nauty.c")
        .file("nauty/nautil.c")
        .file("nauty/naugraph.c")
        .file("nauty/schreier.c")
        .file("nauty/naurng.c")
        .define("WORDSIZE", Some("64"))
        .define("MAXN", Some("32"))
        .compile("libnautyL1.a");
}
