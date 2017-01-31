pub mod builtins;

use std::collections::HashMap;
use std::path::PathBuf;
use std::io;
use std::io::{Read, Write};
use std::collections::hash_map::Entry;

#[derive(PartialEq, Debug, Clone)]
pub struct State {
    cwd: PathBuf,
    environment: HashMap<String, String>,
    aliases: HashMap<String, String>,
    argv: Vec<String>,
    argc: usize,
}

impl State {
    pub fn new(cwd: String) -> State {
        State {
            cwd: PathBuf::from(cwd),
            environment: HashMap::new(),
            aliases: HashMap::new(),
            argv: Vec::new(),
            argc: 0,
        }
    }
}

pub fn run(initial_state: State) {
    let mut builtins = builtins::load();
    let mut s = initial_state.clone();

    println!("Welcome to rsh! {:?}", s);

    loop {

        print!("\n");
        print!("{} -> ", s.cwd.to_str().unwrap());

        // this forces the prompt to print
        io::stdout().flush();

        // read the user input
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        s.argv = parse_args(&input);
        s.argc = s.argv.len();

        print!("\n");
        println!("Input: {}\nState: {:?}", input, s);

        let first_arg = s.argv.get(0).unwrap().clone();
        if let Entry::Occupied(f) = builtins.entry(String::from(first_arg)) {
            let bn = f.get();
            s = bn(s.clone());
        }
    }
}

fn parse_args(args: &String) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();

    if args.len() == 0 {
        return result
    }

    let mut building_string: bool = false;
    let mut build_string: String = String::from("");
    for string in args.split_whitespace() {
        if string.starts_with("\"") {
            if string.ends_with("\"") {
                build_string.push_str(string);
                result.push(build_string);

                building_string = false;
                build_string = String::from("");
            } else {
                building_string = true;

                build_string.push_str(string);
                build_string.push(' ');
            }
        } else if string.ends_with("\"") {
            build_string.push_str(string);
            result.push(build_string);

            building_string = false;
            build_string = String::from("");
        } else {
            if building_string {
                build_string.push_str(string);
                build_string.push(' ');
            } else {
                result.push(string.to_string());
            }
        }
    }

    result
}

#[test]
fn parse_args_test() {
    // parse empty string
    {
        let expected: Vec<String> = Vec::new();
        let result = parse_args(&String::from(""));
        assert_eq!(result, expected);
    }

    // parse single-word string
    {
        let expected: Vec<String> = vec!["echo".to_string()];
        let result = parse_args(&String::from("echo"));
        assert_eq!(result, expected);
    }

    // parse single-word string inside parens
    {
        let expected = vec!["\"echo\"".to_string()];
        let result = parse_args(&String::from("\"echo\""));
        assert_eq!(result, expected);
    }

    // parse multi-word string with closed parens section
    {
        let expected = vec!["echo".to_string(), "-n".into(), "\"Hello Dear World\"".into()];
        let result = parse_args(&String::from("echo -n \"Hello Dear World\""));
        assert_eq!(result, expected);
    }

    // parse multi-word string with multiple closed parents sections
    {
        let expected = vec!["echo".to_string(), "\"Hello\"".into(), "\"Dear World\"".into()];
        let result = parse_args(&String::from("echo \"Hello\" \"Dear World\""));
        assert_eq!(result, expected);
    }
}
