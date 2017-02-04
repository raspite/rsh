extern crate rsh;

use rsh::builtins::*;
use rsh::State;

#[test]
fn test_cd() {
    let mut s = State::default();
    s.argv = ["cd", "/"];

    let i = cd(&mut s);

    assert_eq!(i, 0);
    assert_eq!(s.cwd.to_string(), "/");
}

#[test]
#[should_panic]
fn test_cd_bad_path() {
    let mut s = State::default();
    s.argv = ["cd", "/123"];

    let i = cd(&mut s);

    assert_eq!(i, 0);
}

#[test]
fn test_set() {
    let mut s = State::default();
    s.argv = ["set", "test", "hello world"];

    let i = set(&mut s);

    assert_eq!(i, 0);
    assert_eq!(s.variables.)
}
