use std::{
    env,
    fs::File,
    io::{BufRead, BufReader},
    convert::TryInto,
};

extern crate rand;

use rand::Rng;

enum Direction {
    Up,
    Down,
    Left,
    Right,
}


fn main() {
    if let Some(file_name) = env::args().nth(1) {
        let mut source = BufReader::new(File::open(file_name).unwrap());
        let source_matrix: Vec<Vec<char>> = source.lines()
            .map(|line| line.unwrap().chars().collect()).collect();
        println!("{:?}", source_matrix);
        let mut pointer_direction = Direction::Right;
        let mut pointer = (0 as i32,0 as i32);
        let mut stack: Vec<i32> = Vec::new();
        loop {
            let command = source_matrix[pointer.0 as usize][pointer.1 as usize];
            match command {
                '1'|'2'|'3'|'4'|'5'|'6'|'7'|'8'|'9' => stack.push(command.to_digit(10).unwrap().try_into().unwrap()),
                '+' => {
                    let a = stack.pop().unwrap();
                    let b = stack.pop().unwrap();
                    stack.push(a + b);
                },
                '-' => {
                    let a = stack.pop().unwrap();
                    let b = stack.pop().unwrap();
                    stack.push(b - a);
                },
                '*' => {
                    let a = stack.pop().unwrap();
                    let b = stack.pop().unwrap();
                    stack.push(a * b);
                },
                '/' => {
                    let a = stack.pop().unwrap();
                    let b = stack.pop().unwrap();
                    stack.push(b / a);
                },
                '%' => {
                    let a = stack.pop().unwrap();
                    let b = stack.pop().unwrap();
                    stack.push(b % a);
                },
                '!' => {
                    let a = stack.pop().unwrap();
                    if a == 0 {
                        stack.push(1);
                    } else {
                        stack.push(0);
                    }
                },
                '`' => {
                    let a = stack.pop().unwrap();
                    let b = stack.pop().unwrap();
                    if b > a {
                        stack.push(1);
                    } else {
                        stack.push(0);
                    }
                },
                '>' => {
                    pointer_direction = Direction::Right;
                }  
                '<' => {
                    pointer_direction = Direction::Left;
                }
               '^' => {
                    pointer_direction = Direction::Up;
                }
               'V' => {
                    pointer_direction = Direction::Down;
                },
                '?' => {
                    let new_direction = rand::thread_rng().gen_range(0,4);
                    match new_direction {
                        0 => pointer_direction = Direction::Right,
                        1 => pointer_direction = Direction::Left,
                        2 => pointer_direction = Direction::Up,
                        3 => pointer_direction = Direction::Down,
                        _ => panic!("Generated number out of range"),
                    }
                },
                '_' => {
                    let condition = stack.pop().unwrap();
                    if condition == 0 {
                        pointer_direction = Direction::Right;
                    } else {
                        pointer_direction = Direction::Left;
                    }
                },
                '|' => {
                    let condition = stack.pop().unwrap();
                    if condition == 0 {
                        pointer_direction = Direction::Down;
                    } else {
                        pointer_direction = Direction::Up;
                    }
                },
                //'"' => {} TODO Handle string mode
                ':' => {
                    let value = stack.pop().unwrap();
                    stack.push(value);
                    stack.push(value);
                },
                '\\' => {
                    let a = stack.pop().unwrap();
                    let b = stack.pop().unwrap();
                    stack.push(a);
                    stack.push(b);
                },
                '$' => {
                    let _value = stack.pop();
                },
                '.' => {
                    let value = stack.pop().unwrap();
                    println!("{} ", value);
                },
                ',' => {
                    let value = stack.pop().unwrap();
                    let character = std::char::from_u32(value.try_into().unwrap());
                    println!("{}", character.unwrap());

                },
                '#' => {
                    pointer = increase_pointer(pointer, &pointer_direction);
                },
                _ => println!("{}",command)
            }
            pointer = increase_pointer(pointer, &pointer_direction);
        }
    } else {
        println!("Plase specify source");
    }
}

fn increase_pointer(pointer: (i32,i32), direction: &Direction) -> (i32,i32){
    match direction {
                Direction::Right => (pointer.0, (pointer.1 +1) % 80),
                Direction::Left => (pointer.0, 0.max(pointer.1 -1)),
                Direction::Up => (0.max(pointer.0 -1), pointer.1),
                Direction::Down => ((pointer.0 +1) % 25, pointer.1),
            }
}
