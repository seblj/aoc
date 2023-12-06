use std::collections::HashSet;
use year_2023::matrix::Matrix;

fn is_adjacent(matrix: &Matrix<char>, w: usize, h: usize) -> bool {
    matrix
        .iter_adjacent(w, h)
        .any(|pos| !matrix[pos].is_ascii_digit() && matrix[pos] != '.')
}

fn task_one(input: &[String]) -> usize {
    let matrix = Matrix::new(
        input.iter().flat_map(|s| s.chars()).collect(),
        (input[0].len(), input.len()),
    );

    let mut sum = 0;

    for h in 0..matrix.height() {
        let mut vec: Vec<usize> = Vec::with_capacity(matrix.width());
        let mut is_adj = false;

        for w in 0..matrix.width() {
            let current = matrix[(w, h)];

            if let Some(digit) = current.to_digit(10) {
                if is_adjacent(&matrix, w, h) {
                    is_adj = true;
                }
                vec.push(digit as usize);
            } else {
                if is_adj {
                    sum += vec.iter().fold(0, |acc, x| acc * 10 + x);
                }
                vec.clear();
                is_adj = false;
            }
        }

        if is_adj {
            sum += vec.iter().fold(0, |acc, x| acc * 10 + x);
        }
    }

    sum
}

fn find_number_at_pos(matrix: &Matrix<char>, idx: (usize, usize)) -> ((usize, usize), usize) {
    let start_idx = (0..idx.0)
        .rev()
        .find_map(|i| {
            let non_number = !matrix[(i, idx.1)].is_ascii_digit();
            non_number.then_some(i + 1)
        })
        .unwrap_or(0);

    let num = (start_idx..matrix.width())
        .map_while(|i| matrix[(i, idx.1)].to_digit(10).map(|n| n as usize))
        .fold(0, |acc, it| acc * 10 + it);

    ((start_idx, idx.1), num)
}

fn find_adjecent_numbers(matrix: &Matrix<char>, w: usize, h: usize) -> Vec<usize> {
    matrix
        .iter_adjacent(w, h)
        .filter(|&x| matrix[x].is_ascii_digit())
        .map(|x| find_number_at_pos(matrix, x))
        .collect::<HashSet<_>>()
        .iter()
        .map(|x| x.1)
        .collect()
}

fn task_two(input: &[String]) -> usize {
    let matrix = Matrix::new(
        input.iter().flat_map(|s| s.chars()).collect(),
        (input[0].len(), input.len()),
    );

    let mut sum = 0;

    for h in 0..matrix.height() {
        for w in 0..matrix.width() {
            if matrix[(w, h)] != '*' {
                continue;
            }

            let numbers = find_adjecent_numbers(&matrix, w, h);
            if numbers.len() == 2 {
                sum += numbers[0] * numbers[1];
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
