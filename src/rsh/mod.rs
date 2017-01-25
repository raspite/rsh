pub mod builtins;

use std::collections::HashMap;
use std::path::PathBuf;
use std::io;
use std::io::{Read, Write};

#[derive(PartialEq, Debug)]
pub struct State {
    cwd: PathBuf,
    builtins: HashMap<String, builtins::Builtin>,
    environment: HashMap<String, String>,
    aliases: HashMap<String, String>,
    argv: Vec<str>,
    argc: i32,
}

impl State {
    pub fn new(cwd: String) -> State {
        State{
            cwd: PathBuf::from(cwd),
            builtins: builtins::load(),
            environment: HashMap::new(),
            aliases: HashMap::new(),
            argv: Vec::new(),
            argc: 0,
        }
    }
}

pub fn run(mut s: State) {
    println!("Welcome to rsh! {:?}", s);

    loop {

        print!("{} -> ", s.cwd.to_str().unwrap());

        // this forces the prompt to print
        io::stdout().flush();

        // read the user input
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        s.argv = input.split_whitespace().collect();
        s.argc = s.argv.size();

        print!("\n");
        println!("Input: {}\nState: {:?}", input, s);
    }
}
