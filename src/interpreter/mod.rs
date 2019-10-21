use crate::pointer::*;
use crate::stack::*;
use rand::Rng;
use std::{
    convert::TryInto,
    io::{BufRead, BufReader, BufWriter, Write},
    marker::PhantomData,
};

#[derive(Debug)]
pub struct Interpreter<S, I, O>
where
    S: std::io::BufRead,
    I: std::io::BufRead,
    O: std::io::Write,
{
    source_matrix: Vec<Vec<char>>,
    pointer: Pointer,
    stack: Stack,
    running: bool,
    string_mode: bool,
    stdin: BufReader<I>,
    stdout: BufWriter<O>,
    _marker: PhantomData<S>,
}

impl<S, I, O> Interpreter<S, I, O>
where
    S: std::io::BufRead,
    I: std::io::BufRead,
    O: std::io::Write,
{
    pub fn new(source: BufReader<S>, stdin: BufReader<I>, stdout: BufWriter<O>) -> Self {
        let mut source_matrix = vec![vec!(' '; 80); 25];
        for (x, line) in source.lines().enumerate() {
            for (y, command) in line.unwrap().chars().enumerate() {
                source_matrix[x][y] = command;
            }
        }
        Interpreter {
            source_matrix,
            pointer: Pointer::default(),
            stack: Stack::default(),
            running: true,
            string_mode: false,
            stdin,
            stdout,
            _marker: PhantomData,
        }
    }

    pub fn program_ended(&self) -> bool {
        !self.running
    }

    pub fn execute(&mut self) {
        if self.running {
            let (x, y) = self.pointer.coordinates();
            let command = self.source_matrix[x as usize][y as usize];
            if self.string_mode {
                self.string_mode_execution(command);
            } else {
                self.command_execution(command);
            }
            self.pointer.increase();
        } else {
            panic!("Cannot execute further on an ended program");
        }
    }

    fn string_mode_execution(&mut self, character: char) {
        match character {
            '"' => self.string_mode = false,
            _ => self.stack.push_char(character),
        }
    }

    fn command_execution(&mut self, command: char) {
        match command {
            '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => self
                .stack
                .push(command.to_digit(10).unwrap().try_into().unwrap()),
            '+' => {
                let a = self.stack.pop();
                let b = self.stack.pop();
                self.stack.push(a + b);
            }
            '-' => {
                let a = self.stack.pop();
                let b = self.stack.pop();
                self.stack.push(b - a);
            }
            '*' => {
                let a = self.stack.pop();
                let b = self.stack.pop();
                self.stack.push(a * b);
            }
            '/' => {
                let a = self.stack.pop();
                let b = self.stack.pop();
                self.stack.push(b / a);
            }
            '%' => {
                let a = self.stack.pop();
                let b = self.stack.pop();
                self.stack.push(b % a);
            }
            '!' => {
                let a = self.stack.pop();
                if a == 0 {
                    self.stack.push(1);
                } else {
                    self.stack.push(0);
                }
            }
            '`' => {
                let a = self.stack.pop();
                let b = self.stack.pop();
                if b > a {
                    self.stack.push(1);
                } else {
                    self.stack.push(0);
                }
            }
            '>' => {
                self.pointer.set_direction(Direction::Right);
            }
            '<' => {
                self.pointer.set_direction(Direction::Left);
            }
            '^' => {
                self.pointer.set_direction(Direction::Up);
            }
            'v' => {
                self.pointer.set_direction(Direction::Down);
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
                self.pointer.set_direction(pointer_direction);
            }
            '_' => {
                let condition = self.stack.pop();
                let pointer_direction;
                if condition == 0 {
                    pointer_direction = Direction::Right;
                } else {
                    pointer_direction = Direction::Left;
                }
                self.pointer.set_direction(pointer_direction)
            }
            '|' => {
                let condition = self.stack.pop();
                let pointer_direction;
                if condition == 0 {
                    pointer_direction = Direction::Down;
                } else {
                    pointer_direction = Direction::Up;
                }
                self.pointer.set_direction(pointer_direction);
            }
            '"' => self.string_mode = true,
            ':' => {
                let value = self.stack.pop();
                self.stack.push(value);
                self.stack.push(value);
            }
            '\\' => {
                let a = self.stack.pop();
                let b = self.stack.pop();
                self.stack.push(a);
                self.stack.push(b);
            }
            '$' => {
                let _value = self.stack.pop();
            }
            '.' => {
                let value = self.stack.pop();
                print!("{} ", value);
                self.stdout.flush().unwrap();
            }
            ',' => {
                let character = self.stack.pop_char();
                print!("{}", character);
                self.stdout.flush().unwrap();
            }
            '#' => {
                self.pointer.increase();
            }
            'p' => {
                let y = self.stack.pop() as usize;
                let x = self.stack.pop() as usize;
                let v = self.stack.pop() as u8;
                std::mem::replace(&mut self.source_matrix[x][y], v.into());
            }
            'g' => {
                let y = self.stack.pop() as usize;
                let x = self.stack.pop() as usize;
                if x < self.source_matrix.len() && y < self.source_matrix[x].len() {
                    let read_char = self.source_matrix[x][y];
                    let numeric_value = read_char as i32;
                    self.stack.push(numeric_value);
                } else {
                    self.stack.push(0);
                }
            }
            '&' => {
                let mut line = String::new();
                self.stdin.read_line(&mut line).unwrap();
                line.pop();
                let read_char = line.parse::<i32>().unwrap();
                self.stack.push(read_char.try_into().unwrap());
            }
            '~' => {
                let mut line = String::new();
                self.stdin.read_line(&mut line).unwrap();
                let read_char = line.chars().next().unwrap();
                self.stack.push(read_char as i32);
            }
            '@' => {
                self.running = false;
            }
            ' ' => (),
            _ => panic!("Unknown command '{}'", command),
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    fn new_interpreter<'a>(
        source: &'a mut String,
        stdin: &'a mut String,
        stdout: &'a mut Vec<u8>,
    ) -> Interpreter<&'a [u8], &'a [u8], Vec<u8>> {
        Interpreter::new(
            BufReader::new(source.as_bytes()),
            BufReader::new(stdin.as_bytes()),
            BufWriter::new(stdout.to_vec()),
        )
    }

    #[test]
    fn new() {
        let mut source = String::from("");
        let mut stdin = String::from("");
        let mut stdout = vec![];
        let interpreter = new_interpreter(&mut source, &mut stdin, &mut stdout);
        assert!(interpreter.running);
        assert!(!interpreter.string_mode);
        let source_matrix = interpreter.source_matrix;
        assert_eq!(25, source_matrix.len());
        for line in source_matrix {
            assert_eq!(80, line.len());
        }
    }

    #[test]
    fn program_ended_false() {
        let mut source = String::from("");
        let mut stdin = String::from("");
        let mut stdout = vec![];
        let interpreter = new_interpreter(&mut source, &mut stdin, &mut stdout);
        assert!(!interpreter.program_ended(), "Program shouldn't be ended");
    }

    #[test]
    fn program_ended_true() {
        let mut source = String::from("@");
        let mut stdin = String::from("");
        let mut stdout = vec![];
        let mut interpreter = new_interpreter(&mut source, &mut stdin, &mut stdout);
        assert!(!interpreter.program_ended(), "Program shouldn't be ended");
        interpreter.execute();
        assert!(interpreter.program_ended(), "Program should be ended");
    }

    #[test]
    #[should_panic]
    fn execute_panic() {
        let mut source = String::from("@");
        let mut stdin = String::from("");
        let mut stdout = vec![];
        let mut interpreter = new_interpreter(&mut source, &mut stdin, &mut stdout);
        assert!(!interpreter.program_ended(), "Program shouldn't be ended");
        interpreter.execute();
        assert!(interpreter.program_ended(), "Program should be ended");
        interpreter.execute();
    }

    #[test]
    fn string_mode_execution_push_to_stack() {
        let mut source = String::from("@");
        let mut stdin = String::from("");
        let mut stdout = vec![];
        let mut interpreter = new_interpreter(&mut source, &mut stdin, &mut stdout);
        interpreter.string_mode_execution('A');
        assert_eq!(65, interpreter.stack.pop());
    }

    #[test]
    fn string_mode_execution_end() {
        let mut source = String::from("@");
        let mut stdin = String::from("");
        let mut stdout = vec![];
        let mut interpreter = new_interpreter(&mut source, &mut stdin, &mut stdout);
        interpreter.string_mode = true;
        interpreter.string_mode_execution('"');
        assert_eq!(0, interpreter.stack.pop());
        assert!(!interpreter.string_mode);
    }

    #[test]
    fn command_execution_digits() {
        let digits = vec!['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];
        let mut source = String::from("@");
        let mut stdin = String::from("");
        let mut stdout = vec![];
        let mut interpreter = new_interpreter(&mut source, &mut stdin, &mut stdout);
        for digit in digits {
            interpreter.command_execution(digit);
            assert_eq!(digit.to_digit(10).unwrap(), interpreter.stack.pop() as u32);
        }
    }

    #[test]
    fn command_execution_add() {
        let mut source = String::from("@");
        let mut stdin = String::from("");
        let mut stdout = vec![];
        let mut interpreter = new_interpreter(&mut source, &mut stdin, &mut stdout);
        interpreter.stack.push(1);
        interpreter.stack.push(2);
        interpreter.command_execution('+');
        assert_eq!(3, interpreter.stack.pop());
    }

    #[test]
    fn command_execution_subtract() {
        let mut source = String::from("@");
        let mut stdin = String::from("");
        let mut stdout = vec![];
        let mut interpreter = new_interpreter(&mut source, &mut stdin, &mut stdout);
        interpreter.stack.push(1);
        interpreter.stack.push(2);
        interpreter.command_execution('-');
        assert_eq!(-1, interpreter.stack.pop());
    }

    #[test]
    fn command_execution_multiply() {
        let mut source = String::from("@");
        let mut stdin = String::from("");
        let mut stdout = vec![];
        let mut interpreter = new_interpreter(&mut source, &mut stdin, &mut stdout);
        interpreter.stack.push(3);
        interpreter.stack.push(2);
        interpreter.command_execution('*');
        assert_eq!(6, interpreter.stack.pop());
    }

    #[test]
    fn command_execution_divide() {
        let mut source = String::from("@");
        let mut stdin = String::from("");
        let mut stdout = vec![];
        let mut interpreter = new_interpreter(&mut source, &mut stdin, &mut stdout);
        interpreter.stack.push(10);
        interpreter.stack.push(2);
        interpreter.command_execution('/');
        assert_eq!(5, interpreter.stack.pop());
    }

    #[test]
    fn command_execution_modulus() {
        let mut source = String::from("@");
        let mut stdin = String::from("");
        let mut stdout = vec![];
        let mut interpreter = new_interpreter(&mut source, &mut stdin, &mut stdout);
        interpreter.stack.push(5);
        interpreter.stack.push(2);
        interpreter.command_execution('%');
        assert_eq!(1, interpreter.stack.pop());
    }

    #[test]
    fn command_execution_not_true() {
        let mut source = String::from("@");
        let mut stdin = String::from("");
        let mut stdout = vec![];
        let mut interpreter = new_interpreter(&mut source, &mut stdin, &mut stdout);
        interpreter.stack.push(0);
        interpreter.command_execution('!');
        assert_eq!(1, interpreter.stack.pop());
    }

    #[test]
    fn command_execution_not_false() {
        let mut source = String::from("@");
        let mut stdin = String::from("");
        let mut stdout = vec![];
        let mut interpreter = new_interpreter(&mut source, &mut stdin, &mut stdout);
        interpreter.stack.push(78);
        interpreter.command_execution('!');
        assert_eq!(0, interpreter.stack.pop());
    }

    #[test]
    fn command_execution_greater_than_greater() {
        let mut source = String::from("@");
        let mut stdin = String::from("");
        let mut stdout = vec![];
        let mut interpreter = new_interpreter(&mut source, &mut stdin, &mut stdout);
        interpreter.stack.push(99);
        interpreter.stack.push(0);
        interpreter.command_execution('`');
        assert_eq!(1, interpreter.stack.pop());
    }

    #[test]
    fn command_execution_greater_than_lesser() {
        let mut source = String::from("@");
        let mut stdin = String::from("");
        let mut stdout = vec![];
        let mut interpreter = new_interpreter(&mut source, &mut stdin, &mut stdout);
        interpreter.stack.push(0);
        interpreter.stack.push(99);
        interpreter.command_execution('`');
        assert_eq!(0, interpreter.stack.pop());
    }

    #[test]
    fn pointer_right() {
        let mut source = String::from("@");
        let mut stdin = String::from("");
        let mut stdout = vec![];
        let mut interpreter = new_interpreter(&mut source, &mut stdin, &mut stdout);
        interpreter.pointer.set_direction(Direction::Down);
        interpreter.command_execution('>');
        assert_eq!(Direction::Right, interpreter.pointer.direction());
    }

    #[test]
    fn pointer_left() {
        let mut source = String::from("@");
        let mut stdin = String::from("");
        let mut stdout = vec![];
        let mut interpreter = new_interpreter(&mut source, &mut stdin, &mut stdout);
        interpreter.command_execution('<');
        assert_eq!(Direction::Left, interpreter.pointer.direction());
    }

    #[test]
    fn pointer_up() {
        let mut source = String::from("@");
        let mut stdin = String::from("");
        let mut stdout = vec![];
        let mut interpreter = new_interpreter(&mut source, &mut stdin, &mut stdout);
        interpreter.pointer.set_direction(Direction::Down);
        interpreter.command_execution('^');
        assert_eq!(Direction::Up, interpreter.pointer.direction());
    }

    #[test]
    fn pointer_down() {
        let mut source = String::from("@");
        let mut stdin = String::from("");
        let mut stdout = vec![];
        let mut interpreter = new_interpreter(&mut source, &mut stdin, &mut stdout);
        interpreter.pointer.set_direction(Direction::Down);
        interpreter.command_execution('v');
        assert_eq!(Direction::Down, interpreter.pointer.direction());
    }

    #[test]
    fn pointer_random() {
        let mut source = String::from("@");
        let mut stdin = String::from("");
        let mut stdout = vec![];
        let mut interpreter = new_interpreter(&mut source, &mut stdin, &mut stdout);
        interpreter.pointer.set_direction(Direction::Down);
        interpreter.command_execution('?');
        /*
            Since there's a 1/4 chance that the random direction will be Right, I cannot test the method with assert_ne!(Direction::Right, interpreter.pointer.direction()).
            A way around might be to check if the function set_direction was called interpreter.pointer but I don't know how to do that and it's two o'clock in the morning
        */
    }

    #[test]
    fn command_execution_horizontal_if_zero() {
        let mut source = String::from("@");
        let mut stdin = String::from("");
        let mut stdout = vec![];
        let mut interpreter = new_interpreter(&mut source, &mut stdin, &mut stdout);
        interpreter.stack.push(0);
        interpreter.command_execution('_');
        assert_eq!(Direction::Right,interpreter.pointer.direction());
    }

    #[test]
    fn command_execution_horizontal_if_not_zero() {
        let mut source = String::from("@");
        let mut stdin = String::from("");
        let mut stdout = vec![];
        let mut interpreter = new_interpreter(&mut source, &mut stdin, &mut stdout);
        interpreter.stack.push(1);
        interpreter.command_execution('_');
        assert_eq!(Direction::Left,interpreter.pointer.direction());
    }

    #[test]
    fn command_execution_vertical_if_zero() {
        let mut source = String::from("@");
        let mut stdin = String::from("");
        let mut stdout = vec![];
        let mut interpreter = new_interpreter(&mut source, &mut stdin, &mut stdout);
        interpreter.stack.push(0);
        interpreter.command_execution('|');
        assert_eq!(Direction::Down,interpreter.pointer.direction());
    }

    #[test]
    fn command_execution_vertical_if_not_zero() {
        let mut source = String::from("@");
        let mut stdin = String::from("");
        let mut stdout = vec![];
        let mut interpreter = new_interpreter(&mut source, &mut stdin, &mut stdout);
        interpreter.stack.push(1);
        interpreter.command_execution('|');
        assert_eq!(Direction::Up,interpreter.pointer.direction());
    }

    #[test]
    fn command_execution_string_mode() {
        let mut source = String::from("@");
        let mut stdin = String::from("");
        let mut stdout = vec![];
        let mut interpreter = new_interpreter(&mut source, &mut stdin, &mut stdout);
        interpreter.command_execution('"');
        assert!(interpreter.string_mode);
    }

    #[test]
    fn command_execution_duplicate() {
        let mut source = String::from("@");
        let mut stdin = String::from("");
        let mut stdout = vec![];
        let mut interpreter = new_interpreter(&mut source, &mut stdin, &mut stdout);
        interpreter.stack.push(2);
        interpreter.command_execution(':');
        assert_eq!(2,interpreter.stack.pop());
        assert_eq!(2,interpreter.stack.pop());
    }

    #[test]
    #[should_panic]
    fn command_execution_unknown_command() {
        let mut source = String::from("@");
        let mut stdin = String::from("");
        let mut stdout = vec![];
        let mut interpreter = new_interpreter(&mut source, &mut stdin, &mut stdout);
        interpreter.command_execution('A');
    }
}
