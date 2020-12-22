pub mod day_eighteen {
    use std::str::FromStr;
    use std::string::ParseError;

    #[derive(Copy,Clone,Debug)]
    enum Symbol {
        Open,
        Close,
        Add,
        Mul,
        Number(i64),
    }
    impl FromStr for Symbol {
        type Err = ParseError;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            match s {
                "*" => Ok(Self::Mul),
                "+" => Ok(Self::Add),
                "(" => Ok(Self::Open),
                ")" => Ok(Self::Close),
                s if s.chars().all(char::is_numeric) => Ok(Self::Number(s.parse().unwrap())),
                _ => panic!("Unrecognized symbol!")
            }
        }
    }
    struct Stack {
        stack: Vec<Symbol>,
    }
    impl Stack {
        fn new() -> Self {
            Stack{stack: Vec::new()}
        }
        fn top(&self) -> Option<&Symbol> {
            self.stack.last()
        }
        fn push(&mut self, s: Symbol) {
            match (self.top(), s) {
                (None, _) => self.stack.push(s),
                (Some(Symbol::Number(_)), Symbol::Mul) => self.stack.push(s),
                (Some(Symbol::Number(_)), Symbol::Add) => self.stack.push(s),
                (Some(Symbol::Mul), Symbol::Open) => self.stack.push(s),
                (Some(Symbol::Add), Symbol::Open) => self.stack.push(s),
                (Some(Symbol::Open), Symbol::Open) => self.stack.push(s),
                (Some(Symbol::Number(_)), Symbol::Close) => self.close(),
                (Some(Symbol::Add), Symbol::Number(n) ) => self.add(n),
                (Some(Symbol::Mul), Symbol::Number(n) ) => self.mul(n),
                (Some(Symbol::Open), Symbol::Number(_) ) => self.stack.push(s),
                _ => panic!("Unexpected symbol {:?} with top {:?}", s, self.top().unwrap())
            }
        }
        fn add(&mut self, n: i64) {
            self.stack.pop(); // add symbol
            if let Symbol::Number(m) = self.stack.pop().unwrap() {
                self.push(Symbol::Number(m+n));
            } else {
                panic!("Non-number under multiply")
            }
        }
        fn mul(&mut self, n: i64) {
            self.stack.push(Symbol::Number(n));
            // The below was part 1's implementation of mul
            //
            // self.stack.pop(); // mul symbol
            // if let Symbol::Number(m) = self.stack.pop().unwrap() {
            //     self.stack.push(Symbol::Number(m*n));
            // } else {
            //     panic!("Non-number under add")
            // }
        }
        fn close(&mut self) {
            let trace = format!("{:?}", self.stack);
            // This was modified for part 2; in the part 1 implementation
            // it simply popped the number and open paren off and pushed
            // the number back on
            let mut product: i64 = 1;
            loop {
                match self.stack.pop() {
                    Some(Symbol::Number(n)) => product *= n,
                    Some(Symbol::Open) => break,
                    Some(Symbol::Mul) => (),
                    None => break,
                    _ => panic!("Unexpected symbol during close: {:?}", trace),
                }
            }
            self.push(Symbol::Number(product));
        }
        fn take(&mut self, ch: char) {
            match ch {
                '(' => self.push(Symbol::Open),
                ')' => self.push(Symbol::Close),
                '*' => self.push(Symbol::Mul),
                '+' => self.push(Symbol::Add),
                ch if ch.is_numeric() => self.push(Symbol::Number(ch.to_string().parse().unwrap())),
                ' ' => (),
                _ => panic!("Unrecognized character {}", ch),
            }

        }
    }

    pub fn run_one(input: String) {
        let mut total: i64 = 0;
        for line in input.lines() {
            let mut stack = Stack::new();
            for ch in line.chars() {
                stack.take(ch);
            }
            if stack.stack.len() > 1 {
                stack.close()
            }
            if let Some(Symbol::Number(result)) = stack.top() {
                total += result
            } else {
                panic!("Stack top is not a number!")
            }
        }
        println!("Total result: {}", total);

    }
}