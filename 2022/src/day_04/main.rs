use std::{ops::RangeInclusive, str::FromStr};

struct Pair {
    first: RangeInclusive<i16>,
    second: RangeInclusive<i16>,
}

impl FromStr for Pair {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (range1, range2) = s.split_once(',').unwrap();
        let get_range = |s: &str| {
            let (start, end) = s.split_once('-').unwrap();
            RangeInclusive::new(start.parse().unwrap(), end.parse().unwrap())
        };
        Ok(Self {
            first: get_range(range1),
            second: get_range(range2),
        })
    }
}

trait Overlap {
    fn overlap_all(&mut self) -> bool;
    fn overlap_any(&mut self) -> bool;
}

impl Overlap for Pair {
    fn overlap_all(&mut self) -> bool {
        if self.first.len() < self.second.len() {
            self.first.all(|x| self.second.contains(&x))
        } else {
            self.second.all(|x| self.first.contains(&x))
        }
    }
    fn overlap_any(&mut self) -> bool {
        if self.first.len() < self.second.len() {
            self.first.any(|x| self.second.contains(&x))
        } else {
            self.second.any(|x| self.first.contains(&x))
        }
    }
}

fn task_one(input: &[String]) -> usize {
    input
        .into_iter()
        .filter(|pair| Pair::from_str(pair).unwrap().overlap_all())
        .count()
}

fn task_two(input: &[String]) -> usize {
    input
        .into_iter()
        .filter(|pair| Pair::from_str(pair).unwrap().overlap_any())
        .count()
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
