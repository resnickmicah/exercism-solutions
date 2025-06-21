// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(PartialEq, Eq, Debug)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

pub struct Robot {
    position: (i32, i32),
    direction: Direction,
}

impl Robot {
    pub fn new(x: i32, y: i32, d: Direction) -> Self {
        Robot {
            position: (x, y),
            direction: d,
        }
    }

    #[must_use]
    pub fn turn_right(mut self) -> Self {
        use self::Direction::*;
        self.direction = match self.direction {
            North => East,
            East => South,
            South => West,
            West => North,
        };
        self
    }

    #[must_use]
    pub fn turn_left(mut self) -> Self {
        use self::Direction::*;
        self.direction = match self.direction {
            North => West,
            East => North,
            South => East,
            West => South,
        };
        self
    }

    #[must_use]
    pub fn advance(mut self) -> Self {
        use self::Direction::*;
        self.position = match self.direction {
            North => (self.position.0, self.position.1 + 1),
            East => (self.position.0 + 1, self.position.1),
            South => (self.position.0, self.position.1 - 1),
            West => (self.position.0 - 1, self.position.1),
        };
        self
    }

    #[must_use]
    pub fn instructions(mut self, instructions: &str) -> Self {
        for instr in instructions.chars() {
            self = match instr {
                'R' => self.turn_right(),
                'L' => self.turn_left(),
                'A' => self.advance(),
                _ => panic!("Invalid instruction"),
            };
        }
        self
    }

    pub fn position(&self) -> (i32, i32) {
        self.position
    }

    pub fn direction(&self) -> &Direction {
        &self.direction
    }
}
