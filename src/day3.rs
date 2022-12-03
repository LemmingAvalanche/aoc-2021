use Freq::*;

#[derive(PartialEq, Debug)]
enum Freq {
    Zeros,
    Ones,
    Equal,
}

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

pub fn solve_part2(input: &str) -> u32 {
    let max_index = input.lines().nth(0).unwrap().len() - 1;
    let count_numbers = input.lines().count();
    let vector = input
        .lines()
        .map(|s| u16::from_str_radix(s, 2).unwrap())
        .collect::<Vec<u16>>();
    let ox = ox_rating(&vector, max_index, count_numbers);
    let co2 = co2_rating(&vector, max_index, count_numbers);
    ox * co2
}

fn ox_rating(input: &Vec<u16>, max_index: usize, count_numbers: usize) -> u32 {
    let mut remaining = input.clone();
    for i in 0..=max_index {
        if remaining.len() == 1 {
            break;
        }
        let (freq, hits) = frequency(&remaining, i, max_index, remaining.len());
        if freq == Ones || freq == Equal {
            // NOTE duplicate code (e.g.)
            remaining.retain(|num| ((*num >> (max_index - i)) % 2) == 1);
        } else {
            remaining.retain(|num| ((*num >> (max_index - i)) % 2) == 0);
        }
    }
    assert!(remaining.len() == 1);
    remaining[0] as u32
}

fn co2_rating(input: &Vec<u16>, max_index: usize, count_numbers: usize) -> u32 {
    let mut remaining = input.clone();
    for i in 0..=max_index {
        if remaining.len() == 1 {
            break;
        }
        let (freq, hits) = frequency(&remaining, i, max_index, remaining.len());
        if freq == Zeros {
            remaining.retain(|num| ((*num >> (max_index - i)) % 2) == 1);
        } else {
            remaining.retain(|num| !(((*num >> (max_index - i)) % 2) == 1));
        }
    }
    assert!(remaining.len() == 1);
    remaining[0] as u32
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

fn frequency(
    input: &[u16],
    position: usize,
    max_index: usize,
    count_numbers: usize,
) -> (Freq, Vec<bool>) {
    let ones = input
        .iter()
        .map(|num| ((*num >> (max_index - position)) % 2) == 1)
        .collect::<Vec<bool>>();
    let count1s = ones.iter().filter(|t| **t).count();
    let compare = count_numbers / 2;
    let freq = if count1s == compare && (count_numbers % 2 == 0) {
        Equal
    } else if count1s == compare && (count_numbers % 2 == 1) {
        Zeros
    } else if count1s > compare {
        Ones
    } else {
        Zeros
    };
    (freq, ones)
}
