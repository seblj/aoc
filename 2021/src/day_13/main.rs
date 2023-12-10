#[derive(Clone, Debug)]
struct Coordinate {
    x: usize,
    y: usize,
}

#[derive(Debug)]
enum Fold {
    Vertical(usize),
    Horizontal(usize),
    None,
}

impl std::str::FromStr for Fold {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (dir, val) = s.split_once('=').unwrap();
        match dir {
            "x" => Ok(Fold::Vertical(val.parse::<usize>().unwrap())),
            "y" => Ok(Fold::Horizontal(val.parse::<usize>().unwrap())),
            _ => Ok(Fold::None),
        }
    }
}

impl std::str::FromStr for Coordinate {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, y) = s.split_once(',').unwrap();
        let x = x.parse::<usize>().unwrap();
        let y = y.parse::<usize>().unwrap();
        Ok(Coordinate { x, y })
    }
}

struct Map {
    coordinates: Vec<Coordinate>,
    folds: Vec<Fold>,
    max_x: usize,
    max_y: usize,
}

fn print_paper(paper: &Vec<Vec<char>>) {
    for x in paper {
        println!("{:?}", x);
    }
}

fn fold_vertical(paper: &mut Vec<Vec<char>>, position: usize) {
    for i in 0..paper.len() {
        for j in position + 1..paper[0].len() {
            let insert_j = (position - 1) - (j % (position + 1));
            if paper[i][insert_j] == '.' {
                paper[i][insert_j] = paper[i][j];
            }
        }
    }
    for p in paper {
        p.truncate(position);
    }
}

fn fold_horizontal(paper: &mut Vec<Vec<char>>, position: usize) {
    let pos = position + 1;
    for i in pos..paper.len() {
        let row = i - pos;
        let insert_row = (pos - 2) - row;
        for j in 0..paper[0].len() {
            if paper[insert_row][j] == '.' {
                paper[insert_row][j] = paper[i][j];
            }
        }
    }
    paper.truncate(position);
}

fn parse(input: &[String]) -> Map {
    let coordinates: Vec<Coordinate> = input
        .to_owned()
        .into_iter()
        .filter(|c| c.contains(','))
        .collect::<Vec<String>>()
        .iter()
        .map(|x| x.parse::<Coordinate>().unwrap())
        .collect();

    let folds: Vec<Fold> = input
        .to_owned()
        .into_iter()
        .filter(|c| c.contains('='))
        .collect::<Vec<String>>()
        .iter()
        .map(|x| x.replace("fold along ", "").parse::<Fold>().unwrap())
        .collect();

    let max_x = coordinates.iter().max_by_key(|p| p.x).unwrap().x as usize;
    let max_y = coordinates.iter().max_by_key(|p| p.y).unwrap().y as usize;

    Map {
        coordinates,
        folds,
        max_x,
        max_y,
    }
}

fn task_one(input: &[String]) -> usize {
    let map = parse(input);
    let mut paper = vec![vec!['.'; map.max_x + 1]; map.max_y + 1];

    for coordinate in &map.coordinates {
        let x = coordinate.x;
        let y = coordinate.y;
        paper[y][x] = '#';
    }

    match map.folds.iter().next().unwrap() {
        Fold::Horizontal(x) => fold_horizontal(&mut paper, *x),
        Fold::Vertical(x) => fold_vertical(&mut paper, *x),
        _ => unreachable!(),
    }

    paper.iter().flatten().filter(|&&x| x == '#').count()
}

fn task_two(input: &[String]) -> usize {
    let map = parse(input);
    let mut paper = vec![vec!['.'; map.max_x + 1]; map.max_y + 1];

    for coordinate in &map.coordinates {
        let x = coordinate.x;
        let y = coordinate.y;
        paper[y][x] = '#';
    }
    for fold in &map.folds {
        match fold {
            Fold::Horizontal(x) => fold_horizontal(&mut paper, *x),
            Fold::Vertical(x) => fold_vertical(&mut paper, *x),
            _ => unreachable!(),
        }
    }

    print_paper(&paper);
    0
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
