use rsh::State;
use std::path::PathBuf;
use std::collections::HashMap;
use std::slice::Iter;

pub type Builtin = fn(State) -> State;

pub fn load() -> HashMap<String, Builtin> {
    let mut h = HashMap::new();

    h.insert("cd".to_string(), cd as fn(State) -> State);
    // h.insert("ls".to_string(), ls as fn(State) -> State);

    h
}

fn cd(s: State) -> State {
    match s.argv.get(1) {
        Some(x) => {
            let mut newS = s.clone();

            newS.cwd = PathBuf::from(x);
            return newS
        },
        None => s.clone(),
    }
}

//  fn ls<'a>(s: State) -> State {
//      let dirs: Iter<String>;

//      if s.argv.len() == 0 {
//          let cwd = s.cwd.to_str().unwrap_or("").to_string();
//          let mut args: &'a Vec<String>;
//          args = Vec::new();
//          args.push(cwd);

//          dirs = args.iter();
//      } else {
//          let mut args = s.argv.clone().iter();
//          args.next();
//          dirs = args;
//      }

//      for d in dirs {
//          let mut p = PathBuf::from(d);

//          // Cheking if file so we don't do extra processing 
//          if p.is_file() {
//              println!("{}", p.to_str().unwrap_or(""));
//              continue;
//          }

//          // Unwrapping because we know it's a dir, not a file 
//          for entry in p.read_dir().unwrap() {
//              match entry {
//                  Ok(e) => {
//                      // TODO replace this unwrap to something safer
//                      println!("{}", e.file_name().into_string().unwrap());
//                  },
//                  Err(e) => println!("Error: {}", e),

//              }
//          }
//      }

//      s
// }








