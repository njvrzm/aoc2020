pub mod day_six {
    use std::str::FromStr;
    use std::string::ParseError;
    use std::ops::{Index, IndexMut};

    const LOWERCASE_A: u32 = 'a' as u32;
    const LOWERCASE_Z: u32 = 'z' as u32;
    struct Yes {
        answered: [u32;26],
    }
    impl Yes {
        fn new(on: bool) -> Yes {
            if on {
                Yes { answered: [1; 26] }
            } else {
                Yes { answered: [0; 26] }
            }
        }
        fn count(&self) -> u32 {
            return self.answered.iter().sum();
        }
    }
    impl Index<usize> for Yes {
        type Output = u32;

        fn index(&self, index: usize) -> &Self::Output {
            return &self.answered[index];
        }
    }
    impl IndexMut<usize> for Yes {
        fn index_mut(&mut self, index: usize) -> &mut Self::Output {
            return &mut self.answered[index];
        }
    }
    impl FromStr for Yes {
        type Err = ParseError;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let mut ans = Yes::new(false);
            for char in s.chars().map(|c| c as u32) {
                if LOWERCASE_A <= char && char <= LOWERCASE_Z {
                    ans[(char-LOWERCASE_A) as usize] = 1;
                }
            }
            return Ok(ans);
        }
    }
    pub fn run_one(input: String) {
        let mut c: u32 = 0;
        for chunk in input.split("\n\n") {
            c += Yes::from_str(chunk).unwrap().count();
        }
        println!("Weird result: {}", c);
    }
    pub fn run_two(input: String) {
        let mut c: u32 = 0;
        for chunk in input.split("\n\n") {
            let mut all = Yes::new(true);
            for line in chunk.lines() {
                let these = Yes::from_str(line).unwrap();
                for (i, yes) in these.answered.iter().enumerate() {
                    if *yes == 0 {
                        all[i] = 0;
                    }
                }
            }
            c += all.count();
        }
        println!("Second weird result: {}", c);

    }
}