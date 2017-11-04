extern crate cc;

fn main() {
    use cc::Build;
    Build::new()
        .file("src/imp.c")
        .compile("libc_stdhandle");
}
