use std::io;
use std::io::{Write, Read};
use std::ffi::OsStr;

use radix_trie::Trie;

use rsh::State;
use rsh::builtins;

pub struct Input {
    completions: Trie<String, String>,
}

impl Input {
    pub fn from(s: &State) -> Input {
        let mut i = Input { completions: Trie::new() };
        let mut strs: Vec<String> = Vec::new();

        // we don't have to worry about directories here anything in the
        // path should be an executable
        for path in s.exec_paths().iter() {
            let items = path.iter().collect::<Vec<&OsStr>>();
            let mut strings = items.iter()
                .map(|x| x.to_str().unwrap_or(""))
                .map(|x| x.to_string())
                .collect::<Vec<String>>();

            strs.append(&mut strings);
        }

        let builtins = builtins::load();
        let mut keys: Vec<String> = builtins.keys()
            .map(|x| x.clone())
            .collect();

        strs.append(&mut keys);

        for s in strs {
            let mut chars = s.chars();
            let mut current_char = chars.next();
            let mut next_char = chars.next();

            while current_char.is_some() && next_char.is_some() {
                i.completions.insert(current_char.unwrap().to_string(),
                                     next_char.unwrap().to_string());
                current_char = next_char.clone();
                next_char = chars.next();
            }
        }

        i
    }

    pub fn prompt(&self, prompt: String) -> String {
        print!("{}", prompt);

        // this forces the prompt to print
        io::stdout().flush().expect("unable to flush stdout");

        // read the user input
        let mut input = String::new();

        for byt in io::stdin().bytes() {
            match byt {
                Ok(b) => {
                    // TODO this is naive it ignores multi-byte utf-8 characters
                    let c = b as char;

                    input.push(c);

                    if c == '\n' {
                        break;
                    }
                }
                Err(e) => println!("Error reading from stdin: {}", e),
            };
        }

        input
    }
}
