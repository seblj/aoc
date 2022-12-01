#[derive(PartialOrd, Ord, Eq, PartialEq)]
struct Elf {
    calories: i32,
}

impl FromIterator<i32> for Elf {
    fn from_iter<T: IntoIterator<Item = i32>>(iter: T) -> Self {
        Self {
            calories: iter.into_iter().sum(),
        }
    }
}

fn parse_input(input: &[String]) -> Vec<Elf> {
    input
        .split(|str| str.is_empty())
        .map(|calories| {
            calories
                .into_iter()
                .map(|c| c.parse::<i32>().unwrap())
                .collect::<Elf>()
        })
        .collect::<Vec<Elf>>()
}

fn task_one(input: &[String]) -> i32 {
    let elfs = parse_input(input);
    elfs.iter().map(|elf| elf.calories).max().unwrap()
}

fn task_two(input: &[String]) -> i32 {
    let mut elfs = parse_input(input);
    elfs.sort_unstable_by(|a, b| b.calories.cmp(&a.calories));
    elfs.into_iter().take(3).map(|elf| elf.calories).sum()
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
