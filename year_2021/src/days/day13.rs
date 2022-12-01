use aoc::{read_input, time};
use std::path::Path;

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
            "x" => return Ok(Fold::Vertical(val.parse::<usize>().unwrap())),
            "y" => return Ok(Fold::Horizontal(val.parse::<usize>().unwrap())),
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
        return Ok(Coordinate { x, y });
    }
}

struct Map {
    coordinates: Vec<Coordinate>,
    folds: Vec<Fold>,
    max_x: usize,
    max_y: usize,
}

pub fn solve() {
    let input = "input";
    let file = Path::new(file!()).file_stem().unwrap().to_str().unwrap();
    let input = format!(
        "{}/src/days/input/{}.{}",
        env!("CARGO_MANIFEST_DIR"),
        file,
        input
    );

    let input: Vec<String> = read_input(input);
    let coordinates: Vec<Coordinate> = input
        .clone()
        .into_iter()
        .filter(|c| c.contains(','))
        .collect::<Vec<String>>()
        .iter()
        .map(|x| x.parse::<Coordinate>().unwrap())
        .collect();

    let folds: Vec<Fold> = input
        .clone()
        .into_iter()
        .filter(|c| c.contains('='))
        .collect::<Vec<String>>()
        .iter()
        .map(|x| x.replace("fold along ", "").parse::<Fold>().unwrap())
        .collect();

    let max_x = coordinates.iter().max_by_key(|p| p.x).unwrap().x as usize;
    let max_y = coordinates.iter().max_by_key(|p| p.y).unwrap().y as usize;

    let map = Map {
        coordinates,
        folds,
        max_x,
        max_y,
    };
    time("one", task_one, &map);
    time("two", task_two, &map);
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

fn task_one(map: &Map) -> i32 {
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
        break;
    }

    paper.iter().flatten().filter(|&&x| x == '#').count() as i32
}

fn task_two(map: &Map) -> i32 {
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
