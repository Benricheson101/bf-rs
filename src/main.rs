mod interpreter;
use interpreter::BF;
use std::{env, fs};

fn main() {
    let path = env::args()
        .nth(1)
        .expect("no path given");

    let contents = fs::read_to_string(path).expect("Error opening file");

    BF::new(contents).run();
}
