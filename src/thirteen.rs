pub mod day_thirteen {
    use itertools::Itertools;
    use std::i32::MAX;
    use std::fmt;

    fn parse(input: String) -> (i32, Vec<i32>) {
        let (one, two): (&str, &str) = input.splitn(2, "\n").collect_tuple().unwrap();
        (one.parse().unwrap(), two.split(",").map(|b| b.parse().unwrap_or(0)).filter(|b| b>&0).collect())
    }
    fn parse_no_filter(input: String) -> (i32, Vec<i32>) {
        let (one, two): (&str, &str) = input.splitn(2, "\n").collect_tuple().unwrap();
        (one.parse().unwrap(), two.trim().split(",").map(|b| b.parse().unwrap_or(0)).collect())
    }
    pub fn run_one(input: String) {

        let (time, buses) = parse(input);
        let mut minimest = MAX;
        let mut thebus  = 0;
        for bus in buses {
            if bus - time%bus < minimest {
                minimest = bus - time%bus;
                thebus = bus;
            }
        }
        println!("{}: {} ({})", minimest, thebus, minimest*thebus);
    }


    struct Aspect {
        modulus: u64,
        remainder: u64,
    }
    impl fmt::Debug for Aspect {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "=%{} {}", self.modulus, self.remainder)
        }
    }

    pub fn run_two(input: String) {
        let (_, buses) = parse_no_filter(input);
        let mut requirements: Vec<Aspect> = Vec::new();
        for (i, bus) in buses.iter().enumerate() {
            let modulus = *bus;
            if modulus == 0 {
                continue
            }
            let remainder = (modulus*100 - i as i32)%modulus;
            println!("{}: {}", modulus, remainder);
            requirements.push(Aspect{modulus: modulus as u64, remainder: (remainder%modulus) as u64});
        }
        requirements.sort_by(|a, b| b.modulus.cmp(&a.modulus));
        let mut goal: u64 = 0;
        let mut step: u64 = 1;
        for requirement in &requirements {
            if goal == 0 {
                goal = requirement.remainder;
                step = requirement.modulus;
                continue
            }
            while goal%requirement.modulus != requirement.remainder {
                goal += step;
            }
            step *= requirement.modulus;
            println!("{}", goal);
        }
        for requirement in &requirements {
            println!("{} % {} = {} (need {})", goal, requirement.modulus, goal%requirement.modulus, requirement.remainder);

        }
    }


}