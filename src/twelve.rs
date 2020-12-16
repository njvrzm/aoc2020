pub mod day_twelve {
    use std::ops::{Mul, AddAssign};
    use std::str::FromStr;
    use std::string::ParseError;
    use std::fmt;
    use crate::twelve::day_twelve::Direction::{West, East, South, North};
    use itertools::Itertools;

    #[derive(Debug)]
    enum Instruction {
        Move(Option<Direction>, i32),
        Right(i32),
        Left(i32)
    }
    #[derive(Debug)]
    #[derive(Copy,Clone)]
    enum Direction {
        West,
        North,
        South,
        East,
    }

    impl FromStr for Instruction {
        type Err = ParseError;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let (instruction, value) = s.split_at(1);
            let v = value.parse::<i32>().expect( &format!("Unexpected value format: {}", value));
            Ok(match instruction {
                "W" => Instruction::Move(Some(West), v),
                "N" => Instruction::Move(Some(North), v),
                "E" => Instruction::Move(Some(East), v),
                "S" => Instruction::Move(Some(South), v),
                "R" => Instruction::Right(v),
                "L" => Instruction::Left(v),
                "F" => Instruction::Move(None, v),
                x => panic!("Unrecognized instruction: {}", x),
            })
        }
    }
    impl fmt::Display for Instruction {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                Instruction::Move(Some(West), v) => write!(f, "W{}", v),
                Instruction::Move(Some(East), v) => write!(f, "E{}", v),
                Instruction::Move(Some(South), v) => write!(f, "S{}", v),
                Instruction::Move(Some(North), v) => write!(f, "N{}", v),
                Instruction::Move(None, v) => write!(f, "F{}", v),
                Instruction::Right(v) => write!(f, "R{}", v),
                Instruction::Left(v) => write!(f, "L{}", v),
            }
        }
    }
    #[derive(Debug)]
    struct Locus {
        dx: i32,
        dy: i32,
    }

    const ORIGIN: Locus = Locus{dx: 0, dy: 0};

    const EAST: Locus = Locus{dx: 1, dy: 0};
    const WEST: Locus = Locus{dx: -1, dy: 0};
    const NORTH: Locus = Locus{dx: 0, dy: 1};
    const SOUTH: Locus = Locus{dx: 0, dy: -1};

    impl Mul<i32> for Locus {
        type Output = Self;

        fn mul(self, rhs: i32) -> Self::Output {
            return Self::new(self.dx * rhs, self.dy * rhs)
        }
    }
    impl AddAssign<Locus> for Locus {
        fn add_assign(&mut self, rhs: Locus) {
            self.dx += rhs.dx;
            self.dy += rhs.dy;
        }
    }
    impl Locus {
        fn new(dx: i32, dy: i32) -> Self {
            return Self{dx, dy}

        }
    }
    struct Ship {
        at: Locus,
        facing: Direction,
    }
    impl Ship {
        fn follow(&mut self, i: &Instruction) {
            match i {
                Instruction::Move(direction, distance) => self.go(direction, distance),
                Instruction::Right(d) => self.turn(d),
                Instruction::Left(d) => self.turn(&(360-d)),
            }
        }
        fn go(&mut self, direction: &Option<Direction>, distance: &i32) {
            self.at += (match direction.unwrap_or(self.facing) {
                West => WEST,
                South => SOUTH,
                East => EAST,
                North => NORTH,
            }) * *distance;
        }
        fn go_to(&mut self, other: &Ship, times: i32) {
            self.at.dx += other.at.dx * times;
            self.at.dy += other.at.dy * times;
        }
        fn turn(&mut self, d: &i32) {
            self.facing = match (&self.facing, d) {
                (West, 90)|(East, 270)|(South, 180) => North,
                (East, 90)|(West, 270)|(North, 180) => South,
                (South, 90)|(North, 270)|(East, 180) => West,
                (North, 90)|(South, 270)|(West, 180) => East,
                _ => panic!("Unrecognized turning situation: {:?} {}", self.facing, d),
            }
        }
        fn rotate(&mut self, d: &i32) {
            let x = self.at.dx;
            let y = self.at.dy;
            match d {
                90 => {self.at.dx = y; self.at.dy = -x},
                180 => {self.at.dx = -x; self.at.dy = -y},
                270 => {self.at.dx = -y; self.at.dy = x},
                _ => panic!("Unrecognized rotation: {:?} {}", self.at, d),
            }
        }
    }
    fn instructions(input: String) -> Vec<Instruction> {
        return input.lines().map(|l| Instruction::from_str(l).unwrap()).collect_vec();
    }
    pub fn run_one(input: String) {
        let mut ship = Ship{at: ORIGIN, facing: East};
        for instruction in instructions(input) {
            ship.follow(&instruction);
            println!("Instruction: {} \tAt: {}*{}\tFacing: {:?}", instruction, ship.at.dx, ship.at.dy, ship.facing);
        }
        println!("Taxicab distance: {}", ship.at.dx.abs() + ship.at.dy.abs())

    }
    pub fn run_two(input: String) {
        let mut ship = Ship{at: ORIGIN, facing: East};
        let mut waypoint = Ship{at: Locus{dx: 10, dy: 1}, facing: North};
        for instruction in instructions(input) {
            match instruction {
                Instruction::Move(None, d) => ship.go_to(&waypoint, d),
                Instruction::Move(_, _) => waypoint.follow(&instruction),
                Instruction::Right(d) => waypoint.rotate(&d),
                Instruction::Left(d) => waypoint.rotate(&(360-d)),
            }
            println!("Instruction: {} \tAt: {}*{}\tFacing: {:?}", instruction, ship.at.dx, ship.at.dy, ship.facing);
        }
        println!("Taxicab distance: {}", ship.at.dx.abs() + ship.at.dy.abs())

    }
}