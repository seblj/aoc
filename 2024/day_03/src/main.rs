use regex::Regex;

const MUL: &str = r"mul\((?<left>[0-9]{1,3}),(?<right>[0-9]{1,3})\)";
const DO: &str = r"(?<do>)do\(\)";
const DONT: &str = r"(?<dont>)don't\(\)";

fn task_one(input: &[String]) -> usize {
    let re = Regex::new(MUL).unwrap();
    let hay = input.iter().flat_map(|s| s.chars()).collect::<String>();

    re.captures_iter(&hay)
        .map(|c| c.extract())
        .map(|(_, [left, right])| left.parse::<usize>().unwrap() + right.parse::<usize>().unwrap())
        .sum()
}

fn task_two(input: &[String]) -> usize {
    let hay = input.iter().flat_map(|s| s.chars()).collect::<String>();

    let re = Regex::new(&format!("{MUL}|{DO}|{DONT}")).unwrap();
    let mut enabled = true;
    let mut sum = 0;

    for it in re.captures_iter(&hay) {
        match it.get(0).unwrap().as_str() {
            "don't()" => enabled = false,
            "do()" => enabled = true,
            s => {
                let (left, right) = s.split_once(',').unwrap();
                let left = left.replace("mul(", "").parse::<usize>().unwrap();
                let right = right.replace(")", "").parse::<usize>().unwrap();

                if enabled {
                    sum += left * right;
                }
            }
        }
    }

    sum
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
