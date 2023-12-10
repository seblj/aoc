use std::{cmp::Ordering, collections::BinaryHeap};

#[derive(Debug, Copy, Clone)]
struct Edge {
    node: usize,
    cost: usize,
}

fn get_edge(map: &Vec<Vec<i32>>, i: i32, j: i32) -> Option<Edge> {
    if i < 0 || j < 0 || i as usize >= map[0].len() || j as usize >= map.len() {
        return None;
    }
    let i = i as usize;
    let j = j as usize;
    Some(Edge {
        node: i * map[0].len() + j,
        cost: map[i][j] as usize,
    })
}

fn get_input(vec: &Vec<Vec<i32>>) -> Vec<Vec<Edge>> {
    let mut map: Vec<Vec<Edge>> = Vec::new();
    vec.iter().flatten().enumerate().for_each(|(idx, _)| {
        let mut v: Vec<Edge> = Vec::new();

        let i = (idx / vec[0].len()) as i32;
        let j = (idx % vec.len()) as i32;

        let up = get_edge(vec, i - 1, j);
        let right = get_edge(vec, i, j + 1);
        let down = get_edge(vec, i + 1, j);
        let left = get_edge(vec, i, j - 1);

        [up, right, down, left].iter().for_each(|dir| match dir {
            Some(x) => v.push(*x),
            None => {}
        });

        map.push(v);
    });
    map
}

fn get_larger_input(map: &Vec<Vec<i32>>) -> Vec<Vec<Edge>> {
    let mut matrix = vec![vec![0_i32; map[0].len() * 5]; map.len() * 5];
    for i in 0..matrix.len() {
        for j in 0..matrix[0].len() {
            let x = j / map[0].len() + i / map.len();
            let mut cost = (map[i % map[0].len()][j % map.len()] as usize + x) % 9;
            if cost == 0 {
                cost = 9;
            }
            matrix[i][j] = cost as i32;
        }
    }
    get_input(&matrix)
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
struct State {
    cost: usize,
    position: usize,
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

fn shortest_path(adj_list: &Vec<Vec<Edge>>, start: usize, goal: usize) -> Option<usize> {
    let mut dist: Vec<_> = (0..adj_list.len()).map(|_| usize::MAX).collect();
    let mut heap = BinaryHeap::new();

    dist[start] = 0;
    heap.push(State {
        cost: 0,
        position: start,
    });
    while let Some(State { cost, position }) = heap.pop() {
        if position == goal {
            return Some(cost);
        }
        if cost > dist[position] {
            continue;
        }
        for edge in &adj_list[position] {
            let next = State {
                cost: cost + edge.cost,
                position: edge.node,
            };
            if next.cost < dist[next.position] {
                heap.push(next);
                dist[next.position] = next.cost;
            }
        }
    }
    None
}

fn task_one(input: &[String]) -> usize {
    let vec: Vec<Vec<i32>> = input
        .iter()
        .map(|s| s.chars().map(|c| c.to_digit(10).unwrap() as i32).collect())
        .collect();

    let map = get_input(&vec);

    let last = map.len() - 1;
    match shortest_path(&map, 0, last) {
        Some(x) => x,
        None => unreachable!(),
    }
}

fn task_two(input: &[String]) -> usize {
    let vec: Vec<Vec<i32>> = input
        .iter()
        .map(|s| s.chars().map(|c| c.to_digit(10).unwrap() as i32).collect())
        .collect();

    let map = get_larger_input(&vec);

    let last = map.len() - 1;
    match shortest_path(&map, 0, last) {
        Some(x) => x,
        None => unreachable!(),
    }
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
