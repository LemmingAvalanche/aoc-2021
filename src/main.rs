static input: &str = include_str!("/home/kristoffer/programming/advent-of-code-2021/input-text.txt");

fn main() {
    // I just update this function for each problem that I bother to solve,
    // setting the input file path and the function call.
    println!("{}", advent_of_code_2021::day1::solve_part1(input));
}
