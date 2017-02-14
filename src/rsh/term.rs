use std::str::{from_utf8, Utf8Error};

use std::fmt;
use std::io;
use std::io::Write;
use std::io::{Stdin, Stdout};

pub struct Term {
    pub stdout: Stdout,
    pub stdin: Stdin,
    pub buffer: String,
}

impl fmt::Debug for Term {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.buffer)
    }
}

impl Write for Term {
    fn write(&mut self, buf: &[u8]) -> Result<usize, io::Error> {
        self.buffer = from_utf8(buf).unwrap_or("").to_string();
        self.stdout.write(buf)
    }

    fn flush(&mut self) -> Result<(), io::Error> {
        self.stdout.flush()
    }
}

impl Term {
    pub fn default() -> Term {
        Term {
            stdout: io::stdout(),
            stdin: io::stdin(),
            buffer: String::new(),
        }
    }
}
