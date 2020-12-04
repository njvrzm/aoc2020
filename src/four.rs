
pub mod day_four {

    use std::collections::{HashSet, HashMap};
    use itertools::Itertools;
    use std::string::ParseError;
    use std::str::FromStr;
    use regex::Regex;
    use lazy_static::lazy_static;

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

    pub fn run_two(input: String) {
        let mut valid = 0;
        for chunk in input.split("\n\n") {
            let passport = chunk.parse::<Passport>().unwrap();
            if passport.is_valid() {
                valid += 1
            }
        }
        println!("Total valid passports: {}", valid);
    }
    struct Passport {
        birth_year: BirthYear,
        issue_year: IssueYear,
        expiration_year: ExpirationYear,
        height: Height,
        hair_color: HairColor,
        eye_color: EyeColor,
        passport_id: PassportId,
        country_id: CountryId,
    }
    impl FromStr for Passport {
        type Err = ParseError;
        fn from_str(input: &str) -> Result<Self, Self::Err> {
            let mut parts: HashMap<String, String> = HashMap::new();
            for part in input.split_whitespace() {
                // let (key, val): (&str, &str) = part.splitn(2, ":").collect_tuple::<(&str, &str)>().unwrap();
                let (key, val) = part.splitn(2, ":").collect_tuple().unwrap();
                parts.insert(key.to_string(), val.to_string());
            }
            return Ok(Passport{
                birth_year: BirthYear::new(parts.get("byr")),
                issue_year: IssueYear::new(parts.get("iyr")),
                expiration_year: ExpirationYear::new(parts.get("eyr")),
                height: Height::new(parts.get("hgt")),
                hair_color: HairColor::new(parts.get("hcl")),
                eye_color: EyeColor::new(parts.get("ecl")),
                passport_id: PassportId::new(parts.get("pid")),
                country_id: CountryId::new(parts.get("cid")),
            })
        }
    }
    impl Passport {
        fn is_valid(self) -> bool {
            return self.birth_year.is_valid()
                   && self.issue_year.is_valid()
                   && self.expiration_year.is_valid()
                   && self.height.is_valid()
                   && self.eye_color.is_valid()
                   && self.hair_color.is_valid()
                   && self.passport_id.is_valid()
                   && self.country_id.is_valid();
        }
    }
    struct BirthYear {
        value: Option<i32>,
    }
    impl BirthYear {
        fn new(value: Option<&String>) -> Self {
            return Self{value: value.map(|s| s.parse::<i32>().unwrap())};
        }
        fn is_valid(&self) -> bool {
            match self.value {
                None => false,
                Some(i) => (1920 <= i) && (i <= 2002),
            }
        }
    }
    struct IssueYear {
        value: Option<i32>,
    }
    impl IssueYear {
        fn new(value: Option<&String>) -> Self {
            return Self{value: value.map(|s| s.parse::<i32>().unwrap())};
        }
        fn is_valid(&self) -> bool {
            match self.value {
                None => false,
                Some(i) => (2010 <= i) && (i <= 2020),
            }
        }
    }
    enum LengthUnit {
        Inches,
        Centimeters,
    }
    struct Height {
        value: Option<i32>,
        unit: Option<LengthUnit>,
    }
    impl Height {
        fn new(value: Option<&String>) -> Self {
            match value {
                Some(s) if s.ends_with("cm") =>
                    Self{value: Some(s[0..s.len()-2].parse::<i32>().unwrap()), unit: Some(LengthUnit::Centimeters) },
                Some(s) if s.ends_with("in") =>
                    Self{value: Some(s[0..s.len()-2].parse::<i32>().unwrap()), unit: Some(LengthUnit::Inches)},
                _ => Self{value: None, unit: None}, // hacky!
            }
        }
        fn is_valid(&self) ->bool {
            match self.unit {
                None => false,
                Some(LengthUnit::Centimeters) => self.value.unwrap() >= 150 && self.value.unwrap() <= 193,
                Some(LengthUnit::Inches) => self.value.unwrap() >= 59 && self.value.unwrap() <= 76,
            }
        }
    }
    struct ExpirationYear {
        value: Option<i32>,
    }
    impl ExpirationYear {
        fn new(value: Option<&String>) -> Self {
            return Self{value: value.map(|s| s.parse::<i32>().unwrap())};
        }
        fn is_valid(&self) -> bool {
            match self.value {
                None => false,
                Some(i) => (2020 <= i) && (i <= 2030),
            }
        }
    }
    struct HairColor {
        value: Option<String>,
    }
    lazy_static! {
        static ref HAIR_MATCH:Regex = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
    }
    impl HairColor {
        fn new(value: Option<&String>) -> Self {
            return Self{value: value.map(|s| s.clone())}
        }
        fn is_valid(&self) -> bool {
            match &self.value {
                None => false,
                Some(s) => HAIR_MATCH.is_match(s.as_str()),
            }
        }
    }
    struct EyeColor {
        value: Option<String>,
    }
    lazy_static! {
        static ref EYE_MATCH: Regex = Regex::new("amb|blu|brn|gry|grn|hzl|oth").unwrap();
    }
    impl EyeColor {
        fn new(value: Option<&String>) -> Self {
            return Self{value: value.map(|s| s.clone())}
        }
        fn is_valid(&self) -> bool {
            match &self.value {
                None => false,
                Some(s) => EYE_MATCH.is_match(s.as_str()),
            }
       }
    }
    struct PassportId {
        value: Option<String>,
    }
    lazy_static! {
        static ref PASSPORT_ID_MATCH: Regex = Regex::new("^[0-9]{9}$").unwrap();
    }
    impl PassportId {
        fn new(value: Option<&String>) -> Self {
            return Self{value: value.map(|s| s.clone())}
        }
        fn is_valid(&self) -> bool {
            match &self.value {
                None => false,
                Some(s) => PASSPORT_ID_MATCH.is_match(s.as_str()),
            }
        }
    }
    struct CountryId {
        _value: Option<String>,
    }
    impl CountryId {
        fn new(value: Option<&String>) -> Self {
            return Self{_value: value.map(|s| s.clone())}
        }
        fn is_valid(&self) -> bool {
            return true
        }
    }

}