use std::collections::HashSet;

fn is_low_point(vec: &Vec<Vec<u32>>, i: usize, j: usize) -> bool {
    let max_i = vec.len() - 1;
    let max_j = vec[0].len() - 1;
    let num = vec[i][j];

    if j < max_j && vec[i][j + 1] <= num {
        return false;
    }
    if j > 0 && vec[i][j - 1] <= num {
        return false;
    }
    if i < max_i && vec[i + 1][j] <= num {
        return false;
    }
    if i > 0 && vec[i - 1][j] <= num {
        return false;
    }

    true
}

#[derive(Eq, PartialEq)]
struct Point {
    i: i32,
    j: i32,
}

impl Point {
    fn new(i: i32, j: i32) -> Self {
        Self { i, j }
    }

    /// Returns a tuple of all directions from point (i, j)
    /// (down, right, up, left)
    fn get_sides(i: i32, j: i32) -> (Self, Self, Self, Self) {
        (
            Point { i: i - 1, j },
            Point { i, j: j + 1 },
            Point { i: i + 1, j },
            Point { i, j: j - 1 },
        )
    }
}

fn find_basin(
    vec: &Vec<Vec<u32>>,
    seen: &mut HashSet<(i32, i32)>,
    point: &Point,
    prev: u32,
    start: &Point,
) -> bool {
    let i = point.i;
    let j = point.j;

    // If out of bounds
    if i < 0 || j < 0 || i as usize >= vec.len() || j as usize >= vec[0].len() {
        return false;
    }

    let curr = vec[i as usize][j as usize];
    if curr == 9 || (prev >= curr && point != start) || !seen.insert((i, j)) {
        return false;
    }

    // Recurse further to find elements in the basin
    let (down, right, up, left) = Point::get_sides(i, j);
    find_basin(vec, seen, &down, curr, start)
        || find_basin(vec, seen, &right, curr, start)
        || find_basin(vec, seen, &up, curr, start)
        || find_basin(vec, seen, &left, curr, start)
}

fn parse(input: &[String]) -> Vec<Vec<u32>> {
    input
        .iter()
        .map(|v| v.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect()
}

fn task_one(input: &[String]) -> usize {
    let vec = parse(input);
    let mut result = 0;
    for (i, v) in vec.iter().enumerate() {
        for (j, _) in v.iter().enumerate() {
            if is_low_point(&vec, i, j) {
                result += vec[i][j] as usize + 1;
            }
        }
    }
    result
}

fn task_two(input: &[String]) -> usize {
    let vec = parse(input);
    let mut result: Vec<usize> = Vec::new();
    for (i, v) in vec.iter().enumerate() {
        for (j, _) in v.iter().enumerate() {
            if is_low_point(&vec, i, j) {
                let mut seen: HashSet<(i32, i32)> = HashSet::new();
                let point = Point::new(i as i32, j as i32);
                find_basin(&vec, &mut seen, &point, vec[i][j], &point);
                result.push(seen.len());
            }
        }
    }
    result.sort_unstable();
    result.iter().rev().take(3).product()
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
