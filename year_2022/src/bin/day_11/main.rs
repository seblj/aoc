use std::{collections::VecDeque, str::FromStr};

#[derive(Debug)]
enum Operation {
    Multiply { first: Value, second: Value },
    Add { first: Value, second: Value },
}

#[derive(Debug)]
enum Value {
    Old,
    Num(u64),
}

#[derive(Debug)]
struct Test {
    divisible: u64,
    monkey_true: usize,
    monkey_false: usize,
}

#[derive(Debug)]
struct Monkey {
    items: VecDeque<u64>,
    test: Test,
    operation: Operation,
    inspected: u64,
}

#[derive(Debug)]
struct Monkeys {
    monkeys: Vec<Monkey>,
}

impl FromStr for Value {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "old" => Ok(Self::Old),
            num => Ok(Self::Num(num.parse().unwrap())),
        }
    }
}

impl FromStr for Operation {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let op = s.trim().strip_prefix("Operation: new = ").unwrap();
        match op.splitn(3, ' ').collect::<Vec<&str>>()[..] {
            [val1, "+", val2] => Ok(Self::Add {
                first: Value::from_str(val1).unwrap(),
                second: Value::from_str(val2).unwrap(),
            }),
            [val1, "*", val2] => Ok(Self::Multiply {
                first: Value::from_str(val1).unwrap(),
                second: Value::from_str(val2).unwrap(),
            }),
            _ => unreachable!(),
        }
    }
}

fn get_and_parse<T>(s: &str, prefix: &str) -> T
where
    T: FromStr,
    <T as FromStr>::Err: std::fmt::Debug,
{
    s.trim().strip_prefix(prefix).unwrap().parse().unwrap()
}

impl From<&[String]> for Monkey {
    fn from(s: &[String]) -> Self {
        Self {
            items: s[1]
                .trim()
                .strip_prefix("Starting items: ")
                .unwrap()
                .split(',')
                .map(|s| s.trim().parse().unwrap())
                .collect(),
            test: Test {
                divisible: get_and_parse(&s[3], "Test: divisible by "),
                monkey_true: get_and_parse(&s[4], "If true: throw to monkey "),
                monkey_false: get_and_parse(&s[5], "If false: throw to monkey "),
            },
            operation: Operation::from_str(&s[2]).unwrap(),
            inspected: 0,
        }
    }
}

impl Value {
    fn get(&self, item: u64) -> u64 {
        match self {
            Value::Old => item,
            Value::Num(num) => *num,
        }
    }
}

impl Monkey {
    fn run_test(&self, item: u64) -> usize {
        if item % self.test.divisible == 0 {
            self.test.monkey_true
        } else {
            self.test.monkey_false
        }
    }

    fn inspect_item(&mut self, item: u64, task: &Task, divisor: u64) -> u64 {
        self.inspected += 1;
        let new_worry_level = match &self.operation {
            Operation::Multiply { first, second } => first.get(item) * second.get(item),
            Operation::Add { first, second } => first.get(item) + second.get(item),
        };
        if task == &Task::One {
            new_worry_level / 3
        } else {
            new_worry_level % divisor
        }
    }
}

impl Monkeys {
    fn new(input: &[String]) -> Self {
        Self {
            monkeys: input
                .split(|s| s.is_empty())
                .map(|s| s.into())
                .collect::<Vec<_>>(),
        }
    }

    fn get_divisor(&self) -> u64 {
        self.monkeys.iter().fold(0, |acc, monkey| {
            if acc == 0 {
                monkey.test.divisible
            } else {
                monkey.test.divisible * acc
            }
        })
    }

    fn play_rounds(&mut self, num_rounds: i32, task: &Task) {
        let divisor = self.get_divisor();
        for _ in 0..num_rounds {
            for monkey in 0..self.monkeys.len() {
                for i in 0..self.monkeys.get(monkey).unwrap().items.len() {
                    let m = self.monkeys.get_mut(monkey).unwrap();
                    let worry = m.inspect_item(*m.items.get(i).unwrap(), task, divisor);
                    let new = m.run_test(worry);
                    self.monkeys.get_mut(new).unwrap().items.push_back(worry);
                }
                self.monkeys.get_mut(monkey).unwrap().items.clear();
            }
        }
    }

    fn get_monkeybusiness(&self) -> u64 {
        let mut num_inspected = self
            .monkeys
            .iter()
            .map(|monkey| monkey.inspected)
            .collect::<Vec<_>>();

        num_inspected.sort_unstable_by(|a, b| b.cmp(a));
        num_inspected[0] * num_inspected[1]
    }
}

fn task_one(input: &[String]) -> u64 {
    let mut monkeys = Monkeys::new(input);
    monkeys.play_rounds(20, &Task::One);
    monkeys.get_monkeybusiness()
}

fn task_two(input: &[String]) -> u64 {
    let mut monkeys = Monkeys::new(input);
    monkeys.play_rounds(10000, &Task::Two);
    monkeys.get_monkeybusiness()
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

#[derive(PartialEq, Eq)]
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
