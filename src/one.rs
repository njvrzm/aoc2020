pub mod day_one {
    fn parse_lines(input: String) -> Vec<usize> {
        let mut nums = Vec::new();

        for line in input.lines() {
            nums.push(line.parse::<usize>().unwrap());
        }
        nums.sort();
        nums
    }
    pub fn run_one(input: String) {
        let mut calc = 0;
        let nums = parse_lines(input);
        for one in nums.iter() {
            for two in nums.iter() {
                // These shortcuts take the total number of inner loops from 40622 to 776
                calc += 1;
                if one + two > 2020 {
                    break
                }
                if one + two == 2020 {
                    println!("{} + {} == 2020; {} * {} == {}", one, two, one, two, one * two);
                    println!("Total calculations: {}", calc);
                    return
                }
            }
        }
    }

    pub fn run_two(input: String) {
        let mut nums = Vec::new();

        for line in input.lines() {
            nums.push(line.parse::<usize>().unwrap());
        }
        nums.sort();
        let mut calc = 0;
        for one in nums.iter() {
            for two in nums.iter() {
                // These shortcuts take the total number of inner loops from 40622 to 776
                if one + two > 2020 {
                    break
                }
                for three in nums.iter() {
                    if one + two + three > 2020 {
                        break
                    }
                    calc += 1;
                    if one + two + three == 2020 {
                        println!("{} + {} + {} == 2020; {} * {} * {} == {}", one, two, three, one, two, three, one * two * three);
                        println!("Total calculations: {}", calc);
                        return
                    }
                }
            }
        }

    }
}