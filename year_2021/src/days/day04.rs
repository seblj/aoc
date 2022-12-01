use aoc::{read_input, time};
use std::path::Path;

pub fn solve() {
    let input = "input";
    let file = Path::new(file!()).file_stem().unwrap().to_str().unwrap();
    let input = format!(
        "{}/src/days/input/{}.{}",
        env!("CARGO_MANIFEST_DIR"),
        file,
        input
    );

    let vec: Vec<String> = read_input(input);
    let bingo = get_bingo_boards(&vec);

    time("one", task_one, bingo.clone());
    time("two", task_two, bingo.clone());
}

#[derive(Clone, Debug)]
struct Bingo {
    numbers: Vec<i32>,
    boards: Vec<Board>,
}

#[derive(Clone, Debug)]
struct Board {
    board: Vec<Vec<Position>>,
}

#[derive(Clone, Debug)]
struct Position {
    number: i32,
    hit: bool,
}

impl Board {
    fn has_bingo(&self) -> bool {
        for i in 0..5 {
            let bingo =
                (0..5).all(|j| self.board[i][j].hit) || (0..5).all(|j| self.board[j][i].hit);
            if bingo {
                return true;
            }
        }
        false
    }

    fn get_sum(&self) -> i32 {
        let mut sum = 0;
        for i in 0..5 {
            for j in 0..5 {
                if !self.board[j][i].hit {
                    sum += self.board[j][i].number;
                }
            }
        }
        sum
    }

    fn mark_number(&mut self, number: i32) -> bool {
        for row in &mut self.board {
            for mut n in row {
                if number == n.number {
                    n.hit = true;
                    return true;
                }
            }
        }
        false
    }
}

fn get_bingo_boards(vec: &Vec<String>) -> Bingo {
    let numbers: Vec<i32> = vec[0]
        .split(',')
        .map(|x| x.parse::<i32>().unwrap())
        .collect();

    let mut boards: Vec<Board> = Vec::new();
    let mut board = Vec::new();
    for v in vec.iter().skip(2) {
        let row: Vec<Position> = v
            .split_whitespace()
            .map(|c| Position {
                number: c.parse::<i32>().unwrap(),
                hit: false,
            })
            .collect();
        if row.len() > 0 {
            board.push(row.clone());
        }
        if board.len() == 5 {
            boards.push(Board {
                board: board.clone(),
            });
            board.clear();
        }
    }
    Bingo { numbers, boards }
}

fn task_one(mut bingo: Bingo) -> i32 {
    for number in bingo.numbers {
        for board in &mut bingo.boards {
            if !board.mark_number(number) {
                continue;
            }
            if board.has_bingo() {
                return board.get_sum() * number;
            }
        }
    }
    unreachable!();
}

fn task_two(mut bingo: Bingo) -> i32 {
    for number in bingo.numbers {
        let mut remaining = Vec::new();
        let len = bingo.boards.len();

        for mut board in bingo.boards.into_iter() {
            if !board.mark_number(number) {
                remaining.push(board);
                continue;
            }

            if !board.has_bingo() {
                remaining.push(board);
            } else if len == 1 {
                return board.get_sum() * number;
            }
        }
        bingo.boards = remaining;
    }
    unreachable!();
}
