pub mod builtins;
pub mod utils;
pub mod read_line;
pub mod exec;

use std::env;
use std::path::{Path, PathBuf};
use std::fmt;
use std::io;
use std::io::Write;
use std::collections::HashMap;
use std::collections::hash_map::Entry;

#[derive(Clone)]
pub struct State {
    cwd: PathBuf,
    aliases: HashMap<String, String>,
    argv: Vec<String>,
    argc: usize,
    exit_status: i32,
}

impl fmt::Debug for State {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,
               "State {{
\tcwd: {},
\taliases: {:?},
\targv: {:?}
\targc: {}
\texit_status: {}
}}
",
               self.cwd.display(),
               self.aliases,
               self.argv,
               self.argc,
               self.exit_status)
    }
}

impl State {
    pub fn new(cwd: String) -> State {
        State {
            cwd: utils::make_absolute(PathBuf::from(cwd)).unwrap(),
            aliases: HashMap::new(),
            argv: Vec::new(),
            argc: 0,
            exit_status: 0,
        }
    }

    pub fn default() -> State {
        match env::home_dir() {
            Some(cwd) => {
                State {
                    cwd: cwd,
                    aliases: HashMap::new(),
                    argv: Vec::new(),
                    argc: 0,
                    exit_status: 0,
                }
            }
            None => panic!("Unable to determine home directory."),
        }
    }


    #[cfg(any(unix))]
    pub fn exec_paths(&self) -> Vec<PathBuf> {
        let path = if let Ok(s) = env::var("PATH") {
            s.clone()
        } else {
            "".to_string()
        };

        path.split(":")
            .map(|x| Path::new(x).to_path_buf())
            .collect()
    }

    #[cfg(windows)]
    pub fn exec_paths(&self) -> Vec<PathBuf> {
        let path = if let Ok(s) = env::var("PATH") {
            s.clone()
        } else {
            "".to_string()
        };

        path.split(";")
            .map(|x| Path::new(x).to_path_buf())
            .collect()
    }
}

pub fn run(initial_state: State) {
    let mut s = initial_state.clone();
    let mut builtins = builtins::load();
    let i = read_line::Input::from(&s);

    println!("Welcome to rsh! {:?}", s);

    loop {

        let input = i.prompt(format!("{} -> ", s.cwd.display()));

        s.argv = parse_args(&input);
        s.argc = s.argv.len();

        print!("\n");
        println!("Input: {} {:?}", input, s);

        let first_arg = s.argv.get(0).unwrap().clone();
        if let Entry::Occupied(f) = builtins.entry(String::from(first_arg)) {
            let bn = f.get();
            s.exit_status = bn(&mut s);
            // prompt for input again.
            continue;
        }

        // else try to run the command
        exec::exec(&s);
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
    if args.len() == 0 {
        return Vec::new();
    }

    let mut parse_result: ParseResult = ParseResult {
        result: Vec::new(),
        completed: false,
        build_string: String::from(""),
        build_type: BuildType::None,
    };

    let mut argstr = args.clone();

    parse_string_into_vec(&argstr, &mut parse_result);

    while !parse_result.completed {
        // prompt for the rest of input
        print!(">");
        io::stdout().flush();
        argstr = String::from("");
        io::stdin().read_line(&mut argstr).unwrap();

        parse_string_into_vec(&argstr, &mut parse_result);
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
        if build_string.len() > 0 {
            result.push(build_string.clone());
        }
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
