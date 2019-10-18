use std::{
    env,
    fs::File,
    io::{BufRead, BufReader,stdin,stdout,Write},
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
        let source = BufReader::new(File::open(file_name).unwrap());
        let mut source_matrix: Vec<Vec<char>> = source.lines()
            .map(|line| line.unwrap().chars().collect()).collect();
        println!("{:?}", source_matrix);
        let mut pointer_direction = Direction::Right;
        let mut pointer = (0 as i32,0 as i32);
        let mut stack: Vec<i32> = Vec::new();
        let mut running = true;
        let mut string_mode = false;
        while running {
            let command = source_matrix[pointer.0 as usize][pointer.1 as usize];
            if string_mode {
                match command {
                    '"' => string_mode = false,
                    _ => stack.push(command as i32)

                }
            } else {
                match command {
                    '0'|'1'|'2'|'3'|'4'|'5'|'6'|'7'|'8'|'9' => stack.push(command.to_digit(10).unwrap().try_into().unwrap()),
                    '+' => {
                        let a = pop_stack_value(&mut stack);
                        let b = pop_stack_value(&mut stack);
                        stack.push(a + b);
                    },
                    '-' => {
                        let a = pop_stack_value(&mut stack);
                        let b = pop_stack_value(&mut stack);
                        stack.push(b - a);
                    },
                    '*' => {
                        let a = pop_stack_value(&mut stack);
                        let b = pop_stack_value(&mut stack);
                        stack.push(a * b);
                    },
                    '/' => {
                        let a = pop_stack_value(&mut stack);
                        let b = pop_stack_value(&mut stack);
                        stack.push(b / a);
                    },
                    '%' => {
                        let a = pop_stack_value(&mut stack);
                        let b = pop_stack_value(&mut stack);
                        stack.push(b % a);
                    },
                    '!' => {
                        let a = pop_stack_value(&mut stack);
                        if a == 0 {
                            stack.push(1);
                        } else {
                            stack.push(0);
                        }
                    },
                    '`' => {
                        let a = pop_stack_value(&mut stack);
                        let b = pop_stack_value(&mut stack);
                        if b > a {
                            stack.push(1);
                        } else {
                            stack.push(0);
                        }
                    },
                    '>' => {
                        pointer_direction = Direction::Right;
                    },  
                    '<' => {
                        pointer_direction = Direction::Left;
                    },
                   '^' => {
                        pointer_direction = Direction::Up;
                    },
                   'v' => {
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
                        let condition = pop_stack_value(&mut stack);
                        if condition == 0 {
                            pointer_direction = Direction::Right;
                        } else {
                            pointer_direction = Direction::Left;
                        }
                    },
                    '|' => {
                        let condition = pop_stack_value(&mut stack);
                        if condition == 0 {
                            pointer_direction = Direction::Down;
                        } else {
                            pointer_direction = Direction::Up;
                        }
                    },
                    '"' => string_mode = true, 
                    ':' => {
                        let value = pop_stack_value(&mut stack);
                        stack.push(value);
                        stack.push(value);
                    },
                    '\\' => {
                        let a = pop_stack_value(&mut stack);

                        let b = pop_stack_value(&mut stack);
                        stack.push(a);
                        stack.push(b);
                    },
                    '$' => {
                        let _value = pop_stack_value(&mut stack);
                    },
                    '.' => {
                        let value = pop_stack_value(&mut stack);
                        print!("{} ", value);
                        stdout().flush().unwrap();
                    },
                    ',' => {
                        let value = pop_stack_value(&mut stack);
                        let character = std::char::from_u32(value.try_into().unwrap());
                        print!("{}", character.unwrap());
                        stdout().flush().unwrap();
                    },
                    '#' => {
                        pointer = increase_pointer(pointer, &pointer_direction, &source_matrix);
                    },
                    'p' => {
                        let y = pop_stack_value(&mut stack) as usize;
                        let x = pop_stack_value(&mut stack) as usize;
                        let v = pop_stack_value(&mut stack) as u8;
                       std::mem::replace(&mut source_matrix[x][y], v.into());
                    }, 
                    'g' => {
                        let y = pop_stack_value(&mut stack) as usize;
                        let x = pop_stack_value(&mut stack) as usize;
                        let read_char = source_matrix[x][y];
                        let numeric_value = read_char as i32;
                        stack.push(numeric_value);
                    }, 
                    '&' => {
                        let mut line = String::new();
                        stdin().read_line(&mut line).unwrap();
                        line.pop();
                        let read_char = line.parse::<i32>().unwrap();
                        stack.push(read_char.try_into().unwrap());
                    },
                    '~' => {
                        let mut line = String::new();
                        stdin().read_line(&mut line).unwrap();
                        let read_char = line.chars().next().unwrap();
                        stack.push(read_char as i32);
                    },   
                    '@' => {
                        running = false;
                    },
                    ' ' => (),
                    _ => println!("Unknown command '{}'",command)
                }
            }
            pointer = increase_pointer(pointer, &pointer_direction, &source_matrix);
        }
    } else {
        println!("Plase specify source");
    }
}

fn increase_pointer(pointer: (i32,i32), direction: &Direction, source: &Vec<Vec<char>>) -> (i32,i32) {
    let rows = source.len() as i32;
    let columns = source[pointer.0 as usize].len() as i32;
    match direction {
                Direction::Right => (pointer.0, (pointer.1 +1) % columns),
                Direction::Left => {
                    let y;
                    if pointer.1 == 0 {
                        y = columns - 1;
                    } else {
                        y = 0.max(pointer.1 -1);
                    }
                    (pointer.0, y)
                },
                Direction::Up => {
                    let x;
                    if pointer.0 == 0 {
                        x = rows - 1;
                    } else {
                        x = 0.max(pointer.0 -1);
                    }
                    (x, pointer.1)
                },
                Direction::Down => ((pointer.0 + 1) % rows, pointer.1),
            }
}


fn pop_stack_value(stack: &mut Vec<i32>) -> i32 {
    if let Some(value) = stack.pop() {
        value
    } else {
        0
    }
}
