use rsh::State;
use std::collections::HashMap;

pub type Builtin = fn(State) -> State;

pub fn load() -> HashMap<String, Builtin> {
    HashMap::new()
}
