#![feature(slice_partition_dedup)]
use std::{cmp::Ordering, str::FromStr};

#[derive(Eq, PartialEq, PartialOrd, Ord, Copy, Clone, Debug)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Eq, PartialEq, PartialOrd, Ord, Copy, Clone, Debug)]
enum CardPartOne {
    J,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    T,
    Q,
    K,
    A,
}

#[derive(Debug, Eq, PartialEq)]
struct Hand {
    cards: [CardPartOne; 5],
    bid: usize,
    hand_type: HandType,
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.hand_type.cmp(&other.hand_type) {
            Ordering::Equal => self.cards.iter().cmp(other.cards.iter()),
            Ordering::Less => Ordering::Less,
            Ordering::Greater => Ordering::Greater,
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl FromStr for Hand {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (card_str, bid) = s.split_once(' ').unwrap();
        let mut cards = [CardPartOne::A; 5];
        for (idx, c) in card_str.chars().enumerate() {
            cards[idx] = match c {
                '2' => CardPartOne::Two,
                '3' => CardPartOne::Three,
                '4' => CardPartOne::Four,
                '5' => CardPartOne::Five,
                '6' => CardPartOne::Six,
                '7' => CardPartOne::Seven,
                '8' => CardPartOne::Eight,
                '9' => CardPartOne::Nine,
                'T' => CardPartOne::T,
                'J' => CardPartOne::J,
                'Q' => CardPartOne::Q,
                'K' => CardPartOne::K,
                'A' => CardPartOne::A,
                _ => unreachable!(),
            };
        }

        let v = cards;
        let mut sorted_cards: Vec<_> = v.iter().filter(|c| **c != CardPartOne::J).collect();
        let filtered_cards_len = sorted_cards.len();
        sorted_cards.sort();

        let (dedup, duplicates) = sorted_cards.partition_dedup();
        let hand_type = match (filtered_cards_len, dedup.len(), duplicates.len()) {
            (0, _, _) | (1, _, _) => HandType::FiveOfAKind,
            (2, 4, 1) => HandType::OnePair,

            // Default cases. No jokers
            (5, 1, 4) => HandType::FiveOfAKind,
            (5, 4, 1) => HandType::OnePair,
            (5, 5, 0) => HandType::HighCard,
            (5, 2, 3) => {
                if duplicates[0] == duplicates[1] && duplicates[1] == duplicates[2] {
                    HandType::FourOfAKind
                } else {
                    HandType::FullHouse
                }
            }
            (5, 3, 2) => {
                if duplicates[0] == duplicates[1] {
                    HandType::ThreeOfAKind
                } else {
                    HandType::TwoPair
                }
            }
            // (2, 3) => {
            //     if duplicates[0] == duplicates[1] && duplicates[1] == duplicates[2] {
            //         HandType::FourOfAKind
            //     } else {
            //         HandType::FullHouse
            //     }
            // }
            // (3, 2) => {
            //     if duplicates[0] == duplicates[1] {
            //         HandType::ThreeOfAKind
            //     } else {
            //         HandType::TwoPair
            //     }
            // }
            // (4, 1) => HandType::OnePair,
            // (5, 0) => HandType::HighCard,
            _ => unreachable!(),
        };

        Ok(Self {
            cards,
            hand_type,
            bid: bid.parse::<usize>().unwrap(),
        })
    }
}

fn task_one(input: &[String]) -> usize {
    unimplemented!()
}

fn task_two(input: &[String]) -> usize {
    let mut hands: Vec<_> = input
        .iter()
        .filter_map(|s| Hand::from_str(s).ok())
        .collect();

    hands.sort();

    hands
        .iter()
        .enumerate()
        .map(|(i, hand)| (i + 1) * hand.bid)
        .sum()
    // unimplemented!()
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
