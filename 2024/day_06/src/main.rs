use std::collections::HashSet;

use matrix::{Direction, Matrix};

mod matrix;

fn get_start_pos(matrix: &Matrix<char>) -> (usize, usize) {
    for w in 0..matrix.width() {
        for h in 0..matrix.height() {
            if matrix[(h, w)] == '^' {
                return (h, w);
            }
        }
    }
    unreachable!();
}

fn walk(
    matrix: &Matrix<char>,
    visited: &mut HashSet<(usize, usize)>,
    curr: (i32, i32),
    dir: Direction,
) {
    if !matrix.in_grid(curr) {
        return;
    }

    visited.insert((curr.0 as usize, curr.1 as usize));

    if matrix.get(dir.to_index(curr)) == Some(&'#') {
        let new_dir = match dir {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
            _ => unreachable!(),
        };

        walk(matrix, visited, new_dir.to_index(curr), new_dir);
    } else {
        walk(matrix, visited, dir.to_index(curr), dir);
    }
}

fn task_one(input: &[String]) -> usize {
    let matrix = Matrix::from(input);
    let starting_pos = get_start_pos(&matrix);
    let mut visited = HashSet::from([starting_pos]);

    walk(
        &matrix,
        &mut visited,
        (starting_pos.0 as i32, starting_pos.1 as i32),
        Direction::Up,
    );

    visited.len()
}

fn task_two(input: &[String]) -> usize {
    unimplemented!()
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
