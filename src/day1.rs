pub fn solve_part1(input: &str) -> u32 {
    let mut iter = input.lines().map(|s| s.parse::<u32>().expect("not an integer"));
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
