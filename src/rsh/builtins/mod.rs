use std::path::PathBuf;
use std::collections::HashMap;

use rsh::State;
use rsh::utils;

pub type Builtin = fn(&mut State) -> i32;

pub fn load() -> HashMap<String, Builtin> {
    let mut h = HashMap::new();

    h.insert("cd".to_string(), cd as fn(&mut State) -> i32);
    h.insert("ls".to_string(), ls as fn(&mut State) -> i32);
    h.insert("echo".to_string(), echo as fn(&mut State) -> i32);
    h.insert("set".to_string(), set as fn(&mut State) -> i32);
    h.insert("unset".to_string(), unset as fn(&mut State) -> i32);
    h.insert("get".to_string(), get as fn(&mut State) -> i32);

    h
}

fn cd(s: &mut State) -> i32 {
    let mut new_path: PathBuf;

    match s.argv.get(1) {
        Some(x) => new_path = PathBuf::from(x),
        None => {
            new_path = PathBuf::from(s.variables
                .get("HOME")
                .unwrap_or(&"".to_string()))
        }
    };

    if !new_path.has_root() {
        match utils::make_absolute(new_path) {
            Ok(p) => new_path = p,
            Err(e) => {
                println!("cd: {}", e);
                return 1;
            }
        };
    }

    if !new_path.exists() {
        println!("cd: no such file or directory");
        return 1;
    }


    s.cwd = new_path;

    0
}

fn ls(s: &mut State) -> i32 {
    if s.argv.len() == 1 {
        list_dir(&s.cwd);
        return 0;
    }

    for d in s.argv.iter() {
        let p = PathBuf::from(d);
        list_dir(&p);
    }

    0
}

fn list_dir(p: &PathBuf) {
    // Cheking if file so we don't do extra processing
    if p.is_file() {
        println!("FILE: {}", p.to_str().unwrap_or("WTF"));
        return;
    }

    // Unwrapping because we know it's a dir, not a file
    for entry in p.read_dir().unwrap() {
        match entry {
            Ok(e) => {
                // TODO replace this unwrap to something safer
                print!("{} ", e.file_name().into_string().unwrap());
            }
            Err(e) => println!("Error: {}", e),
        }
    }

    print!("\n");
}

pub fn echo(s: &mut State) -> i32 {
    if s.argv[1] == "-n" {
        let strings = &s.argv[2..s.argv.len()].join(" ");
        print!("{}", strings);
    } else {
        let strings = &s.argv[1..s.argv.len()].join(" ");
        println!("{}", strings);
    }

    0
}

fn set(s: &mut State) -> i32 {

    let var_name = s.argv.get(1);
    let value = s.argv.get(2);

    if var_name.is_none() || value.is_none() {
        println!("set: not enough arguments to set");
        return 0;
    }

    let var = var_name.unwrap().clone();
    let val = value.unwrap().clone();

    s.variables.insert(var.to_string(), val.to_string());

    0
}

fn unset(s: &mut State) -> i32 {
    match s.argv.get(1) {
        Some(var) => {
            s.variables.remove(var);
            0
        }
        None => {
            println!("unset: not enough arguments");
            1
        }
    }
}

fn get(s: &mut State) -> i32 {
    match s.argv.get(1) {
        Some(var) => {
            s.variables
                .get(var)
                .map(|val| {
                    println!("{}", val);
                });

            0
        }
        None => {
            println!("get: not enough arguments");
            1
        }
    }
}


// TODO make these tests better once we can capture stdout
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_cd() {
        let mut s = State::default();
        s.argv = vec!["cd".to_string(), "/".to_string()];

        let i = cd(&mut s);

        assert_eq!(i, 0);
        assert_eq!(s.cwd.to_str().unwrap(), "/");
    }

    #[test]
    #[should_panic]
    fn test_cd_bad_path() {
        let mut s = State::default();
        s.argv = vec!["cd".to_string(), "/123".to_string()];

        let i = cd(&mut s);

        assert_eq!(i, 0);
    }

    #[test]
    fn test_set() {
        let mut s = State::default();
        s.argv = vec!["set".to_string(), "test".to_string(), "hello world".to_string()];

        let i = set(&mut s);

        assert_eq!(i, 0);
        assert_eq!(s.variables.get("test").unwrap(), "hello world");
    }

    #[test]
    fn test_unset() {
        let mut s = State::default();
        s.variables.insert("test".to_string(), "hello world".to_string());
        s.argv = vec!["unset".to_string(), "test".to_string()];

        let i = unset(&mut s);

        assert_eq!(i, 0);
        assert_eq!(s.variables.get("test"), None);
    }

    #[test]
    fn test_get() {
        let mut s = State::default();
        s.variables.insert("test".to_string(), "hello world".to_string());
        s.argv = vec!["get".to_string(), "test".to_string()];

        let i = get(&mut s);

        assert_eq!(i, 0);
    }

    #[test]
    fn test_ls() {
        let mut s = State::default();
        s.argv = vec!["ls".to_string()];

        let i = ls(&mut s);

        assert_eq!(i, 0);
    }

    #[test]
    fn test_echo() {
        let mut s = State::default();
        s.argv = vec!["echo".to_string(), "\"Hello world!\"".to_string()];

        let i = get(&mut s);

        assert_eq!(i, 0);
    }
}
