pub mod builtins;
pub mod utils;

use std::env;
use std::path::PathBuf;

use std::io;
use std::io::Write;

use std::collections::HashMap;
use std::collections::hash_map::Entry;

#[derive(Debug, Clone)]
pub struct State {
    cwd: PathBuf,
    variables: HashMap<String, String>,
    aliases: HashMap<String, String>,
    argv: Vec<String>,
    argc: usize,
    exit_status: i32,
}

impl State {
    pub fn new(cwd: String) -> State {
        State {
            cwd: utils::make_absolute(PathBuf::from(cwd)).unwrap(),
            variables: HashMap::new(),
            aliases: HashMap::new(),
            argv: Vec::new(),
            argc: 0,
            exit_status: 0,
        }
    }

    pub fn default() -> State {
        match env::current_dir() {
            Ok(cwd) => {
                State {
                    cwd: cwd,
                    variables: HashMap::new(),
                    aliases: HashMap::new(),
                    argv: Vec::new(),
                    argc: 0,
                    exit_status: 0,
                }
            }
            Err(e) => panic!(e),
        }
    }

    pub fn env<'a>(&'a mut self, key: String, value: String) -> &'a mut State {
        self.variables.insert(key, value);
        self
    }

    pub fn alias<'a>(&'a mut self, alias: String, value: String) -> &'a mut State {
        self.aliases.insert(alias, value);
        self
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
        io::stdout()
            .flush()
            .expect("unable to flush stdout");

        // read the user input
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("unable to read line from stdin");

        s.argv = parse_args(&input);
        s.argc = s.argv.len();

        print!("\n");
        println!("Input: {}\nState: {:?}", input, s);

        let first_arg = s.argv.get(0).unwrap().clone();
        if let Entry::Occupied(f) = builtins.entry(String::from(first_arg)) {
            let bn = f.get();
            s.exit_status = bn(&mut s);
            // prompt for input again.
            continue;
        }
    }
}

fn parse_args(args: &String) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();

    if args.len() == 0 {
        return result;
    }

    let mut building_string: bool = false;
    let mut build_string: String = String::from("");
    for string in args.split_whitespace() {
        if string.starts_with("\"") {
            // the string is surrounded by quotes - "word"
            if string.ends_with("\"") {
                build_string.push_str(string);
                result.push(build_string);

                building_string = false;
                build_string = String::from("");
                // the string only begins with quote - "word
            } else {
                building_string = true;

                build_string.push_str(string);
                build_string.push(' ');
            }
            // the string ends with quote - word"
        } else if string.ends_with("\"") {
            build_string.push_str(string);
            result.push(build_string);

            building_string = false;
            build_string = String::from("");
        } else {
            // the string is inside a quote section
            if building_string {
                build_string.push_str(string);
                build_string.push(' ');
                // the string is not inside a quote section
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
