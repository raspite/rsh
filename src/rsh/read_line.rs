use std::io;
use std::io::{Write, Read};

use radix_trie::Trie;

use rsh::State;

pub struct Input {
    completions: Trie<String, String>,
}

impl Input {
    pub fn from(s: &State) -> Input {
        Input { completions: Trie::new() }
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
