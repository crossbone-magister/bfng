use std::{
    convert::TryInto,
    env,
    fs::File,
    io::{stdin, stdout, BufRead, BufReader, Write},
};

extern crate rand;

mod pointer;
use pointer::*;
mod stack;
use rand::Rng;
use stack::*;
mod interpreter;

fn main() {
    if let Some(file_name) = env::args().nth(1) {
        let source = BufReader::new(File::open(file_name).unwrap());
        let mut source_matrix = vec![vec!(' '; 80); 25];
        for (x, line) in source.lines().enumerate() {
            for (y, command) in line.unwrap().chars().enumerate() {
                source_matrix[x][y] = command;
            }
        }
        println!("{:?}", source_matrix);
        let mut pointer = Pointer::default();
        // let mut pointer_direction = Direction::Right;
        // let mut pointer = (0 as i32, 0 as i32);
        let mut stack = Stack::default();
        let mut running = true;
        let mut string_mode = false;
        while running {
            let (x, y) = pointer.coordinates();
            let command = source_matrix[x as usize][y as usize];
            if string_mode {
                match command {
                    '"' => string_mode = false,
                    _ => stack.push_char(command),
                }
            } else {
                match command {
                    '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
                        stack.push(command.to_digit(10).unwrap().try_into().unwrap())
                    }
                    '+' => {
                        let a = stack.pop();
                        let b = stack.pop();
                        stack.push(a + b);
                    }
                    '-' => {
                        let a = stack.pop();
                        let b = stack.pop();
                        stack.push(b - a);
                    }
                    '*' => {
                        let a = stack.pop();
                        let b = stack.pop();
                        stack.push(a * b);
                    }
                    '/' => {
                        let a = stack.pop();
                        let b = stack.pop();
                        stack.push(b / a);
                    }
                    '%' => {
                        let a = stack.pop();
                        let b = stack.pop();
                        stack.push(b % a);
                    }
                    '!' => {
                        let a = stack.pop();
                        if a == 0 {
                            stack.push(1);
                        } else {
                            stack.push(0);
                        }
                    }
                    '`' => {
                        let a = stack.pop();
                        let b = stack.pop();
                        if b > a {
                            stack.push(1);
                        } else {
                            stack.push(0);
                        }
                    }
                    '>' => {
                        pointer.set_direction(Direction::Right);
                    }
                    '<' => {
                        pointer.set_direction(Direction::Left);
                    }
                    '^' => {
                        pointer.set_direction(Direction::Up);
                    }
                    'v' => {
                        pointer.set_direction(Direction::Down);
                    }
                    '?' => {
                        let new_direction = rand::thread_rng().gen_range(0, 4);
                        let pointer_direction;
                        match new_direction {
                            0 => pointer_direction = Direction::Right,
                            1 => pointer_direction = Direction::Left,
                            2 => pointer_direction = Direction::Up,
                            3 => pointer_direction = Direction::Down,
                            _ => panic!("Generated number out of range"),
                        }
                        pointer.set_direction(pointer_direction);
                    }
                    '_' => {
                        let condition = stack.pop();
                        let pointer_direction;
                        if condition == 0 {
                            pointer_direction = Direction::Right;
                        } else {
                            pointer_direction = Direction::Left;
                        }
                        pointer.set_direction(pointer_direction)
                    }
                    '|' => {
                        let condition = stack.pop();
                        let pointer_direction;
                        if condition == 0 {
                            pointer_direction = Direction::Down;
                        } else {
                            pointer_direction = Direction::Up;
                        }
                        pointer.set_direction(pointer_direction);
                    }
                    '"' => string_mode = true,
                    ':' => {
                        let value = stack.pop();
                        stack.push(value);
                        stack.push(value);
                    }
                    '\\' => {
                        let a = stack.pop();
                        let b = stack.pop();
                        stack.push(a);
                        stack.push(b);
                    }
                    '$' => {
                        let _value = stack.pop();
                    }
                    '.' => {
                        let value = stack.pop();
                        print!("{} ", value);
                        stdout().flush().unwrap();
                    }
                    ',' => {
                        let character = stack.pop_char();
                        print!("{}", character);
                        stdout().flush().unwrap();
                    }
                    '#' => {
                        pointer.increase();
                    }
                    'p' => {
                        let y = stack.pop() as usize;
                        let x = stack.pop() as usize;
                        let v = stack.pop() as u8;
                        std::mem::replace(&mut source_matrix[x][y], v.into());
                    }
                    'g' => {
                        let y = stack.pop() as usize;
                        let x = stack.pop() as usize;
                        if x < source_matrix.len() && y < source_matrix[x].len() {
                            let read_char = source_matrix[x][y];
                            let numeric_value = read_char as i32;
                            stack.push(numeric_value);
                        } else {
                            stack.push(0);
                        }
                    }
                    '&' => {
                        let mut line = String::new();
                        stdin().read_line(&mut line).unwrap();
                        line.pop();
                        let read_char = line.parse::<i32>().unwrap();
                        stack.push(read_char.try_into().unwrap());
                    }
                    '~' => {
                        let mut line = String::new();
                        stdin().read_line(&mut line).unwrap();
                        let read_char = line.chars().next().unwrap();
                        stack.push(read_char as i32);
                    }
                    '@' => {
                        running = false;
                    }
                    ' ' => (),
                    _ => println!("Unknown command '{}'", command),
                }
            }
            pointer.increase();
        }
    } else {
        println!("Plase specify source");
    }
}
