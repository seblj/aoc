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
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    T,
    J,
    Q,
    K,
    A,
}

#[derive(Eq, PartialEq, PartialOrd, Ord, Copy, Clone, Debug)]
enum CardPartTwo {
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

trait FromChar {
    fn from_char(c: char) -> Self;
}

macro_rules! impl_from_char {
    ($ty:tt) => {
        impl FromChar for $ty {
            fn from_char(c: char) -> Self {
                match c {
                    '2' => Self::Two,
                    '3' => Self::Three,
                    '4' => Self::Four,
                    '5' => Self::Five,
                    '6' => Self::Six,
                    '7' => Self::Seven,
                    '8' => Self::Eight,
                    '9' => Self::Nine,
                    'T' => Self::T,
                    'J' => Self::J,
                    'Q' => Self::Q,
                    'K' => Self::K,
                    'A' => Self::A,
                    _ => unreachable!(),
                }
            }
        }
    };
}

impl_from_char!(CardPartOne);
impl_from_char!(CardPartTwo);

#[derive(Debug, Eq, PartialEq)]
struct Hand<T> {
    cards: [T; 5],
    bid: usize,
    hand_type: Option<HandType>,
}

impl<T> Ord for Hand<T>
where
    T: std::cmp::Eq + std::cmp::Ord,
{
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.hand_type
            .cmp(&other.hand_type)
            .then_with(|| self.cards.iter().cmp(other.cards.iter()))
    }
}

impl<T> PartialOrd for Hand<T>
where
    T: std::cmp::PartialEq + std::cmp::Ord,
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(std::cmp::Ord::cmp(self, other))
    }
}

fn fill_cards<T>(cards: &mut [T; 5], s: &str)
where
    T: FromChar,
{
    for (idx, c) in s.chars().enumerate() {
        cards[idx] = T::from_char(c);
    }
}

impl<T> Hand<T>
where
    T: std::cmp::Eq,
{
    fn new(cards: [T; 5], bid: usize) -> Self {
        Hand {
            cards,
            bid,
            hand_type: None,
        }
    }

    fn set_hand(&mut self, dedup: &[T], duplicates: &[T]) {
        match (dedup.len(), duplicates.len()) {
            (5, 0) => self.hand_type = Some(HandType::HighCard),
            (1, 4) => self.hand_type = Some(HandType::FiveOfAKind),
            (4, 1) => self.hand_type = Some(HandType::OnePair),
            (2, 3) => {
                if duplicates[0] == duplicates[1] && duplicates[1] == duplicates[2] {
                    self.hand_type = Some(HandType::FourOfAKind)
                } else {
                    self.hand_type = Some(HandType::FullHouse)
                }
            }
            (3, 2) => {
                if duplicates[0] == duplicates[1] {
                    self.hand_type = Some(HandType::ThreeOfAKind)
                } else {
                    self.hand_type = Some(HandType::TwoPair)
                }
            }
            _ => unreachable!(),
        };
    }
}

impl Hand<CardPartTwo> {
    fn set_custom_hand(&mut self) {
        let mut sorted_cards: Vec<_> = self
            .cards
            .into_iter()
            .filter(|c| *c != CardPartTwo::J)
            .collect();

        sorted_cards.sort();
        let number_of_jokers = 5 - sorted_cards.len();

        let (dedup, duplicates) = unique_and_dupes(&sorted_cards);
        match (number_of_jokers, dedup.len(), duplicates.len()) {
            (5, _, _) => self.hand_type = Some(HandType::FiveOfAKind),
            (4, _, _) => self.hand_type = Some(HandType::FiveOfAKind),

            (3, 2, 0) => self.hand_type = Some(HandType::FourOfAKind),
            (3, 1, 1) => self.hand_type = Some(HandType::FiveOfAKind),

            (2, 3, 0) => self.hand_type = Some(HandType::ThreeOfAKind),
            (2, 2, 1) => self.hand_type = Some(HandType::FourOfAKind),
            (2, 1, 2) => self.hand_type = Some(HandType::FiveOfAKind),

            (1, 4, 0) => self.hand_type = Some(HandType::OnePair),
            (1, 3, 1) => self.hand_type = Some(HandType::ThreeOfAKind),
            (1, 1, 3) => self.hand_type = Some(HandType::FiveOfAKind),
            (1, 2, 2) => {
                if duplicates[0] == duplicates[1] {
                    self.hand_type = Some(HandType::FourOfAKind)
                } else {
                    self.hand_type = Some(HandType::FullHouse)
                }
            }

            // Default cases. No jokers
            _ => self.set_hand(&dedup, &duplicates),
        };
    }
}

fn unique_and_dupes<T: Eq + Clone>(cards: &[T]) -> (Vec<T>, Vec<T>) {
    let mut unique = Vec::new();
    let mut seen = Vec::new();
    for card in cards {
        if !unique.contains(card) {
            unique.push(card.clone());
        } else {
            seen.push(card.clone());
        }
    }
    (unique, seen)
}

impl Hand<CardPartOne> {
    fn set_custom_hand(&mut self) {
        let mut sorted_cards = self.cards;
        sorted_cards.sort();

        let (dedup, duplicates) = unique_and_dupes(&sorted_cards);
        self.set_hand(&dedup, &duplicates);
    }
}

fn task_one(input: &[String]) -> usize {
    let mut hands: Vec<_> = input
        .iter()
        .map(|s| {
            let (s, bid) = s.split_once(' ').unwrap();
            let mut cards = [CardPartOne::Two; 5];
            fill_cards(&mut cards, s);
            let mut hand = Hand::new(cards, bid.parse::<usize>().unwrap());
            hand.set_custom_hand();
            hand
        })
        .collect();

    hands.sort();

    hands
        .iter()
        .enumerate()
        .map(|(i, hand)| (i + 1) * hand.bid)
        .sum()
}

fn task_two(input: &[String]) -> usize {
    let mut hands: Vec<_> = input
        .iter()
        .map(|s| {
            let (s, bid) = s.split_once(' ').unwrap();
            let mut cards = [CardPartTwo::Two; 5];
            fill_cards(&mut cards, s);
            let mut hand = Hand::new(cards, bid.parse::<usize>().unwrap());
            hand.set_custom_hand();
            hand
        })
        .collect();

    hands.sort();

    hands
        .iter()
        .enumerate()
        .map(|(i, hand)| (i + 1) * hand.bid)
        .sum()
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
