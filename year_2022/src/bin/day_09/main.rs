use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

#[derive(Debug)]
enum Direction {
    Up(i32),
    Down(i32),
    Left(i32),
    Right(i32),
}

impl FromStr for Direction {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.split_once(' ').unwrap() {
            ("U", num) => Ok(Direction::Up(num.parse().unwrap())),
            ("D", num) => Ok(Direction::Down(num.parse().unwrap())),
            ("L", num) => Ok(Direction::Left(num.parse().unwrap())),
            ("R", num) => Ok(Direction::Right(num.parse().unwrap())),
            _ => unreachable!(),
        }
    }
}

struct Rope {
    visited_tail: HashSet<(i32, i32)>,
    knot_map: HashMap<i32, (i32, i32)>,
    knots: i32,
}

impl Rope {
    fn new(knots: i32) -> Self {
        let mut set = HashSet::new();
        let mut map = HashMap::new();
        for i in 0..knots {
            map.insert(i, (0, 0));
        }
        set.insert((0, 0));
        Self {
            visited_tail: set,
            knot_map: map,
            knots,
        }
    }

    fn touching(&self, curr: i32, next: i32) -> bool {
        let curr = self.knot_map.get(&curr).unwrap();
        let next = self.knot_map.get(&next).unwrap();
        curr == next
            || next.0 == curr.0 + 1 && next.1 == curr.1
            || next.0 == curr.0 - 1 && next.1 == curr.1
            || next.0 == curr.0 && next.1 == curr.1 + 1
            || next.0 == curr.0 && next.1 == curr.1 - 1
            || next.0 == curr.0 - 1 && next.1 == curr.1 + 1
            || next.0 == curr.0 + 1 && next.1 == curr.1 + 1
            || next.0 == curr.0 + 1 && next.1 == curr.1 - 1
            || next.0 == curr.0 - 1 && next.1 == curr.1 - 1
    }

    // Returns an optional direction tail should walk in if head is straight ahead
    fn straight(&self, curr: i32, next: i32) -> Option<Direction> {
        let curr = self.knot_map.get(&curr).unwrap();
        let next = self.knot_map.get(&next).unwrap();

        if next.0 == curr.0 - 2 && next.1 == curr.1 {
            return Some(Direction::Right(0));
        } else if next.0 == curr.0 + 2 && next.1 == curr.1 {
            return Some(Direction::Left(0));
        } else if next.0 == curr.0 && next.1 == curr.1 - 2 {
            return Some(Direction::Down(0));
        } else if next.0 == curr.0 && next.1 == curr.1 + 2 {
            return Some(Direction::Up(0));
        }
        None
    }

    // Returns an optional direction tail should walk in if head is diagonal
    fn diagonally(&self, curr: i32, next: i32) -> Option<(Direction, Direction)> {
        let curr = self.knot_map.get(&curr).unwrap();
        let next = self.knot_map.get(&next).unwrap();
        if next.0 == curr.0 - 1 && next.1 == curr.1 + 2
            || next.0 == curr.0 - 2 && next.1 == curr.1 + 1
            || next.0 == curr.0 - 2 && next.1 == curr.1 + 2
        {
            return Some((Direction::Right(0), Direction::Up(0)));
        } else if next.0 == curr.0 + 1 && next.1 == curr.1 + 2
            || next.0 == curr.0 + 2 && next.1 == curr.1 + 1
            || next.0 == curr.0 + 2 && next.1 == curr.1 + 2
        {
            return Some((Direction::Left(0), Direction::Up(0)));
        } else if next.0 == curr.0 - 1 && next.1 == curr.1 - 2
            || next.0 == curr.0 - 2 && next.1 == curr.1 - 1
            || next.0 == curr.0 - 2 && next.1 == curr.1 - 2
        {
            return Some((Direction::Right(0), Direction::Down(0)));
        } else if next.0 == curr.0 + 2 && next.1 == curr.1 - 1
            || next.0 == curr.0 + 1 && next.1 == curr.1 - 2
            || next.0 == curr.0 + 2 && next.1 == curr.1 - 2
        {
            return Some((Direction::Left(0), Direction::Down(0)));
        } else {
            None
        }
    }

    fn move_knot(&mut self, knot: i32, direction: &Direction) {
        let knot = self.knot_map.get_mut(&knot).unwrap();
        match direction {
            Direction::Up(_) => knot.1 -= 1,
            Direction::Down(_) => knot.1 += 1,
            Direction::Left(_) => knot.0 -= 1,
            Direction::Right(_) => knot.0 += 1,
        };
    }

    fn knot(&self, knot: i32) -> &(i32, i32) {
        self.knot_map.get(&knot).unwrap()
    }

    fn r#move(&mut self, num: i32, direction: &Direction) {
        for _ in 0..num {
            self.move_knot(0, direction);
            for knot in 0..self.knots - 1 {
                let next = knot + 1;
                if !self.touching(knot, next) {
                    if let Some(straight) = self.straight(knot, next) {
                        self.move_knot(next, &straight);
                        if next == self.knots - 1 {
                            self.visited_tail.insert(*self.knot(next));
                        }
                    } else if let Some(diagonally) = self.diagonally(knot, next) {
                        self.move_knot(next, &diagonally.0);
                        self.move_knot(next, &diagonally.1);
                        if next == self.knots - 1 {
                            self.visited_tail.insert(*self.knot(next));
                        }
                    }
                }
            }
        }
    }
}

impl Direction {
    fn walk(&self, rope: &mut Rope) {
        match self {
            Direction::Up(num) => rope.r#move(*num, self),
            Direction::Down(num) => rope.r#move(*num, self),
            Direction::Left(num) => rope.r#move(*num, self),
            Direction::Right(num) => rope.r#move(*num, self),
        };
    }
}

fn task_one(input: &[String]) -> usize {
    let mut rope = Rope::new(2);
    input.into_iter().for_each(|s| {
        let dir = Direction::from_str(s).unwrap();
        dir.walk(&mut rope);
    });

    rope.visited_tail.into_iter().count()
}

fn task_two(input: &[String]) -> usize {
    let mut rope = Rope::new(10);
    input.into_iter().for_each(|s| {
        let dir = Direction::from_str(s).unwrap();
        dir.walk(&mut rope);
    });

    rope.visited_tail.into_iter().count()
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
