use std::collections::HashMap;
use std::path::PathBuf;
use std::str::FromStr;

#[derive(Debug)]
enum CdArg {
    Home,
    In(String),
    Out,
}

#[derive(Debug)]
enum DirEntry {
    File { size: u64 },
    Dir,
}

#[derive(Debug)]
enum Command {
    Cd(CdArg),
    Ls,
}

#[derive(Debug)]
enum Entry {
    Command(Command),
    DirEntry(DirEntry),
}

#[derive(Debug)]
struct FileTree {
    current_dir: PathBuf,
    dir: HashMap<PathBuf, u64>,
}

impl FileTree {
    fn new(input: &[String]) -> Self {
        let path = PathBuf::from("/");
        let mut filetree = FileTree {
            dir: HashMap::from([(path.clone(), 0)]),
            current_dir: path,
        };

        input.into_iter().for_each(|s| {
            let entry = Entry::from_str(s).unwrap();
            match entry {
                Entry::Command(cmd) => filetree.run_command(cmd),
                Entry::DirEntry(dir_entry) => filetree.add_entry(dir_entry),
            }
        });

        filetree
    }
    fn run_command(&mut self, command: Command) {
        match command {
            Command::Cd(action) => match action {
                CdArg::Home => {}
                CdArg::In(dir) => {
                    self.current_dir = self.current_dir.join(dir);
                    self.dir.insert(self.current_dir.clone(), 0);
                }
                CdArg::Out => {
                    self.current_dir.pop();
                }
            },
            Command::Ls => {}
        }
    }

    fn add_entry(&mut self, entry: DirEntry) {
        match entry {
            DirEntry::File { size, .. } => {
                let mut current_dir = self.current_dir.clone();
                self.dir
                    .entry(current_dir.clone())
                    .and_modify(|e| *e += size)
                    .or_insert(0);

                while current_dir.pop() {
                    self.dir
                        .entry(current_dir.clone())
                        .and_modify(|e| *e += size)
                        .or_insert(0);
                }
            }
            DirEntry::Dir { .. } => {}
        }
    }
}

impl FromStr for Entry {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(cmd) = s.strip_prefix("$ ") {
            if let Some((_, arg)) = cmd.split_once(' ') {
                let cd_arg = match arg {
                    "/" => CdArg::Home,
                    ".." => CdArg::Out,
                    name => CdArg::In(name.to_string()),
                };
                Ok(Self::Command(Command::Cd(cd_arg)))
            } else {
                Ok(Self::Command(Command::Ls))
            }
        } else {
            let (first, _) = s.split_once(' ').unwrap();
            if first == "dir" {
                Ok(Self::DirEntry(DirEntry::Dir))
            } else {
                Ok(Self::DirEntry(DirEntry::File {
                    size: first.parse::<u64>().unwrap(),
                }))
            }
        }
    }
}

fn task_one(input: &[String]) -> usize {
    let filetree = FileTree::new(input);
    filetree.dir.keys().into_iter().fold(0, |acc, dir| {
        let size = filetree.dir.get(dir).unwrap();
        if size < &100000 {
            acc + *size as usize
        } else {
            acc
        }
    })
}

fn task_two(input: &[String]) -> u64 {
    let filetree = FileTree::new(input);

    let total_available = 70000000;
    let needed = 30000000;

    let home_size = *filetree.dir.get(&PathBuf::from("/")).unwrap();
    let missing_space = needed - (total_available - home_size);

    let mut delete = total_available;
    filetree.dir.keys().for_each(|item| {
        let size = *filetree.dir.get(item).unwrap();
        if size >= missing_space {
            if size < delete {
                delete = size;
            }
        }
    });

    delete
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
