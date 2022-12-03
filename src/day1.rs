pub fn solve_part1(input: &str) -> u32 {
    let mut iter = input
        .lines()
        .map(|s| s.parse::<u32>().expect("not an integer"));
    let mut prevm = iter.next().unwrap();
    let mut counter: u32 = 0;
    for num in iter {
        if num > prevm {
            counter = counter + 1;
        }
        prevm = num;
    }
    counter
}

pub fn solve_part2(input: &str) -> u32 {
    let mut iter = input
        .lines()
        .map(|s| s.parse::<u32>().expect("not an integer"));
    let mut first = iter.next().unwrap();
    let mut second = iter.next().unwrap();
    let mut third = iter.next().unwrap();
    let mut sum = first + second + third;
    let mut counter: u32 = 0;
    for num in iter {
        first = second;
        second = third;
        third = num;
        let new_sum = first + second + third;
        if new_sum > sum {
            counter = counter + 1;
        }
        sum = new_sum;
    }
    counter
}

pub fn solve_part2_alt1(input: &str) -> u32 {
    // Create temporary to avoid  E0716
    let binding = input
        .lines()
        .map(|s| s.parse::<u32>().expect("not an integer"))
        .collect::<Vec<u32>>();
    let mut iter = binding.windows(3);
    let mut first = iter.next().unwrap();
    let mut sum: u32 = first.iter().sum();
    let mut counter: u32 = 0;
    for window in iter {
        let new_sum: u32 = window.iter().sum();
        if new_sum > sum {
            counter = counter + 1;
        }
        sum = new_sum;
    }
    counter
}
