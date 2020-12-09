pub mod day_nine {
    use itertools::Itertools;

    fn sums_of_pairs(numbers: &[i64]) -> Vec<i64> {
        let mut sums= Vec::with_capacity(numbers.len()*(numbers.len()-1)/2);
        for i in 0..numbers.len() {
            for j in i+1..numbers.len() {
                if numbers[i] != numbers[j] {
                    sums.push(numbers[i] + numbers[j]);
                }
            }
        }
        sums
    }
    pub fn run_one(input: String) {
        let numbers: Vec<i64> = input.lines().map(|l| l.parse().expect("Bad number")).collect_vec();
        for i in 26..numbers.len() {
            let sums = sums_of_pairs(&numbers[i-25..i]);
            if !sums.contains(&numbers[i]) {
                println!("No sum for {}", numbers[i]);

            }
        }

    }
    pub fn run_two(input: String) {
        let numbers: Vec<i64> = input.lines().map(|l| l.parse().expect("Bad number")).collect_vec();
        let mut running_sum: i64 = 0;
        let mut top: usize = 0;
        let mut bottom: usize = 0;
        while running_sum != 1212510616 {
            if bottom > 0 {
                running_sum -= numbers[bottom]
            }
            bottom += 1;
            while running_sum < 1212510616 {
                top += 1;
                running_sum += numbers[top];
            }
        }
        let small = numbers[bottom..top].iter().min().unwrap();
        let large = numbers[bottom..top].iter().max().unwrap();
        println!("Found it: {}", small + large)
    }
}