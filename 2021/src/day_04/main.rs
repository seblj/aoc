#[derive(Clone, Debug)]
struct Bingo {
    numbers: Vec<usize>,
    boards: Vec<Board>,
}

#[derive(Clone, Debug)]
struct Board {
    board: Vec<Vec<Position>>,
}

#[derive(Clone, Debug)]
struct Position {
    number: usize,
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

    fn get_sum(&self) -> usize {
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

    fn mark_number(&mut self, number: usize) -> bool {
        for row in &mut self.board {
            for n in row {
                if number == n.number {
                    n.hit = true;
                    return true;
                }
            }
        }
        false
    }
}

fn get_bingo_boards(vec: &[String]) -> Bingo {
    let numbers: Vec<_> = vec[0]
        .split(',')
        .map(|x| x.parse::<usize>().unwrap())
        .collect();

    let mut boards: Vec<Board> = Vec::new();
    let mut board = Vec::new();
    for v in vec.iter().skip(2) {
        let row: Vec<Position> = v
            .split_whitespace()
            .map(|c| Position {
                number: c.parse::<usize>().unwrap(),
                hit: false,
            })
            .collect();
        if !row.is_empty() {
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

fn task_one(input: &[String]) -> usize {
    let mut bingo = get_bingo_boards(input);
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

fn task_two(input: &[String]) -> usize {
    let mut bingo = get_bingo_boards(input);
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

fn main() {
    let input = read_input(get_input_file());
    time(Task::One, task_one, &input);
    time(Task::Two, task_two, &input);
}

fn read_input<P>(path: P) -> Vec<String>
where
    P: AsRef<std::path::Path>,
{
    std::fs::read_to_string(path)
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}

enum Task {
    One,
    Two,
}

fn time<F, T, U>(task: Task, f: F, arg: T)
where
    F: Fn(T) -> U,
    U: std::fmt::Display,
{
    let t = std::time::Instant::now();
    let res = f(arg);
    let elapsed = t.elapsed();
    let fmt = std::env::var("TASKUNIT").unwrap_or("ms".to_owned());

    let (u, elapsed) = match fmt.as_str() {
        "ms" => ("ms", elapsed.as_millis()),
        "ns" => ("ns", elapsed.as_nanos()),
        "us" => ("μs", elapsed.as_micros()),
        "s" => ("s", elapsed.as_secs() as u128),
        _ => panic!("unsupported time format"),
    };

    match task {
        Task::One => {
            println!("({}{u})\tTask one: \x1b[0;34;34m{}\x1b[0m", elapsed, res);
        }
        Task::Two => {
            println!("({}{u})\tTask two: \x1b[0;33;10m{}\x1b[0m", elapsed, res);
        }
    };
}

fn get_input_file() -> String {
    std::env::args()
        .nth(1)
        .unwrap_or_else(|| "input".to_string())
}
