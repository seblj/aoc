fn generator_rating<F>(vec: &[String], f: &F) -> usize
where
    F: Fn(usize, usize) -> bool,
{
    let mut ones: Vec<String> = Vec::new();
    let mut zeros: Vec<String> = Vec::new();
    let mut res = vec.to_owned();
    for b in 0..vec[0].len() {
        for v in &res {
            if v.chars().nth(b).unwrap() == '1' {
                ones.push(v.to_string());
            } else {
                zeros.push(v.to_string());
            }
        }
        if ones.is_empty() || zeros.is_empty() {
            break;
        };
        res = if f(ones.len(), zeros.len()) {
            ones.clone()
        } else {
            zeros.clone()
        };
        ones.clear();
        zeros.clear();
    }
    usize::from_str_radix(res[0].as_str(), 2).unwrap()
}

fn to_i32(slice: &[usize]) -> usize {
    slice
        .iter()
        .rev()
        .enumerate()
        .fold(0, |acc, (i, &b)| acc + 2_usize.pow(i as u32) * b)
}

fn task_one(input: &[String]) -> usize {
    let rows = input.len() / 2;
    let length = input[0].len();
    let mut bits: Vec<usize> = vec![0; length];
    let mut flipped: Vec<usize> = vec![0; length];

    for v in input {
        for (i, c) in v.chars().enumerate() {
            bits[i] += c.to_digit(10).unwrap() as usize;
        }
    }

    for i in 0..bits.len() {
        bits[i] = if bits[i] > rows { 1 } else { 0 };
        flipped[i] = if bits[i] == 1 { 0 } else { 1 };
    }

    let gamma = to_i32(&bits);
    let epsilon = to_i32(&flipped);

    gamma * epsilon
}

fn task_two(input: &[String]) -> usize {
    let gamma = generator_rating(input, &|a, b| a >= b);
    let epsilon = generator_rating(input, &|a, b| a < b);

    gamma * epsilon
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
