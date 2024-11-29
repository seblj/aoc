use std::{collections::HashMap, str::FromStr};

#[derive(Debug)]
enum Instruction {
    Addx(i32),
    Noop,
}

#[derive(Debug)]
struct Cpu {
    instructions: Vec<Instruction>,
    register_map: HashMap<i32, i32>,
    register: i32,
    cycles: i32,
}

impl Cpu {
    fn new(input: &[String]) -> Self {
        Self {
            instructions: input
                .into_iter()
                .map(|s| Instruction::from_str(s).unwrap())
                .collect(),
            cycles: 0,
            register_map: HashMap::from([(0, 1)]),
            register: 1,
        }
    }

    fn run(&mut self, cycles: i32, mut crt: Option<&mut Crt>) {
        self.instructions.iter().for_each(|instruction| {
            if self.cycles == cycles {
                return;
            }
            match instruction {
                Instruction::Addx(num) => {
                    self.cycles += 1;
                    if let Some(ref mut crt) = crt {
                        crt.draw(self.cycles, self.register);
                    }
                    self.register_map.insert(self.cycles, self.register);
                    if self.cycles == cycles {
                        return;
                    }
                    self.cycles += 1;
                    self.register_map.insert(self.cycles, self.register);
                    self.register += num;
                    if let Some(ref mut crt) = crt {
                        crt.draw(self.cycles, self.register);
                    }
                }
                Instruction::Noop => {
                    self.cycles += 1;
                    if let Some(ref mut crt) = crt {
                        crt.draw(self.cycles, self.register);
                    }
                    self.register_map.insert(self.cycles, self.register);
                }
            }
        })
    }
}

impl FromStr for Instruction {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "noop" => Ok(Self::Noop),
            x => match x.split_once(' ') {
                Some((_, num)) => Ok(Self::Addx(num.parse().unwrap())),
                None => unreachable!(),
            },
        }
    }
}

fn task_one(input: &[String]) -> i32 {
    let mut cpu = Cpu::new(input);
    cpu.run(220, None);
    [20, 60, 100, 140, 180, 220]
        .into_iter()
        .fold(0, |acc, num| {
            acc + (*cpu.register_map.get(&num).unwrap() * num)
        })
}

fn task_two(input: &[String]) -> String {
    let mut vec = vec!['.'; 240];
    vec[0] = '#';
    let mut crt = Crt { vec, dim: (40, 6) };
    let mut cpu = Cpu::new(input);
    cpu.run(239, Some(&mut crt));
    crt.print();
    "PZBGZEJB".to_string()
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

struct Crt {
    vec: Vec<char>,
    dim: (i32, i32),
}

impl std::fmt::Debug for Crt {
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

impl Crt {
    #[allow(dead_code)]
    fn print(&self) {
        for i in 0..self.vec.len() {
            if i as i32 % self.dim.0 == 0 {
                println!("");
            }
            print!("{}", self.vec[i]);
        }
        println!("");
    }

    fn draw(&mut self, cycles: i32, register: i32) {
        let row = cycles / 40;
        let col = cycles.rem_euclid(40);
        if register - 1 == col || register == col || register + 1 == col {
            self[[col, row]] = '#';
        } else {
            self[[col, row]] = '.';
        }
    }
}

impl std::ops::Index<[i32; 2]> for Crt {
    type Output = char;
    fn index(&self, idx: [i32; 2]) -> &char {
        let idx = ((idx[1] * self.dim.0) as usize) + idx[0] as usize;
        &self.vec[idx]
    }
}

impl std::ops::IndexMut<[i32; 2]> for Crt {
    fn index_mut(&mut self, idx: [i32; 2]) -> &mut char {
        let idx = ((idx[1] * self.dim.0) as usize) + idx[0] as usize;
        &mut self.vec[idx]
    }
}
