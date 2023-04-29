extern crate cc;

fn main() {
    cc::Build::new()
        .file("src/callC/main.c")
        .compile("libmain.a");
}
