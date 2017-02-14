extern crate rsh;

use std::env;

fn main() {
    let mut argv = env::args();

    // skip argv[0]
    argv.next();

    let s = if let Some(path) = argv.next() {
        rsh::State::new(path)
    } else {
        rsh::State::default()
    };

    rsh::run(s)
}
