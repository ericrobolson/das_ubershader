mod data;
mod error;
mod interpreter;
mod ops;

pub use data::*;
pub use error::*;
pub use interpreter::*;
pub use ops::*;
use std::io::{self, BufRead};

fn main() -> Result<(), Error> {
    // TODO: implement REPL vs Loading program
    // For now just REPL.

    repl()?;

    Ok(())
}

enum InputBufferType {
    LoadedImage { image: String },
    NewColor { color: Color },
}
struct Color;
fn get_input_buffer_type() -> InputBufferType {}

fn read_line() -> String {
    let stdin = io::stdin();
    let mut line = String::new();
    stdin.lock().read_line(&mut line).unwrap();

    let trimmed = line.trim();
    println!("> {}", trimmed);

    trimmed.to_string()
}

fn get_choice<'a>(choices: &Vec<String>) -> String {
    loop {
        let input = read_line();
        if choices.contains(&input) {
            return input;
        } else {
            println!("Please choose one of {:?}", choices);
        }
    }
}

fn repl() -> Result<(), Error> {
    let stdin = io::stdin();
    let mut repl = true;
    let mut interpreter = Interpreter::new();

    while repl {
        let mut line = String::new();
        stdin.lock().read_line(&mut line).unwrap();

        let trimmed = line.trim();
        println!("> {}", trimmed);

        // Parse + execute all tokens
        for s in trimmed.split_whitespace() {
            if s == "qq" || s == "quit" {
                repl = false;
                break;
            }

            let op = match interpreter.parse(s) {
                Ok(op) => op,
                Err(e) => {
                    println!("ERR: {:?}", e);
                    break;
                }
            };

            match interpreter.execute(op) {
                Ok(_) => {}
                Err(e) => {
                    println!("ERR: {:?}", e);
                    break;
                }
            }
        }
    }

    Ok(())
}
