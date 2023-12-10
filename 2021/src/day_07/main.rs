fn median(vec: &mut [i32]) -> i32 {
    vec.sort();
    let mid = vec.len() / 2;
    vec[mid]
}

fn mean(vec: &[i32]) -> f32 {
    vec.iter().sum::<i32>() as f32 / vec.len() as f32
}

fn divergent(num: usize) -> usize {
    (num * (num + 1)) / 2
}

fn parse(input: &[String]) -> Vec<i32> {
    input
        .first()
        .unwrap()
        .split(',')
        .map(|x| x.parse::<i32>().unwrap())
        .collect()
}

fn task_one(input: &[String]) -> usize {
    let mut vec = parse(input);
    let med = median(&mut vec);
    let mut sum = 0;
    vec.iter()
        .for_each(|&x| sum += (med - x).unsigned_abs() as usize);
    sum
}

fn task_two(input: &[String]) -> usize {
    let vec = parse(input);

    let mean_ceil = mean(&vec).ceil() as i32;
    let mean_floor = mean(&vec).floor() as i32;

    let mut sum_ceil = 0;
    let mut sum_floor = 0;

    vec.iter().for_each(|&x| {
        sum_floor += divergent((mean_floor - x).unsigned_abs() as usize);
        sum_ceil += divergent((mean_ceil - x).unsigned_abs() as usize);
    });

    std::cmp::min(sum_ceil, sum_floor)
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
