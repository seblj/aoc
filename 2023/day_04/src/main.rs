use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

struct Game {
    winning_numbers: HashSet<usize>,
    my_numbers: HashSet<usize>,
}

impl FromStr for Game {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (_, numbers) = s.split_once(':').unwrap();
        let (winning_numbers, my_numbers) = numbers.split_once('|').unwrap();

        let winning_numbers = winning_numbers
            .split_whitespace()
            .map(|num| num.parse::<usize>().unwrap())
            .collect::<HashSet<usize>>();

        let my_numbers = my_numbers
            .split_whitespace()
            .map(|num| num.parse::<usize>().unwrap())
            .collect::<HashSet<usize>>();

        Ok(Self {
            winning_numbers,
            my_numbers,
        })
    }
}

fn task_one(input: &[String]) -> usize {
    input
        .iter()
        .map(|s| {
            let game = Game::from_str(s).unwrap();
            let count = game.winning_numbers.intersection(&game.my_numbers).count();
            if count > 0 {
                2_usize.pow(count as u32 - 1)
            } else {
                0
            }
        })
        .sum()
}

fn task_two(input: &[String]) -> usize {
    input
        .iter()
        .enumerate()
        .fold(HashMap::new(), |mut acc, (idx, s)| {
            let game = Game::from_str(s).unwrap();
            let count = game.winning_numbers.intersection(&game.my_numbers).count();
            let current = idx + 1;

            let cards = acc
                .entry(current)
                .and_modify(|e| *e += 1)
                .or_insert(1)
                .to_owned();

            let next = current + 1;
            for i in next..count + next {
                acc.entry(i).and_modify(|e| *e += cards).or_insert(cards);
            }

            acc
        })
        .values()
        .sum()
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
