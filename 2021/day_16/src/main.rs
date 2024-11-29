fn to_binary(char: char) -> &'static str {
    match char {
        '0' => "0000",
        '1' => "0001",
        '2' => "0010",
        '3' => "0011",
        '4' => "0100",
        '5' => "0101",
        '6' => "0110",
        '7' => "0111",
        '8' => "1000",
        '9' => "1001",
        'A' => "1010",
        'B' => "1011",
        'C' => "1100",
        'D' => "1101",
        'E' => "1110",
        'F' => "1111",
        _ => unreachable!(),
    }
}

struct Header {
    version: i32,
    type_id: i32,
}

#[derive(Debug, PartialEq, Eq)]
enum PacketType {
    Literal,
    Operator(i32),
}

struct Packet {
    header: Header,
    packet_type: PacketType,
    length_id: Option<u8>,
    sub: Vec<Packet>,
    value: i64,
}

impl Header {
    fn get(binary: &str) -> Self {
        let version = i32::from_str_radix(&binary[0..3], 2).unwrap();
        let type_id = i32::from_str_radix(&binary[3..6], 2).unwrap();
        Self { version, type_id }
    }
}

impl Packet {
    fn new(binary: &str) -> Self {
        let header = Header::get(binary);
        let packet_type = match header.type_id {
            4 => PacketType::Literal,
            x => PacketType::Operator(x),
        };
        let length_id = match header.type_id {
            4 => None,
            _ => Some(binary.chars().nth(6).unwrap().to_digit(2).unwrap() as u8),
        };
        let sub = Vec::new();
        let value: i64 = 0;

        Self {
            header,
            packet_type,
            length_id,
            sub,
            value,
        }
    }
}

fn hex_to_binary(hex: &str) -> String {
    hex.chars().map(to_binary).collect()
}

fn last_group(group: &str) -> bool {
    group.starts_with('0')
}

// Gets length of subpackets in bits, and moves the cursor further on
fn subpacket_length(binary: &str, cursor: &mut usize) -> i32 {
    let cur = *cursor;
    *cursor += 22;
    i32::from_str_radix(&binary[cur + 7..cur + 22], 2).unwrap()
}

// Gets number of subpackets, and moves the cursor further on
fn num_subpackets(binary: &str, cursor: &mut usize) -> i32 {
    let cur = *cursor;
    *cursor += 18;
    i32::from_str_radix(&binary[cur + 7..cur + 18], 2).unwrap()
}

fn solution(original: &String, mut cursor: usize) -> (Packet, usize) {
    let binary = &original.clone()[cursor..];
    let mut packet = Packet::new(binary);
    match packet.packet_type {
        PacketType::Operator(_) => match packet.length_id {
            Some(x) => match x {
                0 => {
                    let num_bits = subpacket_length(original, &mut cursor);
                    let start = cursor;
                    loop {
                        let (p, new) = solution(original, cursor);
                        packet.sub.push(p);
                        cursor = new;
                        if cursor >= start + num_bits as usize {
                            break;
                        }
                    }
                    packet.value = match_typeid(&packet);
                    (packet, cursor)
                }
                1 => {
                    let num_subpackets = num_subpackets(original, &mut cursor);
                    for _ in 0..num_subpackets {
                        let (p, new) = solution(original, cursor);
                        packet.sub.push(p);
                        cursor = new;
                    }
                    packet.value = match_typeid(&packet);
                    (packet, cursor)
                }
                _ => panic!("Unexpected packet lenght_id"),
            },
            None => panic!("Should have length_id"),
        },
        PacketType::Literal => {
            let mut i = 6;
            let mut value = String::new();
            let mut group = &binary[i..i + 5];
            value.push_str(&group[1..]);
            while !last_group(group) {
                i += 5;
                value.push_str(&group[1..]);
                group = &binary[i..i + 5];
            }
            packet.value = i64::from_str_radix(&value, 2).unwrap();
            (packet, cursor + i + 5)
        }
    }
}

fn compare<F>(vec: &[i64], f: F) -> i64
where
    F: Fn(i64, i64) -> bool,
{
    let first = vec.first().unwrap();
    let second = vec.get(1).unwrap();
    if f(*first, *second) {
        1
    } else {
        0
    }
}

fn match_typeid(packet: &Packet) -> i64 {
    let sub: Vec<i64> = packet.sub.iter().map(|x| x.value).collect();
    match packet.packet_type {
        PacketType::Operator(type_id) => match type_id {
            0 => sub.iter().sum(),
            1 => sub.iter().product(),
            2 => sub.into_iter().min().unwrap(),
            3 => sub.into_iter().max().unwrap(),
            5 => compare(&sub, |a, b| a > b),
            6 => compare(&sub, |a, b| a < b),
            7 => compare(&sub, |a, b| a == b),
            _ => unreachable!(),
        },
        _ => unreachable!(),
    }
}

fn parse_version(packet: &Packet) -> i32 {
    let mut version = 0;
    packet.sub.iter().for_each(|x| {
        version += parse_version(x);
    });
    packet.header.version + version
}

fn task_one(input: &[String]) -> usize {
    let hex = input.first().unwrap();
    let binary = hex_to_binary(hex);

    let packet = solution(&binary, 0).0;
    parse_version(&packet) as usize
}

fn task_two(input: &[String]) -> usize {
    let hex = input.first().unwrap();
    let binary = hex_to_binary(hex);

    solution(&binary, 0).0.value as usize
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
