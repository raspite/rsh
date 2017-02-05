use std::path::PathBuf;
use std::fs::canonicalize;
use std::io;

pub fn make_absolute(p: PathBuf) -> Result<PathBuf, io::Error> {
    canonicalize(p)
}
