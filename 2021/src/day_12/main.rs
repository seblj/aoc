use std::collections::HashMap;

trait IsLowercase {
    fn is_lowercase(&self) -> bool;
}

impl IsLowercase for String {
    fn is_lowercase(&self) -> bool {
        &self.to_lowercase() == self
    }
}

fn walk_1(
    map: &HashMap<String, Vec<String>>,
    city: String,
    visited: &mut HashMap<String, bool>,
    paths: &mut usize,
) {
    if city.to_lowercase() == "end" {
        *paths += 1;
        return;
    }
    // if the city is lowercase and we have visited it already
    if city.is_lowercase() && *visited.get(&city).unwrap() {
        return;
    }
    visited.insert(city.to_string(), true);

    let neighbors = map.get(&city).unwrap();
    for n in neighbors {
        walk_1(map, n.to_string(), visited, paths);
    }
    visited.insert(city.to_string(), false);
}

fn walk_2(
    map: &HashMap<String, Vec<String>>,
    city: String,
    visited: &mut HashMap<String, usize>,
    paths: &mut usize,
) {
    if city.to_lowercase() == "end" {
        *paths += 1;
        return;
    }
    // if the city is lowercase and another city has been visited twice
    // and we already have visited this city before
    if city.is_lowercase()
        && *visited.get(&city.to_string()).unwrap() >= 1
        && visited
            .iter()
            .filter(|x| x.0.is_lowercase() && *x.1 == 2)
            .count()
            > 0
    {
        return;
    }
    *visited.get_mut(&city.to_string()).unwrap() += 1;

    let neighbors = map.get(&city).unwrap();
    for n in neighbors {
        walk_2(map, n.to_string(), visited, paths);
    }
    *visited.get_mut(&city.to_string()).unwrap() -= 1;
}

fn parse(input: &[String]) -> HashMap<String, Vec<String>> {
    let mut m: HashMap<String, Vec<String>> = HashMap::new();
    for v in input {
        let (a, b) = v.split_once('-').unwrap();
        if b != "start" {
            m.entry(a.to_string()).or_default().push(b.to_string());
        }
        if a != "start" {
            m.entry(b.to_string()).or_default().push(a.to_string());
        }
    }
    m.remove("end");
    m
}

fn task_one(input: &[String]) -> usize {
    let map = parse(input);
    let mut visited: HashMap<String, bool> = HashMap::new();
    for k in map.keys() {
        visited.insert(k.to_string(), false);
    }

    let mut paths = 0;
    for city in map.get("start").unwrap() {
        walk_1(&map, city.to_string(), &mut visited, &mut paths);
        visited.iter_mut().for_each(|x| *x.1 = false);
    }
    paths
}

fn task_two(input: &[String]) -> usize {
    let map = parse(input);
    let mut visited: HashMap<String, usize> = HashMap::new();
    for k in map.keys() {
        visited.insert(k.to_string(), 0);
    }

    let mut paths = 0;
    for city in map.get("start").unwrap() {
        walk_2(&map, city.to_string(), &mut visited, &mut paths);
        visited.iter_mut().for_each(|x| *x.1 = 0);
    }
    paths
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
