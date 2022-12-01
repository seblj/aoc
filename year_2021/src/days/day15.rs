use aoc::{read_input, time};
use std::cmp::Ordering;
use std::collections::*;
use std::path::Path;

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
    return Some(Edge {
        node: i * map[0].len() + j,
        cost: map[i][j] as usize,
    });
}

fn get_input(vec: &Vec<Vec<i32>>) -> Vec<Vec<Edge>> {
    let mut map: Vec<Vec<Edge>> = Vec::new();
    vec.iter().flatten().enumerate().for_each(|(idx, _)| {
        let mut v: Vec<Edge> = Vec::new();

        let i = (idx / vec[0].len()) as i32;
        let j = (idx % vec.len()) as i32;

        let up = get_edge(&vec, i - 1, j);
        let right = get_edge(&vec, i, j + 1);
        let down = get_edge(&vec, i + 1, j);
        let left = get_edge(&vec, i, j - 1);

        vec![up, right, down, left]
            .iter()
            .for_each(|dir| match dir {
                Some(x) => v.push(*x),
                None => {}
            });

        map.push(v);
    });
    return map;
}

fn get_larger_input(map: &Vec<Vec<i32>>) -> Vec<Vec<Edge>> {
    let mut matrix = vec![vec![0 as i32; map[0].len() * 5]; map.len() * 5];
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
    return get_input(&matrix);
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
    let vec: Vec<String> = read_input(input);
    let vec: Vec<Vec<i32>> = vec
        .iter()
        .map(|s| s.chars().map(|c| c.to_digit(10).unwrap() as i32).collect())
        .collect();

    let map = get_input(&vec);
    let larger_map = get_larger_input(&vec);
    time("one", task_one, &map);
    time("two", task_two, &larger_map);
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

fn task_one(map: &Vec<Vec<Edge>>) -> i32 {
    let last = map.len() - 1;
    match shortest_path(map, 0, last) {
        Some(x) => return x as i32,
        None => return -1,
    };
}

fn task_two(map: &Vec<Vec<Edge>>) -> i32 {
    let last = map.len() - 1;
    match shortest_path(map, 0, last) {
        Some(x) => return x as i32,
        None => return -1,
    };
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = "test";
        let vec: Vec<String> = read_input(input);
        let vec: Vec<Vec<i32>> = vec
            .iter()
            .map(|s| s.chars().map(|c| c.to_digit(10).unwrap() as i32).collect())
            .collect();
        let map = get_input(&vec);
        assert_eq!(shortest_path(&map, 0, 99), Some(40));
    }

    #[test]
    fn test_part_two() {
        let input = "test";
        let vec: Vec<String> = read_input(input);
        let vec: Vec<Vec<i32>> = vec
            .iter()
            .map(|s| s.chars().map(|c| c.to_digit(10).unwrap() as i32).collect())
            .collect();
        let map = get_larger_input(&vec);
        let last = map.len() - 1;
        println!("last: {}", last);
        assert_eq!(shortest_path(&map, 0, last), Some(315));
    }
}
