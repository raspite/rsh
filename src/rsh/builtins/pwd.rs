use rsh::State;

pub fn pwd(s: &mut State) -> i32 {
    println!("{}", s.cwd.display());

    0
}
