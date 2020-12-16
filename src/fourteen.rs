pub mod day_fourteen {
    use std::collections::HashMap;
    use std::str::FromStr;
    use std::string::ParseError;
    use itertools::Itertools;

    fn parse_mask(s: &str) -> Instruction {
        let and_mask = u64::from_str_radix( &s.replace("X", "1"), 2).unwrap();
        let or_mask = u64::from_str_radix( &s.replace("X", "0"), 2).unwrap();
        Instruction::Mask(and_mask, or_mask)
    }
    fn parse_mem(var: &str, val: &str) -> Instruction {
        let addr: u64 = var[var.find("[").unwrap()+1..var.find("]").unwrap()].parse().unwrap();
        let value: u64 = val.parse().unwrap();
        Instruction::Mem(addr, value)

    }
    enum Instruction {
        Mask(u64, u64),
        Mem(u64, u64),
        Nothing,
    }
    impl Instruction {
        fn apply(&self, val: u64) -> Option<u64> {
            match self {
                Instruction::Mask(a, o) => Some(val&a|o),
                _ => None,
            }
        }
    }
    impl FromStr for Instruction {
        type Err = ParseError;

        fn from_str(line: &str) -> Result<Self, Self::Err> {
            let (who, what): (&str, &str) = line.trim().splitn(2, " = ").collect_tuple().unwrap();
            println!("{}: {}", who, what);
            match who {
                "mask" => Ok(parse_mask(what)),
                s if s.starts_with("mem") => Ok(parse_mem(who, what)),
                _ => panic!("Unrecognized instruction {}", line),
            }
        }
    }
    struct Bank {
        memory: HashMap<u64, u64>,
        mask: Instruction,
    }
    impl Bank {
        fn new() -> Self {
            Self{memory: HashMap::new(), mask: Instruction::Nothing}
        }
        fn set(&mut self, addr: u64, value: u64) {
            let val = self.mask.apply(value).unwrap();
            self.memory.insert(addr, val);
        }
        fn set_weird(&mut self, addr: u64, value: u64) {
            if let Instruction::Mask(and_mask, or_mask) = self.mask {
                let x_mask = and_mask^or_mask;
                let addr = (addr|or_mask)&!x_mask;
                let bits: Vec<u64> = (0..36).map(|i| 1<<i & x_mask).filter(|&b|b>0).collect();
                for k in 0..=bits.len() {
                    for combination in bits.iter().combinations(k) {
                        let variant: u64 = combination.iter().copied().sum();
                        self.memory.insert(addr+variant, value);

                    }
                }
            }
        }
        fn sum(self) -> u64 {
            let mut v: u64 = 0;
            // let z: Vec<u64> = Vec::new();

            for k in self.memory.keys().sorted() {
                println!("{}\t{}", k, self.memory.get(k).unwrap());
                v += self.memory.get(k).unwrap();
            }
            v
        }
    }

    pub fn run_one(input: String) {
        let mut bank = Bank::new();
        for line in input.lines() {
            let instruction = Instruction::from_str(line).unwrap();
            match instruction {
                Instruction::Mask(_, _) => bank.mask = instruction,
                Instruction::Mem(addr, value) => bank.set(addr, value),
                _ => panic!("Unexpected instruction"),
            }
        }
        println!("Final sum: {}", bank.sum())

    }
    pub fn run_two(input: String) {
        let mut bank = Bank::new();
        for line in input.lines() {
            let instruction = Instruction::from_str(line).unwrap();
            match instruction {
                Instruction::Mask(_, _) => bank.mask = instruction,
                Instruction::Mem(addr, value) => bank.set_weird(addr, value),
                _ => panic!("Unexpected instruction"),
            }
        }
        println!("Final sum: {}", bank.sum())

    }
}