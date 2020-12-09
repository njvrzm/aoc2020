pub mod day_seven {

    use std::collections::HashMap;
    use itertools::Itertools;
    use std::str::FromStr;
    use std::string::ParseError;

    #[derive(Debug)]
    struct Containment {
        bag: String,
        count: u32,
    }
    impl FromStr for Containment {
        type Err = ParseError;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let (count, kind): (&str, &str) = s.trim_end_matches(".").splitn(2, " ").collect_tuple().expect("Failed to parse content");
            Ok(Self{bag: String::from(kind), count: count.parse().expect("Failed to parse bag count")})
        }
    }
    type Ruleset = HashMap<String, Vec<Containment>>;

    fn parse(input: String) ->  Ruleset {
        let mut ruleset: Ruleset = HashMap::new();
        for line in input.lines() {
            let mut contained: Vec<Containment> = Vec::new();
            let (container, contents): (String, String) = line.splitn(2, " bags contain ")
                                                              .map(|s| String::from(s))
                                                              .collect_tuple()
                                                              .expect("Unrecognized line format");
            for content in contents.trim_end_matches(".").split(", ") {
                let bag_name = content.trim_end_matches(" bag").trim_end_matches(" bags");
                if bag_name != "no other" {
                    contained.push(Containment::from_str(bag_name).unwrap());
                }
            }
            ruleset.insert(container, contained);
        }
        ruleset
    }


    fn number_inside(sought: &String, candidate: &String, ruleset: &Ruleset) -> u32 {
        let mut count: u32 = 0;
        for bag in ruleset.get(candidate.as_str()).expect("What the heck?") {
            if &bag.bag == sought {
                count += 1
            } else {
                count += number_inside(sought, &bag.bag, &ruleset);
            }
        }
        count
    }
    fn total_bags_inside(bag_name: &String, ruleset: &Ruleset) -> u32 {
        let mut count: u32 = 0;
        let contents = ruleset.get(bag_name).expect("oh no");
        for content in contents {
            count += content.count * (total_bags_inside(&content.bag, ruleset) + 1);
        }
        println!("Total bags inside {}: {}", bag_name, count);
        count

    }
    pub fn run_one(input: String) {
        let ruleset = parse(input);
        let mut count:u32 = 0;
        for candidate in ruleset.keys() {
            match number_inside(&String::from("shiny gold"), candidate, &ruleset) {
                0 => (),
                _ => count += 1,
            }
        }
        println!("Bags with shiny gold inside: {}", count)
    }
    pub fn run_two(input: String) {
        let ruleset = parse(input);
        println!("Bags inside shiny gold: {}", total_bags_inside(&String::from("shiny gold"), &ruleset));
    }
}