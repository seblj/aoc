#[derive(Debug, Clone)]
struct Tree {
    visible: bool,
    value: i32,
}

fn parse(input: &[String]) -> Matrix<Tree> {
    Matrix {
        vec: input
            .into_iter()
            .enumerate()
            .flat_map(|(y, s)| {
                s.chars().enumerate().map(move |(x, c)| {
                    let visible = x == 0 || x == s.len() - 1 || y == 0 || y == input.len() - 1;
                    Tree {
                        visible,
                        value: c.to_digit(10).unwrap() as i32,
                    }
                })
            })
            .collect(),
        dim: (input.len() as i32, input[0].len() as i32),
    }
}

fn task_one(input: &[String]) -> usize {
    let mut matrix = parse(input);

    let width = matrix.width() - 1;
    let height = matrix.height() - 1;

    for x in 1..width {
        for y in 1..height {
            let val = matrix[[x, y]].value();

            if (x + 1..=width).all(|x| matrix[[x, y]].value() < val)
                || (y + 1..=height).all(|y| matrix[[x, y]].value() < val)
                || (0..x).rev().all(|x| matrix[[x, y]].value() < val)
                || (0..y).rev().all(|y| matrix[[x, y]].value() < val)
            {
                matrix[[x, y]].visible = true;
            }
        }
    }

    matrix.vec.into_iter().filter(|tree| tree.visible).count()
}

fn task_two(input: &[String]) -> usize {
    let matrix = parse(input);

    let mut highest = 0;

    let width = matrix.width() - 1;
    let height = matrix.height() - 1;

    for x in 1..width {
        for y in 1..height {
            let val = matrix[[x, y]].value();

            let right = (x + 1..=width)
                .position(|x| matrix[[x, y]].value() >= val)
                .and_then(|i| Some(i + 1))
                .unwrap_or((width - x) as usize);

            let down = (y + 1..=width)
                .position(|y| matrix[[x, y]].value() >= val)
                .and_then(|i| Some(i + 1))
                .unwrap_or((height - y) as usize);

            let left = (0..x)
                .rev()
                .position(|x| matrix[[x, y]].value() >= val)
                .and_then(|i| Some(i + 1))
                .unwrap_or(x as usize);

            let up = (0..y)
                .rev()
                .position(|y| matrix[[x, y]].value() >= val)
                .and_then(|i| Some(i + 1))
                .unwrap_or(y as usize);

            let sum = right * left * up * down;

            if sum > highest {
                highest = sum;
            }
        }
    }
    highest
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

struct Matrix<T> {
    vec: Vec<T>,
    dim: (i32, i32),
}

impl<T: std::fmt::Debug> std::fmt::Debug for Matrix<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in 0..self.vec.len() {
            if i as i32 % self.dim.0 == 0 {
                writeln!(f, "").unwrap();
            }
            write!(f, "{:?}, ", self.vec[i]).unwrap();
        }
        writeln!(f, "")
    }
}

impl<T: Value + Clone> Matrix<T> {
    #[allow(dead_code)]
    fn print(&self) {
        for i in 0..self.vec.len() {
            if i as i32 % self.dim.0 == 0 {
                println!("");
            }
            print!("{}", self.vec[i].value());
        }
        println!("");
    }

    fn width(&self) -> i32 {
        self.dim.0
    }

    fn height(&self) -> i32 {
        self.dim.1
    }
}

pub trait Value {
    type Item: std::fmt::Display;
    fn value(&self) -> Self::Item;
}

impl Value for Tree {
    type Item = i32;
    fn value(&self) -> Self::Item {
        self.value
    }
}

impl<T> std::ops::Index<[i32; 2]> for Matrix<T> {
    type Output = T;
    fn index(&self, idx: [i32; 2]) -> &T {
        let idx = ((idx[1] * self.dim.0) as usize) + idx[0] as usize;
        &self.vec[idx]
    }
}

impl<T> std::ops::IndexMut<[i32; 2]> for Matrix<T> {
    fn index_mut(&mut self, idx: [i32; 2]) -> &mut T {
        let idx = ((idx[1] * self.dim.0) as usize) + idx[0] as usize;
        &mut self.vec[idx]
    }
}
