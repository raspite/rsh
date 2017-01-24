pub mod builtins;
use std;

#[derive(PartialEq, Debug)]
pub struct State {
    cwd: std::Path,
}

pub fn run(state: State)
