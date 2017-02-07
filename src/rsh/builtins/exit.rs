use std::process;

use rsh::State;

pub fn exit(s: &mut State) -> i32 {
    match s.argv.get(1) {
        Some(x) => process::exit(x.parse::<i32>().expect("Not a valid exit code")),
        None => process::exit(0),
    };

    // ARE YOU HAPPY NOW RUSTC?!?!
    0
}
