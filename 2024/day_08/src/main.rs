use std::collections::{HashMap, HashSet};

use matrix::Matrix;

mod matrix;

fn get_antinodes(a: (i32, i32), b: (i32, i32)) -> [(i32, i32); 2] {
    let (x, y) = (a.0 - b.0, a.1 - b.1);
    [(a.0 + x, a.1 + y), (b.0 - x, b.1 - y)]
}

fn task_one(input: &[String]) -> usize {
    let matrix = Matrix::from(input);
    let mut map: HashMap<&u8, Vec<(i32, i32)>> = HashMap::new();
    for (pos, it) in matrix.iter() {
        match it {
            b'0'..=b'9' | b'a'..=b'z' | b'A'..=b'Z' => {
                let pos = (pos.0 as i32, pos.1 as i32);
                map.entry(it)
                    .and_modify(|e| e.push(pos))
                    .or_insert(vec![pos]);
            }
            _ => continue,
        }
    }

    let mut set = HashSet::new();

    for values in map.values() {
        for pos1 in values.iter() {
            for pos2 in values.iter() {
                if pos1 == pos2 {
                    continue;
                }

                let antinodes = get_antinodes(*pos1, *pos2);
                if matrix.in_grid(antinodes[0]) {
                    set.insert(antinodes[0]);
                }

                if matrix.in_grid(antinodes[1]) {
                    set.insert(antinodes[1]);
                }
            }
        }
    }

    set.len()
}

fn get_antinodes_2_1(
    matrix: &Matrix<u8>,
    set: &mut HashSet<(i32, i32)>,
    a: (i32, i32),
    b: (i32, i32),
) {
    if !matrix.in_grid(a) {
        return;
    }

    set.insert(a);

    let (x, y) = (a.0 - b.0, a.1 - b.1);
    let new_a = (a.0 + x, a.1 + y);

    get_antinodes_2_1(matrix, set, new_a, a);
}

fn get_antinodes_2_2(
    matrix: &Matrix<u8>,
    set: &mut HashSet<(i32, i32)>,
    a: (i32, i32),
    b: (i32, i32),
) {
    if !matrix.in_grid(b) {
        return;
    }

    set.insert(b);

    let (x, y) = (a.0 - b.0, a.1 - b.1);
    let new_b = (b.0 - x, b.1 - y);

    get_antinodes_2_2(matrix, set, b, new_b);
}

// Basically just need to keep adding antinodes on the map with the
// same distance as between two frequencies, while it is still in the grid
fn task_two(input: &[String]) -> usize {
    let matrix = Matrix::from(input);
    let mut map: HashMap<&u8, Vec<(i32, i32)>> = HashMap::new();
    for (pos, it) in matrix.iter() {
        match it {
            b'0'..=b'9' | b'a'..=b'z' | b'A'..=b'Z' => {
                let pos = (pos.0 as i32, pos.1 as i32);
                map.entry(it)
                    .and_modify(|e| e.push(pos))
                    .or_insert(vec![pos]);
            }
            _ => continue,
        }
    }

    let mut set = HashSet::new();

    for values in map.values() {
        for pos1 in values.iter() {
            for pos2 in values.iter() {
                if pos1 == pos2 {
                    continue;
                }

                get_antinodes_2_1(&matrix, &mut set, *pos1, *pos2);
                get_antinodes_2_2(&matrix, &mut set, *pos1, *pos2);
            }
        }
    }

    set.len()
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
