pub mod day_five {
    fn seat_id(code: &str) -> u32 {
        let binary = code.replace("B", "1")
                                 .replace("F", "0")
                                 .replace("L", "0")
                                 .replace("R", "1");
        return u32::from_str_radix(&binary, 2).unwrap()
    }
    fn get_codes(input: String) -> Vec<u32> {
        let mut codes = input.lines().map(|l| seat_id(l)).collect::<Vec<u32>>();
        codes.sort();
        return codes
    }
    pub fn run_one(input: String) {
        let codes = get_codes(input);
        println!("Max code: {}", codes.last().unwrap());
    }
    pub fn run_two(input: String) {
        let codes = get_codes(input);
        let mut last = codes[0];
        for code in codes {
            if code > last + 1 {
                println!("Missing code: {}", code - 1);
                break
            }
            last = code;
        }
    }
}