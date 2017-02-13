use std::path::PathBuf;
use std::process::{Command, Stdio};

use rsh::State;

// TODO "Error handling mother trucker, do you speak it?"
pub fn exec(s: &State) -> i32 {
    let mut args = s.argv.iter();
    let mut exec_path: Option<PathBuf> = None;
    let exec_name = args.next().unwrap().as_str();

    'outer: for path in s.exec_paths().iter() {
        for entry in path.read_dir().unwrap() {
            if let Ok(e) = entry {
                if e.file_name() == *exec_name {
                    exec_path = Some(e.path().clone());
                    break 'outer;
                }
            }
        }
    }

    if exec_path.is_none() {
        println!("No such command: {}", exec_name);
        return 1000;
    }

    let command = Command::new(exec_path.unwrap().to_str().unwrap())
        .args(&args.collect::<Vec<&String>>())
        .current_dir(s.cwd.clone())
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn();

    if command.is_err() {
        println!("Error running {}: {}", exec_name, command.err().unwrap());
        return 1000;
    }

    match command.unwrap().wait() {
        Ok(exit) => if let Some(i) = exit.code() { i } else { 1000 },
        Err(e) => {
            println!("Error starting {}: {}", exec_name, e);
            1000
        }
    }
}
