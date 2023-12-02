use std::str::FromStr;

fn task_one(input: &[String]) -> usize {
    input.iter().fold(0, |acc, x| {
        [
            x.chars().find_map(|c| c.to_digit(10)).unwrap_or(0) as usize,
            x.chars().rev().find_map(|c| c.to_digit(10)).unwrap_or(0) as usize,
        ]
        .iter()
        .fold(0, |n_acc, it| n_acc * 10 + it)
            + acc
    })
}

#[derive(Debug)]
struct Calibration {
    first: usize,
    second: usize,
}

impl FromStr for Calibration {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let chars = s.chars().collect::<Vec<char>>();
        let window_len = if chars.len() >= 5 { 5 } else { chars.len() };
        let first_try = chars.windows(window_len).find_map(|s| match s {
            ['o', 'n', 'e', ..] => Some(1),
            ['t', 'w', 'o', ..] => Some(2),
            [x, 'o', 'n', 'e', ..] => Some(x.to_digit(10).unwrap_or(1)),
            [x, 't', 'w', 'o', ..] => Some(x.to_digit(10).unwrap_or(2)),
            ['t', 'h', 'r', 'e', 'e'] => Some(3),
            ['f', 'o', 'u', 'r', ..] => Some(4),
            ['f', 'i', 'v', 'e', ..] => Some(5),
            ['s', 'i', 'x', ..] => Some(6),
            [x, 's', 'i', 'x', ..] => Some(x.to_digit(10).unwrap_or(6)),
            ['s', 'e', 'v', 'e', 'n'] => Some(7),
            ['e', 'i', 'g', 'h', 't'] => Some(8),
            ['n', 'i', 'n', 'e', ..] => Some(9),
            _ => s.iter().find_map(|c| c.to_digit(10)),
        });

        let second_try = chars.windows(window_len).rev().find_map(|s| match s {
            [.., 'o', 'n', 'e'] => Some(1),
            [.., 't', 'w', 'o'] => Some(2),
            [.., 'o', 'n', 'e', x] => Some(x.to_digit(10).unwrap_or(1)),
            [.., 't', 'w', 'o', x] => Some(x.to_digit(10).unwrap_or(2)),
            ['t', 'h', 'r', 'e', 'e'] => Some(3),
            [.., 'f', 'o', 'u', 'r'] => Some(4),
            [.., 'f', 'i', 'v', 'e'] => Some(5),
            [.., 's', 'i', 'x'] => Some(6),
            [.., 's', 'i', 'x', x] => Some(x.to_digit(10).unwrap_or(6)),
            ['s', 'e', 'v', 'e', 'n'] => Some(7),
            ['e', 'i', 'g', 'h', 't'] => Some(8),
            [.., 'n', 'i', 'n', 'e'] => Some(9),
            _ => s.iter().rev().find_map(|c| c.to_digit(10)),
        });

        let first = first_try.unwrap_or_else(|| second_try.unwrap()) as usize;
        let second = second_try.unwrap_or_else(|| first_try.unwrap()) as usize;

        Ok(Calibration { first, second })
    }
}

fn task_two(input: &[String]) -> usize {
    input.iter().fold(0, |acc, s| {
        let c = Calibration::from_str(s).unwrap();
        acc + c.first * 10 + c.second
    })
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
    let elapsed = t.elapsed().as_millis();

    match task {
        Task::One => {
            println!("({}ms)\tTask one: \x1b[0;34;34m{}\x1b[0m", elapsed, res);
        }
        Task::Two => {
            println!("({}ms)\tTask two: \x1b[0;33;10m{}\x1b[0m", elapsed, res);
        }
    };
}

fn get_input_file() -> String {
    std::env::args()
        .nth(1)
        .unwrap_or_else(|| "input".to_string())
}
