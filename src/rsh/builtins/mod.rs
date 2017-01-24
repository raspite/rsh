use rsh::State;
use std::collection:HashMap;

pub type Builtin = fn(State) -> State;

pub fn load() -> HashMap<String, Builtin> {
    HashMap::new()
}
