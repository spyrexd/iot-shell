use std::io::{stdin, stdout, Write};
use chrono::prelude::*;

fn main() -> Result<(), std::io::Error> {
    loop {
        print!("> ");
        stdout().flush()?;

        let mut input = String::new();
        stdin().read_line(&mut input)?;

        let mut parts = input.trim().split_ascii_whitespace();
        let command = parts.next().unwrap();
        let args = parts;

        match command {
            "date" => {
                let utc: DateTime<Utc> = Utc::now();
                println!("{}", utc.to_string())
            },
            "echo" => { println!("{}", args.collect::<Vec<_>>().join(" "))}
            "exit" => return Ok(()),
            _ => { println!("{} is not a valid command", command)}


        }
    }
}