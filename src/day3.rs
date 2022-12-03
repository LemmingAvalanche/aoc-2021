pub fn solve_part1(input: &str) -> u32 {
    let max_index = input.lines().nth(0).unwrap().len() - 1;
    let count_numbers = input.lines().count();
    let vector = input
        .lines()
        .map(|s| u16::from_str_radix(s, 2).unwrap())
        .collect::<Vec<u16>>();
    let γ = build_gamma(&vector, max_index, count_numbers);
    let ε = build_epsilon(&vector, max_index, count_numbers);
    γ * ε
}

fn build_gamma(input: &Vec<u16>, max_index: usize, count_numbers: usize) -> u32 {
    let mut γ = 0;
    for i in 0..=max_index {
        γ = γ
            + if mostly_1s(&input, i, max_index, count_numbers) {
                2_u32.pow((max_index - i).try_into().unwrap())
            } else {
                0
            }
    }
    γ
}

fn build_epsilon(input: &Vec<u16>, max_index: usize, count_numbers: usize) -> u32 {
    let mut ε = 0;
    for i in 0..=max_index {
        ε = ε
            + if mostly_1s(&input, i, max_index, count_numbers) {
                0
            } else {
                2_u32.pow((max_index - i).try_into().unwrap())
            }
    }
    ε
}

/// position starts at 0
fn mostly_1s(input: &Vec<u16>, position: usize, max_index: usize, count_numbers: usize) -> bool {
    let count1s = input
        .into_iter()
        .filter(|num| ((*num >> (max_index - position)) % 2) == 1)
        .count();
    count1s > (count_numbers / 2)
}
