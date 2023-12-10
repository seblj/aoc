#[derive(Debug, Clone)]
struct Line {
    x1: usize,
    y1: usize,
    x2: usize,
    y2: usize,
}

#[derive(Debug, Clone)]
struct List {
    list: Vec<Line>,
    diagram: Vec<Vec<i32>>,
}

fn get_list(vec: &[String]) -> List {
    let mut list: Vec<Line> = Vec::new();
    let mut size: usize = 0;
    for v in vec {
        let (start, end) = v.split_once("->").unwrap();
        let (x1, y1) = start.trim().split_once(',').unwrap();
        let (x2, y2) = end.trim().split_once(',').unwrap();

        let x1 = x1.parse::<usize>().unwrap();
        let y1 = y1.parse::<usize>().unwrap();
        let x2 = x2.parse::<usize>().unwrap();
        let y2 = y2.parse::<usize>().unwrap();

        size = if x1 > size { x1 } else { size };
        size = if y1 > size { y1 } else { size };
        size = if x2 > size { x2 } else { size };
        size = if y2 > size { y2 } else { size };

        list.push(Line { x1, y1, x2, y2 })
    }
    List {
        list,
        diagram: vec![vec![0; size + 1]; size + 1],
    }
}

fn is_straight(line: &Line) -> bool {
    line.x1 == line.x2 || line.y1 == line.y2
}

fn is_diagonal(line: &Line) -> bool {
    (line.x1 + line.y2) == (line.x2 + line.y1) || (line.x1 + line.y1) == (line.x2 + line.y2)
}

fn update_straight(diagram: &mut [Vec<i32>], line: &Line) {
    if line.x1 == line.x2 {
        let (start, end) = if line.y1 < line.y2 {
            (line.y1, line.y2)
        } else {
            (line.y2, line.y1)
        };
        for y in start..=end {
            diagram[line.x1][y] += 1;
        }
    } else if line.y1 == line.y2 {
        let (start, end) = if line.x1 < line.x2 {
            (line.x1, line.x2)
        } else {
            (line.x2, line.x1)
        };
        (start..=end).for_each(|x| {
            diagram[x][line.y1] += 1;
        });
    }
}
fn update_diagnoal(diagram: &mut [Vec<i32>], line: &Line) {
    if line.x1 > line.x2 {
        let length = line.x1 - line.x2;
        if (line.x1 + line.y2) == (line.x2 + line.y1) {
            for i in 0..=length {
                let x = line.x2 + i;
                let y = line.y2 + i;
                diagram[x][y] += 1;
            }
        } else {
            for i in 0..=length {
                let x = line.x1 - i;
                let y = line.y1 + i;
                diagram[x][y] += 1;
            }
        }
    } else {
        let length = line.x2 - line.x1;
        if (line.x1 + line.y2) == (line.x2 + line.y1) {
            for i in 0..=length {
                let x = line.x1 + i;
                let y = line.y1 + i;
                diagram[x][y] += 1;
            }
        } else {
            for i in 0..=length {
                let x = line.x1 + i;
                let y = line.y1 - i;
                diagram[x][y] += 1;
            }
        }
    }
}

fn task_one(input: &[String]) -> usize {
    let mut list = get_list(input);
    for line in &list.list {
        if is_straight(line) {
            update_straight(&mut list.diagram, line);
        }
    }
    list.diagram.iter().flatten().filter(|&&x| x >= 2).count()
}

fn task_two(input: &[String]) -> usize {
    let mut list = get_list(input);
    for line in list.list {
        if is_straight(&line) {
            update_straight(&mut list.diagram, &line);
        } else if is_diagonal(&line) {
            update_diagnoal(&mut list.diagram, &line);
        }
    }
    list.diagram.iter().flatten().filter(|&&x| x >= 2).count()
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
