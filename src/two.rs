pub mod day_two {
    use regex::Regex;
    use itertools::Itertools;
    use std::str::FromStr;

    struct Spec {
        low: usize,
        high: usize,
        character: char,
        password: String,
    }

    impl FromStr for Spec {
        type Err = regex::Error;
        fn from_str(line: &str) -> Result<Self, Self::Err> {
            let splitter: Regex = Regex::new(r"[-: ]+").unwrap();
            let (lowbit, highbit, charbit, password): (&str, &str, &str, &str) = splitter.split(line).collect_tuple().unwrap();
            let low = lowbit.parse::<usize>().unwrap();
            let high = highbit.parse::<usize>().unwrap();
            let char = charbit.parse::<char>().unwrap();
            Ok(Spec { low: low, high: high, character: char, password: String::from(password)})
        }
    }

    pub fn run_one(input: String) {
        let mut total_good = 0;
        for line in input.lines() {
            let spec = Spec::from_str(line).unwrap();
            let count = spec.password.chars().filter(|c| c == &spec.character).count();

            if spec.low <= count && count <= spec.high {
                total_good += 1;
            }
        }
        println!("Good passwords: {}", total_good);
    }

    pub fn run_two(input: String) {
        let mut total_good = 0;
        for line in input.lines() {
            let spec = Spec::from_str(line).unwrap();
            let chars: Vec<char> = spec.password.chars().collect();
            let low_is = chars[spec.low - 1] == spec.character;
            let high_is = chars[spec.high - 1] == spec.character;
            if low_is ^ high_is {
                total_good += 1;
            }

        }
        println!("Good passwords: {}", total_good);
    }
}