#[allow(mutable_borrow_reservation_conflict)]
pub mod day_fifteen {
    use std::collections::HashMap;

    struct Record {
        last: Option<i32>,
        previous: Option<i32>,
    }

    impl Record {
        fn say(&mut self, at: i32) {
            self.previous = Some(self.last.unwrap());
            self.last = Some(at);
        }
    }

    struct History {
        last: i32,
        history: HashMap<i32, Record>,
        tick: i32,
    }

    impl History {
        fn new() -> Self {
            Self { last: 0, tick: 1, history: HashMap::new() }
        }
        fn say(&mut self, n: i32) {
            self.tick += 1;
            match self.history.get_mut(&n) {
                None => { self.history.insert(n, Record { last: Some(self.tick), previous: None }); },
                Some(r) => r.say(self.tick),
            }
            self.last = n;
        }
        fn speak(&mut self) {
            let last_record = self.history.get(&self.last).unwrap();
            match last_record {
                Record { last: Some(_), previous: None } => { self.say(0); },
                Record { last: Some(l), previous: Some(p) } => { self.say(l - p); },
                _ => panic!("Shouldn't be speaking an unrecorded number")
            }
        }
    }

    pub fn run_one(input: String) {
        let mut h = History::new();
        for sn in input.trim().split(",") {
            h.say(sn.parse().unwrap());
        }
        // while h.tick < 2021 {
        while h.tick < 30000001 {
            h.speak();
        }
        println!("{}", h.last);

    }
}