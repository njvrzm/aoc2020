pub mod day_sixteen {
    use std::str::FromStr;
    use std::string::ParseError;
    use itertools::Itertools;
    use std::num::ParseIntError;
    use std::collections::HashMap;

    #[derive(Debug)]
    struct Ticket(Vec<i32>);
    impl FromStr for Ticket {
        type Err = ParseIntError;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let numbers = s.split(",")
                .map(|n| n.parse::<i32>())
                .collect::<Result<Vec<i32>, Self::Err>>()?;
            Ok(Self(numbers))
        }
    }

    struct Range(i32, i32);
    impl FromStr for Range {
        type Err = ParseIntError;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let mut split = s.splitn(2, "-");
            let low: i32 = split.next().unwrap().parse()?;
            let high: i32 = split.next().unwrap().parse()?;
            Ok(Self(low, high))
        }
    }
    impl Range {
        fn contains(&self, n: &i32) -> bool {
            self.0 <= *n && *n <= self.1
        }
    }

    struct Rule {
        name: String,
        low_range: Range,
        high_range: Range,
        possibly: Vec<bool>,
    }
    impl FromStr for Rule {
        type Err = ParseError;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let (name, ranges): (&str, &str) = s.splitn(2, ": ").collect_tuple().unwrap();
            let (low, high): (&str, &str) = ranges.splitn(2, " or ").collect_tuple().unwrap();
            let low_range = Range::from_str(low).unwrap();
            let high_range = Range::from_str(high).unwrap();
            Ok(Self{name: String::from(name), low_range, high_range, possibly: Vec::new()})
        }
    }
    impl Rule {
        fn allows(&self, n: &i32) -> bool {
            self.low_range.contains(n) || self.high_range.contains(n)
        }
        fn set_maybe(&mut self, l: usize) {
            self.possibly = vec![true;l]
        }
        fn set_no(&mut self, which: usize) {
            self.possibly[which] = false
        }
        fn definite(&self) -> Option<usize> {
            match self.get_maybe_count() {
                1 => self.possibly.iter().position(|v|*v),
                _ => None
            }
        }
        fn get_maybe_count(&self) -> usize {
            return self.possibly.iter().map(|b| *b as usize).sum()
        }
    }

    struct Ruleset(Vec<Rule>);
    impl Ruleset {
        fn error_count(&self, ticket: &Ticket) -> i32 {
            let mut count = 0;
            for value in ticket.0.iter() {
                if self.0.iter().any(|r| r.allows(value)) {
                    continue
                }
                count += value
            }
            count
        }
    }

    enum Phase {Rules, Mine, Theirs}
    struct ProblemParseState(Phase);
    impl ProblemParseState {
        fn advance(&mut self) {
            self.0 = match self.0 {
                Phase::Rules => Phase::Mine,
                _ => Phase::Theirs,
            }
        }
        fn new() -> Self {
            Self(Phase::Rules)
        }
    }

    struct Problem {
        rules: Ruleset,
        mine: Ticket,
        theirs: Vec<Ticket>,
    }
    impl FromStr for Problem {
        type Err = ParseError;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let mut state = ProblemParseState::new();
            let mut rules: Vec<Rule> = Vec::new();
            let mut mine: Vec<Ticket> = Vec::new();
            let mut theirs: Vec<Ticket> = Vec::new();

            for line in s.lines() {
                if line == "" {state.advance();continue}
                match state.0 {
                    Phase::Rules => rules.push(Rule::from_str(line).unwrap()),
                    _ => {
                        let t = Ticket::from_str(line);
                        match t {
                            Ok(ticket) => {
                                match state.0 {
                                    Phase::Theirs => theirs.push(ticket),
                                    _ => mine.push(ticket),
                                }
                            },
                            _ => (),
                        }
                    }
                }
            }
            let mut problem = Problem{rules: Ruleset(rules), mine: mine.pop().unwrap(), theirs};
            let size = problem.mine.0.len();
            for rule in problem.rules.0.iter_mut() {
                rule.set_maybe(size)
            }
            Ok(problem)

        }
    }
    impl Problem {
        fn clear_invalid_tickets(&mut self) {
            let rules = &self.rules;
            self.theirs.retain(|t| rules.error_count(t)==0);
        }
    }
    pub fn run_one(input: String) {
        let problem = Problem::from_str(&input).unwrap();
        let mut error_rate = 0;
        for ticket in problem.theirs.iter() {
            error_rate += problem.rules.error_count(ticket);
        }
        println!("Error rate: {}", error_rate);
    }

    pub fn run_two(input: String) {
        let mut problem = Problem::from_str(&input).unwrap();
        problem.clear_invalid_tickets();

        for rule in problem.rules.0.iter_mut() {
            for ticket in &problem.theirs {
                for (i, v) in ticket.0.iter().enumerate() {
                    if !rule.allows(v) {
                        rule.set_no(i);
                    }
                }
            }
        }
        let mut result: HashMap<String, usize> = HashMap::new();
        loop {
            let mut which: Option<usize> = None;
            for rule in &problem.rules.0 {
                if result.contains_key(&rule.name) {
                    continue
                }
                match rule.definite() {
                    Some(i) => {
                        result.insert(rule.name.to_string(), i);
                        which = Some(i);
                        break
                    },
                    None => ()
                }
            }
            match which {
                Some(i) => {
                    for rule in problem.rules.0.iter_mut() {
                        match rule.definite() {
                            None => rule.set_no(i),
                            _ => (),
                        }
                    }
                },
                None => break
            }
        }
        let mut total: i64 = 1;
        for (name, index) in result.iter() {
            if name.starts_with("departure") {
                total *= problem.mine.0[*index] as i64;
            }
        }
        println!("My ticket departure product: {}", total);
    }

}