use std::collections::HashSet;
use std::str::FromStr;

pub struct Rucksack {
    first: String,
    second: String,
}

impl FromStr for Rucksack {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (c1, c2) = s.split_at(s.len() / 2);
        Ok(Rucksack {
            first: c1.to_string(),
            second: c2.to_string(),
        })
    }
}

pub trait RucksackItemScore {
    fn rucksack_item_score(self) -> usize;
}

impl RucksackItemScore for char {
    fn rucksack_item_score(self) -> usize {
        if self.is_uppercase() {
            self as usize - 38
        } else {
            self as usize - 96
        }
    }
}

fn task_one(input: &[String]) -> usize {
    input.into_iter().fold(0, |acc, rucksack| {
        let rucksack = Rucksack::from_str(rucksack).unwrap();
        for c1 in rucksack.first.chars() {
            for c2 in rucksack.second.chars() {
                if c2 == c1 {
                    return c1.rucksack_item_score() + acc;
                }
            }
        }
        acc
    })
}

fn task_two(input: &[String]) -> usize {
    input.chunks(3).into_iter().fold(0, |acc, group| {
        let first: HashSet<char> = group.get(0).unwrap().chars().collect();
        let second: HashSet<char> = group.get(1).unwrap().chars().collect();
        let third: HashSet<char> = group.get(2).unwrap().chars().collect();

        first
            .intersection(&second)
            .map(|c| c.to_owned())
            .collect::<HashSet<char>>()
            .intersection(&third)
            .collect::<Vec<&char>>()
            .pop()
            .unwrap()
            .rucksack_item_score()
            + acc
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
