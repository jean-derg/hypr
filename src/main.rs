use std::{path::Path, env::args};

mod tamper;

fn main() {
    tamper::protect(Path::new(&args().collect::<Vec<String>>()[1]));
}
