pub mod builtins;

use std::collections::HashMap;
use std::path::PathBuf;
use std::io;
use std::io::Read;

#[derive(PartialEq, Debug)]
pub struct State {
    cwd: PathBuf,
    builtins: HashMap<String, builtins::Builtin>,
    environment: HashMap<String, String>,
    aliases: HashMap<String, String>,
}

impl State {
    pub fn new(cwd: String) -> State {
        State{
            cwd: PathBuf::from(cwd),
            builtins: builtins::load(),
            environment: HashMap::new(),
            aliases: HashMap::new(),
        }
    }
}

pub fn run(state: State) {
    println!("Welcome to rsh! {:?}", state);

    loop {

        print!("{} -> ", state.cwd.to_str().unwrap());

        loop {
            match io::stdin().bytes().next().unwrap().unwrap() as char {
                '\n' => break,
                x => println!("{}", x),
            }
        }

        print!("\n");

        println!("State: {:?}", state);
    }
}
