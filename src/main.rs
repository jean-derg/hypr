use std::path::Path;

mod tamper;

fn main() {
    tamper::protect(Path::new("./hypr"));
}
