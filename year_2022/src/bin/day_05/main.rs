use std::{collections::HashMap, str::FromStr};

use itertools::Itertools;

#[derive(Debug)]
struct Command {
    num: u32,
    from: u32,
    to: u32,
}

impl FromStr for Command {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (num, from, to) = s
            .split_whitespace()
            .filter_map(|s| s.parse().ok())
            .next_tuple::<(_, _, _)>()
            .unwrap();

        Ok(Command { num, from, to })
    }
}

#[derive(Debug)]
struct Crates {
    map: HashMap<u32, Vec<char>>,
    commands: Vec<Command>,
}

impl Crates {
    fn get_answer(&mut self) -> String {
        let map = self.map.clone();
        map.keys()
            .into_iter()
            .sorted()
            .map(|key| self.map.get_mut(key).unwrap().pop().unwrap())
            .collect::<String>()
    }
}

pub trait Mover {
    fn crate_mover_9000(&mut self);
    fn crate_mover_9001(&mut self);
}

impl Mover for Crates {
    fn crate_mover_9000(&mut self) {
        self.commands.iter().for_each(|c| {
            (0..c.num).into_iter().for_each(|_| {
                let val = self.map.get_mut(&c.from).unwrap().pop().unwrap();
                let vec = self.map.get_mut(&c.to).unwrap();
                vec.push(val);
            })
        })
    }

    fn crate_mover_9001(&mut self) {
        self.commands.iter().for_each(|c| {
            let vec = (0..c.num)
                .into_iter()
                .filter_map(|_| self.map.get_mut(&c.from).unwrap().pop())
                .collect::<Vec<_>>()
                .into_iter()
                .rev()
                .collect::<Vec<_>>();
            if let Some(v) = self.map.get_mut(&c.to) {
                v.extend(vec);
            }
        })
    }
}

impl From<&[String]> for Crates {
    fn from(s: &[String]) -> Self {
        let (map, commands) = s
            .splitn(2, |s| s.len() == 0)
            .next_tuple::<(_, _)>()
            .unwrap();

        let mut crates_map: HashMap<u32, Vec<char>> = HashMap::new();
        map.into_iter().rev().for_each(|s| {
            s.chars().enumerate().for_each(|(i, c)| {
                if c.is_uppercase() {
                    let idx = (i / 4) as u32 + 1;
                    match crates_map.get_mut(&idx) {
                        Some(vec) => vec.push(c),
                        None => {
                            crates_map.insert(idx, vec![c]);
                        }
                    }
                }
            })
        });

        let commands = commands
            .into_iter()
            .map(|command| Command::from_str(command).unwrap())
            .collect();

        Self {
            commands,
            map: crates_map,
        }
    }
}

fn task_one(input: &[String]) -> String {
    let mut crates: Crates = input.into();
    crates.crate_mover_9000();
    crates.get_answer()
}

fn task_two(input: &[String]) -> String {
    let mut crates: Crates = input.into();
    crates.crate_mover_9001();
    crates.get_answer()
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
