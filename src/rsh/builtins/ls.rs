use std::path::PathBuf;

use rsh::State;

pub fn ls(s: &mut State) -> i32 {
    if s.argv.len() == 1 {
        list_dir(&s.cwd);
        return 0;
    }

    for d in s.argv.iter() {
        let p = PathBuf::from(d);
        list_dir(&p);
    }

    0
}

fn list_dir(p: &PathBuf) {
    // Cheking if file so we don't do extra processing
    if p.is_file() {
        println!("FILE: {}", p.to_str().unwrap_or("WTF"));
        return;
    }

    // Unwrapping because we know it's a dir, not a file
    for entry in p.read_dir().unwrap() {
        match entry {
            Ok(e) => {
                // TODO replace this unwrap to something safer
                print!("{} ", e.file_name().into_string().unwrap());
            }
            Err(e) => println!("Error: {}", e),
        }
    }

    print!("\n");
}
