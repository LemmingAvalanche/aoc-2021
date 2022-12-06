// NOTE I would have liked to use this rather than `(u8, bool)`
// use bitvec::prelude::*;
// type State = BitArr!(for 25, in u32, Msb0);

const SIZE: usize = 5;

struct Game {
    numbers: Vec<u8>,
    boards: Vec<BoardState>,
}

#[derive(Debug)]
struct BoardState {
    board: [[(u8, bool); SIZE]; SIZE],
}

impl Game {
    pub fn from_str(input: &str) -> Self {
        let (num, boards_str) = input.split_once("\n\n").unwrap();
        let numbers: Vec<u8> = num.split(",").map(|n| try_parse_int(n)).collect();
        let boards = boards_str
            .split("\n\n")
            .map(|b| Self::make_board_from_str(b))
            .collect::<Vec<BoardState>>();
        Game { numbers: numbers, boards: boards }
    }

    fn make_board_from_str(input: &str) -> BoardState {
        let mut array = [[(0_u8, false); SIZE]; SIZE];
        for (i, l) in input.lines().enumerate() {
            for (j, num) in l.split_whitespace().enumerate() {
                array[i][j] = (try_parse_int(num), false);
            }
        }
        BoardState { board: array }
    }

}

impl BoardState {
    pub fn mark_numbers(&mut self, num: u8) {
        for i in 0..SIZE {
            for j in 0..SIZE {
                if num == self.board[i][j].0 {
                    self.board[i][j].1 = true;
                }
            }
        }
    }

    fn sum_unmarked(&self) -> u32 {
        let mut sum: u32 = 0;
        for i in 0..SIZE {
            for j in 0..SIZE {
                if !self.board[i][j].1 {
                    sum = sum + (self.board[i][j].0 as u32);
                }
            }
        }
        sum
    }

    fn has_won(&self) -> bool {
        for i in 0..SIZE {
            let mut row = true;
            let mut column = true;
            for j in 0..SIZE {
                row = row && self.board[i][j].1;
                column = column && self.board[j][i].1;
            }
            if row || column {
                return true;
            }
        }
        false
    }
}

pub fn solve_part1(input: &str) -> u32 {
    let mut game = Game::from_str(&input);
    for n in game.numbers {
        for b in game.boards.iter_mut() {
            b.mark_numbers(n);
            if b.has_won() {
                return b.sum_unmarked() * n as u32;
            }
        }
    }
    panic!("exhausted input!")
}

fn try_parse_int(i: &str) -> u8 {
    i.parse::<u8>().expect("not an integer")
}
