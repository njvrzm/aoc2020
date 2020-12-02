pub mod day_one {
    pub fn run(input: String) {
        let mut nums = Vec::new();

        for line in input.lines() {
            nums.push(line.parse::<i32>().unwrap());
        }
        nums.sort();
        let mut calc = 0;
        for one in nums.iter() {
            for two in nums.iter() {
                // These shortcuts take the total number of inner loops from 40622 to 776
                if one + two > 2020 {
                    continue
                }
                for three in nums.iter() {
                    if one + two + three > 2020 {
                        continue
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