use std::{cmp::Ordering, collections::BinaryHeap};

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
struct State {
    cost: i32,
    position: [i32; 2],
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug)]
struct Position {
    neighbours: Vec<[i32; 2]>,
    character: char,
    position: [i32; 2],
}

fn shortest_path(adj_list: &Matrix<Position>, start: [i32; 2], goal: [i32; 2]) -> Option<i32> {
    let mut dist: Matrix<i32> = Matrix {
        vec: vec![i32::MAX; adj_list.dim.0 as usize * adj_list.dim.1 as usize],
        dim: adj_list.dim,
    };
    dist[start] = 0;

    let mut heap = BinaryHeap::from([State {
        cost: 0,
        position: start,
    }]);

    while let Some(State { cost, position }) = heap.pop() {
        if position == goal {
            return Some(cost);
        }
        if cost > dist[position] {
            continue;
        }
        for neighbor in &adj_list[position].neighbours {
            let next = State {
                cost: cost + 1,
                position: *neighbor,
            };
            if next.cost < dist[next.position] {
                heap.push(next);
                dist[next.position] = next.cost;
            }
        }
    }
    None
}

fn get_neighbours(vec: &Vec<Vec<char>>, x: usize, y: usize) -> Vec<[i32; 2]> {
    let in_range = |v: &Vec<Vec<char>>, pos: &(i32, i32)| -> Option<char> {
        if pos.0 < 0 || pos.1 < 0 || pos.0 >= v[0].len() as i32 || pos.1 >= v.len() as i32 {
            return None;
        }
        Some(v[pos.1 as usize][pos.0 as usize])
    };

    let x = x as i32;
    let y = y as i32;
    [(x, y - 1), (x - 1, y), (x + 1, y), (x, y + 1)]
        .iter()
        .filter_map(|pos| {
            in_range(vec, &pos).and_then(|mut c| {
                let mut me = vec[y as usize][x as usize];
                if c == 'E' {
                    c = 'z'
                }
                if me == 'S' {
                    me = 'a';
                }

                if me as u32 + 1 >= c as u32 {
                    return Some([pos.0, pos.1]);
                } else {
                    return None;
                }
            })
        })
        .collect()
}

impl From<&[String]> for Matrix<Position> {
    fn from(s: &[String]) -> Self {
        let vec: Vec<Vec<char>> = s
            .into_iter()
            .map(|s| s.chars().map(|c| c).collect())
            .collect();

        let vec = vec
            .iter()
            .enumerate()
            .map(|(y, s)| {
                s.iter()
                    .enumerate()
                    .map(|(x, c)| Position {
                        neighbours: get_neighbours(&vec, x, y),
                        character: *c,
                        position: [x as i32, y as i32],
                    })
                    .collect::<Vec<Position>>()
            })
            .flatten()
            .collect::<Vec<Position>>();

        Self {
            vec,
            dim: (s[0].len() as i32, s.len() as i32),
        }
    }
}

fn get_pos(vec: &Vec<Position>, c: char) -> [i32; 2] {
    vec.iter().find(|s| s.character == c).unwrap().position
}

fn task_one(input: &[String]) -> i32 {
    let matrix: Matrix<Position> = input.into();

    let start_pos = get_pos(&matrix.vec, 'S');
    let end_pos = get_pos(&matrix.vec, 'E');

    shortest_path(&matrix, start_pos, end_pos).unwrap()
}

fn task_two(input: &[String]) -> i32 {
    let matrix: Matrix<Position> = input.into();

    let start = matrix
        .vec
        .iter()
        .filter_map(|p| match p.character {
            'a' | 'S' => Some(p.position),
            _ => None,
        })
        .collect::<Vec<[i32; 2]>>();

    let end_pos = get_pos(&matrix.vec, 'E');

    let mut paths = start
        .iter()
        .filter_map(|s| shortest_path(&matrix, *s, end_pos))
        .collect::<Vec<i32>>();

    paths.sort_unstable();

    paths[0]
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
