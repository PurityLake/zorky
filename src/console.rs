use std::io::{self, Write, Error};

pub struct Console {
    size: usize,
    pub last_result: String,
    prompt: String
}

fn read_line() -> Result<(usize, String), Error> {
    let mut buffer = String::with_capacity(32);
    let stdin = io::stdin();
    match stdin.read_line(&mut buffer) {
        Ok(size) => Ok((size, buffer)),
        Err(e) => {
            println!("Failed to read from stdin");
            Err(e)
        }
    }
}

impl Console {
    pub fn new(prompt: String) -> Console {
        Console{
            size: 0,
            last_result: String::with_capacity(32),
            prompt: prompt
        }
    }

    pub fn read(&mut self) -> Result<(), Error> {
        print!("{}", self.prompt);
        io::stdout().flush().unwrap();
        match read_line() {
            Ok((size, buffer)) => {
                self.size = size;
                self.last_result = String::from(buffer.trim());
                Ok(())
            },
            Err(e) => Err(e)
        }
    }

    pub fn match_any(&mut self, values: &[String]) -> bool {
        for v in values.iter() {
            if *v == self.last_result {
                return true;
            }
        }
        false
    }

    pub fn print_result(&self) {
        print!("{}", self.last_result);
        io::stdout().flush().unwrap();
    }

    pub fn println_result(&self) {
        println!("{}", self.last_result);
    }
}