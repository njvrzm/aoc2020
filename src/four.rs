
pub mod day_four {

    use std::collections::HashSet;
    use itertools::Itertools;

    pub fn run_one(input: String) {
        let required:HashSet<&str> = hashset!{"byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"};
        let mut good = 0;
        for chunk in input.split("\n\n") {
            let mut unseen = required.clone();
            for part in chunk.split_ascii_whitespace() {
                let (key, _val) = part.splitn(2, ":").collect_tuple().unwrap();
                let _ = unseen.remove(key);
            }
            if unseen.is_empty(){
                good += 1;
            }
        }
        println!("Total good passports: {}", good);
    }
}