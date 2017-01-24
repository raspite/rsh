pub mod rsh;

use std::env;

fn main() {
    let mut argv = env::args();
    
    let path = argv.
        next().
        unwrap_or(".".to_string());

    let s = rsh::State::new(path);
    rsh::run(s)
}
