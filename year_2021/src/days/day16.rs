use aoc::{read_input, time};
use std::path::Path;

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
    fn get(binary: &String) -> Self {
        let version = i32::from_str_radix(&binary[0..3], 2).unwrap();
        let type_id = i32::from_str_radix(&binary[3..6], 2).unwrap();
        return Self { version, type_id };
    }
}

impl Packet {
    fn new(binary: &String) -> Self {
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

fn hex_to_binary(hex: &String) -> String {
    hex.chars().map(to_binary).collect()
}

pub fn solve() {
    let input = "test";
    let file = Path::new(file!()).file_stem().unwrap().to_str().unwrap();
    let input = format!(
        "{}/src/days/input/{}.{}",
        env!("CARGO_MANIFEST_DIR"),
        file,
        input
    );

    let vec: Vec<String> = read_input(input);
    let hex = vec.first().unwrap();
    let binary = hex_to_binary(&hex);

    time("one", task_one, &binary);
    time("two", task_two, &binary);
}

fn last_group(group: &str) -> bool {
    group.chars().nth(0).unwrap() == '0'
}

// Gets length of subpackets in bits, and moves the cursor further on
fn subpacket_length(binary: &String, cursor: &mut usize) -> i32 {
    let cur = *cursor;
    *cursor += 22;
    i32::from_str_radix(&binary[cur + 7..cur + 22], 2).unwrap()
}

// Gets number of subpackets, and moves the cursor further on
fn num_subpackets(binary: &String, cursor: &mut usize) -> i32 {
    let cur = *cursor;
    *cursor += 18;
    i32::from_str_radix(&binary[cur + 7..cur + 18], 2).unwrap()
}

fn solution(original: &String, mut cursor: usize) -> (Packet, usize) {
    let binary = &original.clone()[cursor..];
    let mut packet = Packet::new(&binary.to_string());
    match packet.packet_type {
        PacketType::Operator(_) => match packet.length_id {
            Some(x) => match x {
                0 => {
                    let num_bits = subpacket_length(&original, &mut cursor);
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
                    return (packet, cursor);
                }
                1 => {
                    let num_subpackets = num_subpackets(&original, &mut cursor);
                    for _ in 0..num_subpackets {
                        let (p, new) = solution(original, cursor);
                        packet.sub.push(p);
                        cursor = new;
                    }
                    packet.value = match_typeid(&packet);
                    return (packet, cursor);
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
            while !last_group(&group) {
                i += 5;
                value.push_str(&group[1..]);
                group = &binary[i..i + 5];
            }
            packet.value = i64::from_str_radix(&value, 2).unwrap();
            return (packet, cursor + i + 5);
        }
    }
}

fn compare<F>(vec: &Vec<i64>, f: F) -> i64
where
    F: Fn(i64, i64) -> bool,
{
    let first = vec.iter().nth(0).unwrap();
    let second = vec.iter().nth(1).unwrap();
    return if f(*first, *second) { 1 } else { 0 };
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
    return packet.header.version + version;
}

fn task_one(binary: &String) -> i32 {
    let packet = solution(binary, 0).0;
    parse_version(&packet)
}

fn task_two(binary: &String) -> i64 {
    solution(binary, 0).0.value
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_one() {
        let hex = String::from("8A004A801A8002F478");
        let binary = hex_to_binary(&hex);
        assert_eq!(task_one(&binary), 16);
    }
    #[test]
    fn test_part_one_two() {
        let hex = String::from("620080001611562C8802118E34");
        let binary = hex_to_binary(&hex);
        assert_eq!(task_one(&binary), 12);
    }
    #[test]
    fn test_part_one_three() {
        let hex = String::from("C0015000016115A2E0802F182340");
        let binary = hex_to_binary(&hex);
        assert_eq!(task_one(&binary), 23);
    }
    #[test]
    fn test_part_one_four() {
        let hex = String::from("A0016C880162017C3686B18A3D4780");
        let binary = hex_to_binary(&hex);
        assert_eq!(task_one(&binary), 31);
    }

    #[test]
    fn test_part_two_one() {
        let hex = String::from("C200B40A82");
        let binary = hex_to_binary(&hex);
        assert_eq!(task_two(&binary), 3);
    }

    #[test]
    fn test_part_two_two() {
        let hex = String::from("04005AC33890");
        let binary = hex_to_binary(&hex);
        assert_eq!(task_two(&binary), 54);
    }

    #[test]
    fn test_part_two_three() {
        let hex = String::from("880086C3E88112");
        let binary = hex_to_binary(&hex);
        assert_eq!(task_two(&binary), 7);
    }

    #[test]
    fn test_part_two_four() {
        let hex = String::from("CE00C43D881120");
        let binary = hex_to_binary(&hex);
        assert_eq!(task_two(&binary), 9);
    }

    #[test]
    fn test_part_two_five() {
        let hex = String::from("D8005AC2A8F0");
        let binary = hex_to_binary(&hex);
        assert_eq!(task_two(&binary), 1);
    }

    #[test]
    fn test_part_two_six() {
        let hex = String::from("F600BC2D8F");
        let binary = hex_to_binary(&hex);
        assert_eq!(task_two(&binary), 0);
    }

    #[test]
    fn test_part_two_seven() {
        let hex = String::from("9C005AC2F8F0");
        let binary = hex_to_binary(&hex);
        assert_eq!(task_two(&binary), 0);
    }

    #[test]
    fn test_part_two_eight() {
        let hex = String::from("9C0141080250320F1802104A08");
        let binary = hex_to_binary(&hex);
        assert_eq!(task_two(&binary), 1);
    }
}
