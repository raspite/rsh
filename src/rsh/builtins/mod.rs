use rsh::State;
use std::collections::HashMap;

pub type Builtin = fn(State) -> State;

pub fn load() -> HashMap<String, Builtin> {
    HashMap::new()
}

pub fn echo(s: State) -> State {
    if s.argv[1] == "-n" {
        let strings = &s.argv[2..s.argv.len()].join(" ");
        print!("{}", strings);
    } else {
        let strings = &s.argv[1..s.argv.len()].join(" ");
        println!("{}", strings);
    }

    s
}
