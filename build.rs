extern crate gcc;

fn main() {
    gcc::Config::new()
        .file("nauty/nauty.c")
        .file("nauty/nautil.c")
        .file("nauty/nausparse.c")
        .file("nauty/naugraph.c")
        .file("nauty/schreier.c")
        .file("nauty/naurng.c")
        .file("nauty/traces.c")
        .file("nauty/gtools.c")
        .file("nauty/naututil.c")
        .file("nauty/nautinv.c")
        .file("nauty/gutil1.c")
        .file("nauty/gutil2.c")
        .file("nauty/gtnauty.c")
        .file("nauty/naugroup.c")
        .define("WORDSIZE", Some("64"))
        .define("MAXN", Some("WORDSIZE"))
        .compile("libnautyL1.a");
}
