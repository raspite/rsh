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

#[derive(PartialEq)]
enum BuildType {
    None,
    Single,
    Double,
}

struct ParseResult {
    result: Vec<String>,
    completed: bool,
    build_string: String,
    build_type: BuildType,
}

fn parse_args(args: &String) -> Vec<String> {
    let mut parse_result: ParseResult = ParseResult {
        result: Vec::new(),
        completed: false,
        build_string: String::from(""),
        build_type: BuildType::None,
    };

    if args.len() == 0 {
        return parse_result.result;
    }

    let mut argstr = args.clone();
    loop {
        parse_string_into_vec(&argstr, &mut parse_result);

        if (parse_result.completed) {
            break;
        }

        match parse_result.build_type {
            BuildType::Single => print!("quote> "),
            BuildType::Double => print!("dquote> "),
            _ => {}
        }
        io::stdout().flush();
        argstr = String::from("");
        io::stdin().read_line(&mut argstr).unwrap();
    }

    let result: Vec<String> = parse_result.result
        .into_iter()
        .filter(|s| s.len() > 0)
        .collect();

    result
}

fn parse_string_into_vec(string: &String, parse_result: &mut ParseResult) {
    let &mut ParseResult { ref mut build_string,
                           ref mut build_type,
                           ref mut completed,
                           ref mut result } = parse_result;
    let mut iter = string.chars().peekable();

    while let Some(c) = iter.next() {
        match c {
            '\'' => {
                match *build_type {
                    BuildType::Single => {
                        *build_type = BuildType::None;
                        if iter.peek() == Some(&' ') {
                            result.push(build_string.clone());
                            *build_string = String::from("");
                        }
                    }

                    BuildType::None => {
                        *build_type = BuildType::Single;
                    }

                    _ => {
                        build_string.push(c);
                    }
                }
            }

            '\"' => {
                match *build_type {
                    BuildType::Double => {
                        *build_type = BuildType::None;
                        if iter.peek() == Some(&' ') {
                            result.push(build_string.clone());
                            *build_string = String::from("");
                        }
                    }

                    BuildType::None => {
                        *build_type = BuildType::Double;
                    }

                    _ => {
                        build_string.push(c);
                    }
                }
            }

            ' ' => {
                match *build_type {
                    BuildType::None => {
                        result.push(build_string.clone());
                        *build_string = String::from("");
                    }

                    _ => {
                        build_string.push(c);
                    }
                }
            }

            '\n' => {
                match *build_type {
                    BuildType::None => {
                        match iter.peek() {
                            Some(_) => {
                                build_string.push(c);
                            }
                            None => {}
                        }
                    }
                    _ => {
                        build_string.push(c);
                    }
                }
            }

            _ => {
                build_string.push(c);
            }
        }
    }

    if *build_type == BuildType::None {
        *completed = true;
        result.push(build_string.clone());
        *build_string = String::from("");
    }
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

    // parse single-word string inside quotes
    {
        let expected = vec!["echo".to_string()];
        let result = parse_args(&String::from("\"echo\""));
        assert_eq!(result, expected);
    }

    // parse multi-word string with closed quotes section
    {
        let expected = vec!["echo".to_string(), "-n".into(), "Hello Dear World".into()];
        let result = parse_args(&String::from("echo -n \"Hello Dear World\""));
        assert_eq!(result, expected);
    }

    // parse multi-word string with multiple closed quotes sections
    {
        let expected = vec!["echo".to_string(), "Hello".into(), "Dear World".into()];
        let result = parse_args(&String::from("echo \"Hello\" \"Dear World\""));
        assert_eq!(result, expected);
    }

    // parse multi-word string with no spaces around single quotes
    {
        let expected = vec!["echo".to_string(), "helloworld".into()];
        let result = parse_args(&String::from("echo 'hello'world"));
        assert_eq!(result, expected);
    }

    // parse multi-word string with no spaces around double quotes
    {
        let expected = vec!["echo".to_string(), "helloworld".into()];
        let result = parse_args(&String::from("echo \"hello\"world"));
        assert_eq!(result, expected);
    }

    // allow double quotes inside single quotes
    {
        let expected = vec!["\"".to_string()];
        let result = parse_args(&String::from("'\"'"));
        assert_eq!(result, expected);
    }

    // allow single quotes inside double quotes
    {
        let expected = vec!["\'".to_string()];
        let result = parse_args(&String::from("\"\'\""));
        assert_eq!(result, expected);
    }

    // handle multiple quote sections in succession
    {
        let expected = vec!["echo".to_string(), "hello world how are you".into()];
        let result = parse_args(&String::from("echo \"hello world\"\' how are you\'"));
        assert_eq!(result, expected);
    }

    // preserve newlines
    {
        let expected = vec!["echo".to_string(), "hello\nworld".into()];
        let result = parse_args(&String::from("echo \"hello\nworld\""));
        assert_eq!(result, expected);
    }

    // remove final newline
    {
        let expected = vec!["echo".to_string()];
        let result = parse_args(&String::from("echo\n"));
        assert_eq!(result, expected);
    }
}
