use matrix::Matrix;

mod matrix;

enum Direction {
    Up,
    Down,
    Left,
    Right,
    UpLeft,
    UpRight,
    DownLeft,
    DownRight,
}

impl Direction {
    fn to_index(&self, curr: (i32, i32)) -> (i32, i32) {
        match self {
            Direction::Up => (curr.0 - 1, curr.1),
            Direction::Down => (curr.0 + 1, curr.1),
            Direction::Left => (curr.0, curr.1 - 1),
            Direction::Right => (curr.0, curr.1 + 1),
            Direction::UpLeft => (curr.0 - 1, curr.1 - 1),
            Direction::UpRight => (curr.0 - 1, curr.1 + 1),
            Direction::DownLeft => (curr.0 + 1, curr.1 - 1),
            Direction::DownRight => (curr.0 + 1, curr.1 + 1),
        }
    }
}

fn find_next(
    matrix: &Matrix<char>,
    current_pos: (i32, i32),
    next: char,
    direction: Direction,
) -> bool {
    let new_pos = direction.to_index(current_pos);
    match matrix.get(new_pos) {
        Some(item) if *item == next => find_xmas(matrix, new_pos, next, direction),
        _ => false,
    }
}

fn find_xmas(matrix: &Matrix<char>, pos: (i32, i32), prev: char, direction: Direction) -> bool {
    match prev {
        'X' => find_next(matrix, pos, 'M', direction),
        'M' => find_next(matrix, pos, 'A', direction),
        'A' => find_next(matrix, pos, 'S', direction),
        'S' => true,
        _ => false,
    }
}

fn task_one(input: &[String]) -> usize {
    let matrix = Matrix::from(input);

    let mut sum = 0;

    for h in 0..matrix.height() {
        for w in 0..matrix.width() {
            let current = matrix[(w, h)];
            if current != 'X' {
                continue;
            }

            let index = (w as i32, h as i32);

            find_xmas(&matrix, index, 'X', Direction::Up).then(|| sum += 1);
            find_xmas(&matrix, index, 'X', Direction::Down).then(|| sum += 1);
            find_xmas(&matrix, index, 'X', Direction::Left).then(|| sum += 1);
            find_xmas(&matrix, index, 'X', Direction::Right).then(|| sum += 1);
            find_xmas(&matrix, index, 'X', Direction::UpLeft).then(|| sum += 1);
            find_xmas(&matrix, index, 'X', Direction::UpRight).then(|| sum += 1);
            find_xmas(&matrix, index, 'X', Direction::DownLeft).then(|| sum += 1);
            find_xmas(&matrix, index, 'X', Direction::DownRight).then(|| sum += 1);
        }
    }

    sum
}

fn task_two(input: &[String]) -> usize {
    let matrix = Matrix::from(input);

    let mut sum = 0;

    for h in 0..matrix.height() {
        for w in 0..matrix.width() {
            let current = matrix[(w, h)];
            if current != 'A' {
                continue;
            }

            let index = (w as i32, h as i32);

            if let Some(c) = matrix.get(Direction::UpRight.to_index(index)) {
                let opposite = match c {
                    'M' => 'S',
                    'S' => 'M',
                    _ => continue,
                };

                if Some(&opposite) == matrix.get(Direction::DownLeft.to_index(index)) {
                    let found = match matrix.get(Direction::UpLeft.to_index(index)) {
                        Some('M') => matrix.get(Direction::DownRight.to_index(index)) == Some(&'S'),
                        Some('S') => matrix.get(Direction::DownRight.to_index(index)) == Some(&'M'),
                        _ => false,
                    };

                    found.then(|| sum += 1);
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
