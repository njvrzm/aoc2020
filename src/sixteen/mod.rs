pub mod day_sixteen {
    use std::str::FromStr;
    use std::string::ParseError;
    use itertools::Itertools;

    #[derive(Debug)]
    struct Range(i32, i32);
    impl Range {
        fn contains(self, n: i32) -> bool {
            self.0<=n && n<=self.1
        }
    }
    impl FromStr for Range {
        type Err = ParseError;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let (low, high): (i32, i32) = s.splitn(2, "-").map(|s| s.parse().unwrap()).collect_tuple().unwrap();
            Ok(Self(low, high))
        }
    }
    impl Range {
        fn from_ranges(s: &str) -> (Self, Self) {
            let (low, high): (&str, &str) = s.splitn(2, " or ").collect_tuple().unwrap();
            (Range::from_str(low).unwrap(), Range::from_str(high).unwrap())

        }
    }

    #[derive(Debug)]
    struct Field {
        name: String,
        low: Range,
        high: Range,
    }
    impl Field {
        fn matches(self, n: i32) -> bool {
            self.low.contains(n) || self.high.contains(n)
        }
    }

    impl FromStr for Field {
        type Err = ParseError;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let (name, ranges): (&str, &str) = s.splitn(2, ": ").collect_tuple().unwrap();
            let (low, high) = Range::from_ranges(ranges);
            Ok(Self{name: String::from(name), low, high})

        }
    }
    pub fn run_one(input: String) {
        let lines = input.lines();
        let 
        for line in lines {
            if line == "" {
                break
            }
        }
        let f = Field::from_str(input.lines().next().unwrap()).unwrap();
        println!("{:?}", f);

    }
}