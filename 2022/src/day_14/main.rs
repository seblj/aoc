use std::collections::HashSet;

use itertools::Itertools;

fn parse(input: &[String]) -> HashSet<(i32, i32)> {
    let mut set: HashSet<(i32, i32)> = HashSet::new();
    input
        .into_iter()
        .map(|s| {
            s.split(" -> ")
                .map(|x| {
                    let (a, b) = x.trim().split(',').collect_tuple().unwrap();
                    (a.parse().unwrap(), b.parse().unwrap())
                })
                .collect()
        })
        .into_iter()
        .for_each(|paths: Vec<(i32, i32)>| {
            paths.windows(2).for_each(|path| {
                let x_min = path[0].0.min(path[1].0);
                let y_min = path[0].1.min(path[1].1);
                let x_max = path[0].0.max(path[1].0);
                let y_max = path[0].1.max(path[1].1);

                for x in x_min..=x_max {
                    for y in y_min..=y_max {
                        set.insert((x, y));
                    }
                }
            })
        });
    set
}

fn let_it_rain(
    map: &mut HashSet<(i32, i32)>,
    pos: (i32, i32),
    max_y: i32,
    task: Task,
) -> Option<(i32, i32)> {
    match task {
        Task::One => {
            if pos.1 == max_y {
                return None;
            }
        }
        Task::Two => {
            if map.contains(&(500, 0)) {
                return None;
            }

            if pos.1 + 1 == max_y {
                map.insert(pos);
                return Some(pos);
            }
        }
    }

    match [(0, 1), (-1, 1), (1, 1)]
        .into_iter()
        .find(|(dx, dy)| !map.contains(&(pos.0 + dx, pos.1 + dy)))
    {
        Some((dx, dy)) => let_it_rain(map, (pos.0 + dx, pos.1 + dy), max_y, task),
        _ => {
            map.insert(pos);
            return Some(pos);
        }
    }
}

fn task_one(input: &[String]) -> usize {
    let mut set = parse(input);
    let max_y = set.iter().max_by_key(|a| a.1).unwrap().1;

    let mut num = 0;
    while let_it_rain(&mut set, (500, 0), max_y, Task::One).is_some() {
        num += 1;
    }
    num
}

fn task_two(input: &[String]) -> usize {
    let mut set = parse(input);
    let max_y = set.iter().max_by_key(|a| a.1).unwrap().1 + 2;

    let mut num = 0;
    while let_it_rain(&mut set, (500, 0), max_y, Task::Two).is_some() {
        num += 1;
    }
    num
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

#[derive(PartialEq, Eq)]
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
