use std::cmp::Ordering;
use std::collections::HashMap;
use crate::Day;


#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
enum Card {
    Joker = 1,
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
    Six = 6,
    Seven = 7,
    Eight = 8,
    Nine = 9,
    Ten = 10,
    Jack = 11,
    Queen = 12,
    King = 13,
    Ace = 14,
}

impl Card {
    fn from_str(s: char) -> Card {
        match s {
            '2' => Card::Two,
            '3' => Card::Three,
            '4' => Card::Four,
            '5' => Card::Five,
            '6' => Card::Six,
            '7' => Card::Seven,
            '8' => Card::Eight,
            '9' => Card::Nine,
            'T' => Card::Ten,
            'J' => Card::Jack,
            'Q' => Card::Queen,
            'K' => Card::King,
            'A' => Card::Ace,
            _ => panic!("Invalid card")
        }
    }

    fn jack_to_joker(&self) -> &Card {
        match self {
            Card::Jack => &Card::Joker,
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
        let jokers = types.remove(&Card::Joker).unwrap_or(0);

        if types.is_empty() {
            types.insert(Card::Joker, jokers);
        } else {
            let card = types.iter().max_by_key(|&(_, i)| i).map(|(&card, _)| card).unwrap();
            types.entry(card).and_modify(|i| *i += jokers);
        }

        Hand::get_type_without_joker(types)
    }

    fn get_type(&self) -> HandType {
        let mut types: HashMap<Card, usize> = HashMap::new();
        self.cards.iter().for_each(|&card| *types.entry(card).or_insert(0) += 1);

        match types.contains_key(&Card::Joker) {
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
        Ordering::Equal
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

#[derive(Debug, Default)]
pub struct Day7Of2023 {
    data: Vec<Hand>,
}

impl Day for Day7Of2023 {
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
        let mut day = Day7Of2023::default();
        day.parse(INPUT.to_string());
        assert_eq!(day.task1(), "6440");
    }

    #[test]
    fn task_2() {
        let mut day = Day7Of2023::default();
        day.parse(INPUT.to_string());
        assert_eq!(day.task2(), "5905");
    }
}
