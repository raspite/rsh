use rsh::State;

pub type Builtin = fn(State) -> State;

pub fn load() -> Vec<Builtin> {
    Vec::new()
}
