use std::env;

use rsh::State;

pub fn set(s: &mut State) -> i32 {

    let var_name = s.argv.get(1);
    let value = s.argv.get(2);

    if var_name.is_none() || value.is_none() {
        println!("set: not enough arguments to set");
        return 0;
    }

    let var = var_name.unwrap().clone();
    let val = value.unwrap().clone();

    env::set_var(var, val);

    0
}

pub fn unset(s: &mut State) -> i32 {
    match s.argv.get(1) {
        Some(var) => {
            env::remove_var(var);
            0
        }
        None => {
            println!("unset: not enough arguments");
            1
        }
    }
}

pub fn get(s: &mut State) -> i32 {
    match s.argv.get(1) {
        Some(var) => {
            if let Ok(val) = env::var(var) {
                println!("{}", val);
            }

            0
        }
        None => {
            println!("get: not enough arguments");
            1
        }
    }
}
