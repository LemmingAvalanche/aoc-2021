use regex::Regex;

use Dir::*;

#[derive(Debug)]
struct Command {
    dir: Dir,
    value: u32,
}

#[derive(Debug)]
enum Dir {
    Up,
    Down,
    Forward
}

struct Navigation {
    horiz: u32,
    depth: u32,
}

impl Navigation {
    fn new() -> Self {
        Navigation { horiz: 0, depth: 0 }
    }

    fn nav(self, command: Command) -> Self {
        match command.dir {
            Up => Navigation { horiz: self.horiz, depth: self.depth - command.value },
            Down => Navigation { horiz: self.horiz, depth: self.depth + command.value },
            Forward => Navigation { horiz: self.horiz + command.value, depth: self.depth },
        }
    }
    fn result(self) -> u32 {
        self.horiz * self.depth
    }
}

impl Command {
    pub fn from_str(input: &str) -> Self {
        lazy_static::lazy_static! {
            static ref RE_U: Regex = Regex::new(r"up (\d*)").unwrap();
            static ref RE_D: Regex = Regex::new(r"down (\d*)").unwrap();
            static ref RE_F: Regex = Regex::new(r"forward (\d*)").unwrap();
        }

        if let Some(num) = RE_U.captures(input) {
            Command { dir: Up, value: try_parse_int(&num[1]) }
        } else if let Some(num) = RE_D.captures(input) {
            Command { dir: Down, value: try_parse_int(&num[1]) }
        } else if let Some(num) = RE_F.captures(input) {
            Command { dir: Forward, value: try_parse_int(&num[1]) }
        } else {
            panic!("bad input")
        }
    }
}

fn try_parse_int(i: &str) -> u32 {
    i.parse::<u32>().expect("not an integer")
}

pub fn solve_part1(input: &str) -> u32 {
    input
        .lines()
        .map(|l| Command::from_str(l))
        .fold(Navigation::new(), |acc, c| acc.nav(c))
        .result()
}

// pub fn solve_part2(input: &str) -> u32 {

// }
