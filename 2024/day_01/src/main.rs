use std::collections::HashMap;

fn task_one(input: &[String]) -> usize {
    let (mut left, mut right): (Vec<usize>, Vec<usize>) = input
        .iter()
        .map(|it| {
            let numbers = it
                .split_whitespace()
                .map(|num| num.parse::<usize>().unwrap())
                .collect::<Vec<usize>>();
            (numbers[0], numbers[1])
        })
        .collect();

    left.sort();
    right.sort();

    left.iter()
        .zip(right.iter())
        .fold(0, |acc, it| acc + it.0.abs_diff(*it.1))
}

fn task_two(input: &[String]) -> usize {
    let mut right: HashMap<usize, usize> = HashMap::new();

    let left: Vec<usize> = input
        .iter()
        .map(|it| {
            let numbers = it
                .split_whitespace()
                .map(|num| num.parse::<usize>().unwrap())
                .collect::<Vec<usize>>();

            right.entry(numbers[1]).and_modify(|e| *e += 1).or_insert(1);

            numbers[0]
        })
        .collect();

    left.iter()
        .fold(0, |acc, it| acc + it * right.get(it).unwrap_or(&0))
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
