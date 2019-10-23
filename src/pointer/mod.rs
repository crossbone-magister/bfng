#[derive(PartialEq, Debug, Copy, Clone)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug)]
pub struct Pointer {
    direction: Direction,
    x: i32,
    y: i32,
    max_rows: i32,
    max_cols: i32,
}

impl Default for Pointer {
    fn default() -> Self {
        Pointer {
            direction: Direction::Right,
            x: 0,
            y: 0,
            max_rows: 25,
            max_cols: 80,
        }
    }
}

impl Pointer {
    pub fn increase(&mut self) {
        match self.direction {
            Direction::Right => self.y = (self.y + 1) % self.max_cols,
            Direction::Left => {
                if self.y == 0 {
                    self.y = self.max_cols - 1;
                } else {
                    self.y = 0.max(self.y - 1);
                }
            }
            Direction::Up => {
                if self.x == 0 {
                    self.x = self.max_rows - 1;
                } else {
                    self.x = 0.max(self.x - 1);
                }
            }
            Direction::Down => self.x = (self.x + 1) % self.max_rows,
        }
    }

    pub fn set_direction(&mut self, direction: Direction) {
        self.direction = direction;
    }

    pub fn coordinates(&self) -> (i32, i32) {
        (self.x, self.y)
    }

    #[cfg(test)]
    pub fn direction(&self) -> Direction {
        self.direction
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn new_pointer() -> Pointer {
        Pointer::default()
    }

    #[test]
    fn default() {
        let pointer = Pointer::default();
        assert_eq!(0, pointer.x);
        assert_eq!(0, pointer.y);
        assert_eq!(25, pointer.max_rows);
        assert_eq!(80, pointer.max_cols);
        assert_eq!(Direction::Right, pointer.direction);
    }
    #[test]
    fn coordinates() {
        let pointer = new_pointer();
        assert_eq!((0, 0), pointer.coordinates());
    }

    #[test]
    fn set_direction() {
        let mut pointer = new_pointer();
        pointer.set_direction(Direction::Down);
        assert_eq!(Direction::Down, pointer.direction);
    }

    #[test]
    fn increase_right() {
        let mut pointer = new_pointer();
        pointer.increase();
        assert_eq!((0, 1), pointer.coordinates());
    }

    #[test]
    fn increase_down() {
        let mut pointer = new_pointer();
        pointer.set_direction(Direction::Down);
        pointer.increase();
        assert_eq!((1, 0), pointer.coordinates());
    }

    #[test]
    fn increase_left() {
        let mut pointer = new_pointer();
        pointer.increase();
        assert_eq!((0, 1), pointer.coordinates());
        pointer.set_direction(Direction::Left);
        pointer.increase();
        assert_eq!((0, 0), pointer.coordinates());
    }

    #[test]
    fn increase_up() {
        let mut pointer = new_pointer();
        pointer.set_direction(Direction::Down);
        pointer.increase();
        assert_eq!((1, 0), pointer.coordinates());
        pointer.set_direction(Direction::Up);
        pointer.increase();
        assert_eq!((0, 0), pointer.coordinates());
    }

    #[test]
    fn increase_right_wrap() {
        let mut pointer = new_pointer();
        for _ in 1..pointer.max_cols {
            pointer.increase();
        }
        assert_eq!((0, 79), pointer.coordinates());
        pointer.increase();
        assert_eq!((0, 0), pointer.coordinates());
    }

    #[test]
    fn increase_left_wrap() {
        let mut pointer = new_pointer();
        pointer.set_direction(Direction::Left);
        pointer.increase();
        assert_eq!((0, 79), pointer.coordinates());
    }

    #[test]
    fn increase_up_wrap() {
        let mut pointer = new_pointer();
        pointer.set_direction(Direction::Up);
        pointer.increase();
        assert_eq!((24, 0), pointer.coordinates());
    }

    #[test]
    fn increase_down_wrap() {
        let mut pointer = new_pointer();
        pointer.set_direction(Direction::Down);
        for _ in 1..pointer.max_rows {
            pointer.increase();
        }
        assert_eq!((24, 0), pointer.coordinates());
        pointer.increase();
        assert_eq!((0, 0), pointer.coordinates());
    }

    #[test]
    fn direction() {
        let pointer = new_pointer();
        assert_eq!(Direction::Right, pointer.direction);
    }
}
