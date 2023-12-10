#[derive(Clone)]
struct Octopus {
    energy: u32,
    visited: bool,
}

fn update(vec: &mut Vec<Vec<Octopus>>, i: i32, j: i32) {
    if i < 0 || j < 0 || i >= vec.len() as i32 || j >= vec[0].len() as i32 {
        return;
    }
    let i = i as usize;
    let j = j as usize;
    vec[i][j].energy += 1;
    if vec[i][j].energy > 9 && !vec[i][j].visited {
        vec[i][j].visited = true;
        update_surroundings(vec, i, j);
    }
}

fn update_surroundings(vec: &mut Vec<Vec<Octopus>>, i: usize, j: usize) {
    let i = i as i32;
    let j = j as i32;
    update(vec, i - 1, j + 1); // upper right corner
    update(vec, i, j + 1); // right
    update(vec, i + 1, j + 1); // lower right corner
    update(vec, i + 1, j - 1); // lower left corner
    update(vec, i, j - 1); // left
    update(vec, i - 1, j - 1); // upper left corner
    update(vec, i - 1, j); // up
    update(vec, i + 1, j); // down
}

fn parse(input: &[String]) -> Vec<Vec<Octopus>> {
    input
        .iter()
        .map(|line| {
            line.chars()
                .map(|c| Octopus {
                    energy: c.to_digit(10).unwrap(),
                    visited: false,
                })
                .collect()
        })
        .collect()
}

fn task_one(input: &[String]) -> usize {
    let mut vec = parse(input);
    let mut flashes = 0;
    for _ in 0..100 {
        for i in 0..vec.len() {
            for j in 0..vec[0].len() {
                update(&mut vec, i as i32, j as i32);
            }
        }
        for i in 0..vec.len() {
            for j in 0..vec[0].len() {
                if vec[i][j].energy > 9 {
                    vec[i][j].energy = 0;
                    vec[i][j].visited = false;
                    flashes += 1;
                }
            }
        }
    }
    flashes as usize
}

fn task_two(input: &[String]) -> usize {
    let mut vec = parse(input);
    for step in 1.. {
        for i in 0..vec.len() {
            for j in 0..vec[0].len() {
                update(&mut vec, i as i32, j as i32);
            }
        }
        // If visited all
        if vec.iter().all(|x| x.iter().all(|c| c.visited)) {
            return step;
        }
        for i in 0..vec.len() {
            for j in 0..vec[0].len() {
                if vec[i][j].energy > 9 {
                    vec[i][j].energy = 0;
                    vec[i][j].visited = false;
                }
            }
        }
    }
    unreachable!();
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
