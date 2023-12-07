use std::cmp::Ordering;
use std::collections::HashMap;
use advent_of_code::Day;


#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
enum Card {
    JOKER = 1,
    TWO = 2,
    THREE = 3,
    FOUR = 4,
    FIVE = 5,
    SIX = 6,
    SEVEN = 7,
    EIGHT = 8,
    NINE = 9,
    TEN = 10,
    JACK = 11,
    QUEEN = 12,
    KING = 13,
    ACE = 14,
}

impl Card {
    fn from_str(s: char) -> Card {
        match s {
            '2' => Card::TWO,
            '3' => Card::THREE,
            '4' => Card::FOUR,
            '5' => Card::FIVE,
            '6' => Card::SIX,
            '7' => Card::SEVEN,
            '8' => Card::EIGHT,
            '9' => Card::NINE,
            'T' => Card::TEN,
            'J' => Card::JACK,
            'Q' => Card::QUEEN,
            'K' => Card::KING,
            'A' => Card::ACE,
            _ => panic!("Invalid card")
        }
    }

    fn jack_to_joker(&self) -> &Card {
        match self {
            Card::JACK => &Card::JOKER,
            _ => self
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd)]
enum HandType {
    FiveOfAKind = 6,
    FourOfAKind = 5,
    FullHouse = 4,
    ThreeOfAKind = 3,
    TwoPair = 2,
    OnePair = 1,
    HighCard = 0,
}

#[derive(Debug)]
struct Hand {
    cards: [Card; 5],
    bid: i32,
}

impl Hand {
    fn new(hand: &str, bid: &str) -> Self {
        Self {
            cards: hand.chars().map(Card::from_str).collect::<Vec<Card>>().try_into().expect("Invalid hand"),
            bid: bid.parse::<i32>().expect("Invalid bid"),
        }
    }

    fn get_type_without_joker(types: HashMap<Card, usize>) -> HandType {
        match types.values().collect::<Vec<&usize>>() {
            values if values.len() == 5 => HandType::HighCard,
            values if values.len() == 4 => HandType::OnePair,
            values if values.len() == 1 => HandType::FiveOfAKind,
            values if values.len() == 2 => match (values[0], values[1]) {
                (4, 1) | (1, 4) => HandType::FourOfAKind,
                (3, 2) | (2, 3) => HandType::FullHouse,
                _ => panic!("Invalid type!")
            },
            values if values.len() == 3 => match (values[0], values[1], values[2]) {
                (3, 1, 1) | (1, 1, 3) | (1, 3, 1) => HandType::ThreeOfAKind,
                (2, 2, 1) | (1, 2, 2) | (2, 1, 2) => HandType::TwoPair,
                _ => panic!("Invalid type!")
            },
            _ => panic!("Invalid type!")
        }
    }
    fn get_type_with_joker(mut types: HashMap<Card, usize>) -> HandType {
        let jokers = types.remove(&Card::JOKER).unwrap_or(0);

        if types.is_empty() {
            types.insert(Card::JOKER, jokers);
        } else {
            let card = types.iter().max_by_key(|&(_, i)| i).map(|(&card, _)| card).unwrap();
            types.entry(card).and_modify(|i| *i += jokers);
        }

        Hand::get_type_without_joker(types)
    }

    fn get_type(&self) -> HandType {
        let mut types: HashMap<Card, usize> = HashMap::new();
        self.cards.iter().for_each(|&card| *types.entry(card).or_insert(0) += 1);

        match types.contains_key(&Card::JOKER) {
            true => Hand::get_type_with_joker(types),
            false => Hand::get_type_without_joker(types)
        }
    }

    fn compare_cards(&self, other: &Hand) -> Ordering {
        for (&s, &o) in self.cards.iter().zip(other.cards.iter()) {
            match s.cmp(&o) {
                Ordering::Equal => continue,
                o => return o,
            }
        }
        return Ordering::Equal;
    }

    fn compare_types(&self, other: &Hand) -> Ordering {
        self.get_type().cmp(&other.get_type())
    }

    fn compare(&self, other: &Hand) -> Ordering {
        match (self.compare_types(other), self.compare_cards(other)) {
            (Ordering::Equal, Ordering::Equal) => Ordering::Equal,
            (Ordering::Equal, Ordering::Greater) => Ordering::Greater,
            (Ordering::Equal, Ordering::Less) => Ordering::Less,
            (Ordering::Greater, _) => Ordering::Greater,
            (Ordering::Less, _) => Ordering::Less,
        }
    }
}

#[derive(Debug)]
pub struct Day7Of2023 {
    data: Vec<Hand>,
}

impl Day7Of2023 {
    pub fn new() -> Self {
        Self {
            data: Vec::new(),
        }
    }
}

impl Day for Day7Of2023 {
    fn get_day(&self) -> (i32, i32) {
        (2023, 7)
    }

    fn parse(&mut self, data: String) {
        self.data = data.lines()
            .map(|line|
                line.split_once(" ")
                    .map(|(hand, bid)| Hand::new(hand, bid))
                    .unwrap())
            .collect();
        self.data.sort_by(|a, b| a.compare(b));
    }

    fn task1(&self) -> String {
        self.data.iter().enumerate()
            .map(|(i, hand)| (i as i32 + 1) * hand.bid)
            .sum::<i32>().to_string()
    }

    fn task2(&self) -> String {
        let mut data = self.data.iter().clone().map(|hand| Hand {
            cards: hand.cards.map(|c| *c.jack_to_joker()),
            bid: hand.bid,
        }).collect::<Vec<Hand>>();
        data.sort_by(|a, b| a.compare(b));
        data.iter().enumerate()
            .map(|(i, hand)| (i as i32 + 1) * hand.bid)
            .sum::<i32>().to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "32T3K 765\nT55J5 684\nKK677 28\nKTJJT 220\nQQQJA 483";

    #[test]
    fn task_1() {
        let mut day = Day7Of2023::new();
        day.parse(INPUT.to_string());
        assert_eq!(day.task1(), "6440");
    }

    #[test]
    fn task_2() {
        let mut day = Day7Of2023::new();
        day.parse(INPUT.to_string());
        assert_eq!(day.task2(), "5905");
    }
}
