use std::{
    env,
    fs::File,
    io::{stdin, stdout, BufReader},
};

extern crate rand;

mod interpreter;
mod pointer;
mod stack;
use interpreter::*;

fn main() {
    if let Some(file_name) = env::args().nth(1) {
        let file = File::open(file_name).unwrap();
        let source = BufReader::new(file);
        let stdin_buf = BufReader::new(stdin());
        let mut interpreter = Interpreter::new(source, stdin_buf, stdout());
        while !interpreter.program_ended() {
            interpreter.execute();
        }
    } else {
        println!("Plase specify source");
    }
}
