pub mod builtins;

use std::path::PathBuf;
use std::io;

#[derive(PartialEq, Debug)]
pub struct State {
    cwd: PathBuf,
    builtins: Vec<builtins::Builtin>,
}

impl State {
    pub fn new(cwd: String) -> State {
        State{
            cwd: PathBuf::from(cwd),
            builtins: builtins::load(),
        }
    }
}

pub fn run(state: State) {
    loop {
        print!("\n{} -> ", state.cwd.to_str().unwrap_or(""));
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(n) => {
                println!("{:?}", state);
                println!("{}", input);
            },
            Err(e) => println!("error: {}", e),
        };
    }
}
