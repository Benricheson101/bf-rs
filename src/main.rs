mod interpreter;

use std::{env, fs};

fn main() {
    let path = env::args()
        .nth(1)
        .expect("no path given");

    let contents = fs::read_to_string(path).expect("Error opening file");

    interpreter::BF::new(contents).run();
}
