pub mod day_eight {
    use itertools::Itertools;
    use std::str::FromStr;
    use std::string::ParseError;

    enum Operator {
        Accumulate,
        Jump,
        Noop,
    }
    struct Console {
        accumulator: i32,
        pointer: i32,
        code: Vec<Instruction>,
        history: Vec<i32>,
    }
    struct Instruction {
        kind: Operator,
        value: i32,
    }
    impl FromStr for Instruction {
        type Err = ParseError;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let (name, val): (&str, &str) = s.splitn(2, " ").collect_tuple().expect("Bad instruction");
            match name {
                "acc" => Ok(Instruction{kind: Operator::Accumulate, value: val.parse().expect("Bad val")}),
                "jmp" => Ok(Instruction{kind: Operator::Jump, value: val.parse().expect("Bad val")}),
                _ => Ok(Instruction{kind: Operator::Noop, value: val.parse().expect("Bad val")}),
            }
        }
    }

    enum State {
        Running,
        Looping,
        Terminated,
    }
    impl Console {
        fn accumulate(&mut self, n: i32) {
            self.accumulator += n;
            self.pointer += 1;
        }
        fn jump(&mut self, n: i32) {
            self.pointer += n;
        }
        fn noop(&mut self, _n: i32) {
            self.pointer += 1;
        }
        fn twiddle(&mut self, pointer: usize) -> bool {
            match self.code[pointer].kind {
                Operator::Jump => {
                    self.code[pointer].kind = Operator::Noop;
                    true
                },
                Operator::Noop => {
                    self.code[pointer].kind  = Operator::Jump;
                    true
                },
                Operator::Accumulate => false,
            }
        }

        fn step(&mut self) -> State {
            if self.pointer == self.code.len() as i32 {
                return State::Terminated
            } else if self.history[self.pointer as usize] == 1 {
                return State::Looping
            }

            self.history[self.pointer as usize] = 1;
            match self.code[self.pointer as usize] {
                Instruction{kind: Operator::Accumulate, value: v} => {self.accumulate(v); State::Running},
                Instruction{kind: Operator::Jump, value: v} => {self.jump(v); State::Running},
                Instruction{kind: Operator::Noop, value: v} => {self.noop(v); State::Running},
            }
        }

        fn reset(&mut self) {
            self.accumulator = 0;
            self.pointer = 0;
            self.history = vec![0;self.code.len()];
        }

        fn run(&mut self) -> bool {
            loop {
                match self.step() {
                    State::Looping => return false,
                    State::Terminated => return true,
                    State::Running => ()
                }
            }
        }

        fn new(input: String) -> Self {
            let code = input.lines().map(|l| Instruction::from_str(l).expect("Failed to parse instruction")).collect_vec();
            let size = code.len();
            Self {
                accumulator: 0,
                pointer: 0,
                code,
                history: vec![0;size],
            }
        }

    }
    pub fn run_one(input: String) {
        let mut console = Console::new(input);
        match console.run() {
            false => println!("Loop detected at {}. Accumulator: {}", console.pointer, console.accumulator),
            true => panic!("Error: success")
        }
    }

    pub fn run_two(input: String) {
        let mut console = Console::new(input);
        for p in 0..console.code.len() {
            if console.twiddle(p) {
                match console.run() {
                    true => {println!("Success! Terminated with {}", console.accumulator); return},
                    false => {
                        console.twiddle(p);
                        console.reset();
                    }
                }
            }
        }
    }
}