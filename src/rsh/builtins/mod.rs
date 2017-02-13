mod cd;
mod ls;
mod echo;
mod set;
mod pwd;
mod exit;

use std::collections::HashMap;

use rsh::State;

pub use self::cd::cd;
pub use self::ls::ls;
pub use self::echo::echo;
pub use self::set::{set, unset, get};
pub use self::pwd::pwd;
pub use self::exit::exit;

pub type Builtin = fn(&mut State) -> i32;

pub fn load() -> HashMap<String, Builtin> {
    let mut h: HashMap<String, Builtin> = HashMap::new();

    h.insert("cd".to_string(), cd);
    h.insert("ls".to_string(), ls);
    h.insert("echo".to_string(), echo);
    h.insert("set".to_string(), set);
    h.insert("unset".to_string(), unset);
    h.insert("get".to_string(), get);
    h.insert("pwd".to_string(), pwd);
    h.insert("exit".to_string(), exit);

    h
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

    #[test]
    fn test_pwd() {
        let mut s = State::default();

        s.argv = vec!["pwd".to_string()];

        let i = pwd(&mut s);

        assert_eq!(i, 0);
    }
}
