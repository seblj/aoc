use matrix::{Direction, Matrix};

mod matrix;

fn find_next(matrix: &Matrix<u8>, current_pos: (i32, i32), next: u8, direction: Direction) -> bool {
    let new_pos = direction.to_index(current_pos);
    match matrix.get(new_pos) {
        Some(item) if *item == next => find_xmas(matrix, new_pos, next, direction),
        _ => false,
    }
}

fn find_xmas(matrix: &Matrix<u8>, pos: (i32, i32), prev: u8, direction: Direction) -> bool {
    match prev {
        b'X' => find_next(matrix, pos, b'M', direction),
        b'M' => find_next(matrix, pos, b'A', direction),
        b'A' => find_next(matrix, pos, b'S', direction),
        b'S' => true,
        _ => false,
    }
}

fn task_one(input: &[String]) -> usize {
    let matrix = Matrix::from(input);

    let mut sum = 0;

    for (pos, _) in matrix.iter() {
        let current = matrix[pos];
        if current != b'X' {
            continue;
        }

        let index = (pos.0 as i32, pos.1 as i32);

        find_xmas(&matrix, index, b'X', Direction::Up).then(|| sum += 1);
        find_xmas(&matrix, index, b'X', Direction::Down).then(|| sum += 1);
        find_xmas(&matrix, index, b'X', Direction::Left).then(|| sum += 1);
        find_xmas(&matrix, index, b'X', Direction::Right).then(|| sum += 1);
        find_xmas(&matrix, index, b'X', Direction::UpLeft).then(|| sum += 1);
        find_xmas(&matrix, index, b'X', Direction::UpRight).then(|| sum += 1);
        find_xmas(&matrix, index, b'X', Direction::DownLeft).then(|| sum += 1);
        find_xmas(&matrix, index, b'X', Direction::DownRight).then(|| sum += 1);
    }

    sum
}

fn task_two(input: &[String]) -> usize {
    let matrix = Matrix::from(input);

    let mut sum = 0;

    for (pos, _) in matrix.iter() {
        let current = matrix[pos];
        if current != b'A' {
            continue;
        }

        let index = (pos.0 as i32, pos.1 as i32);

        if let Some(c) = matrix.get(Direction::UpRight.to_index(index)) {
            let opposite = match c {
                b'M' => b'S',
                b'S' => b'M',
                _ => continue,
            };

            if Some(&opposite) == matrix.get(Direction::DownLeft.to_index(index)) {
                let down_right = Direction::DownRight.to_index(index);
                let found = match matrix.get(Direction::UpLeft.to_index(index)) {
                    Some(b'M') => matrix.get(down_right) == Some(&b'S'),
                    Some(b'S') => matrix.get(down_right) == Some(&b'M'),
                    _ => false,
                };

                found.then(|| sum += 1);
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
