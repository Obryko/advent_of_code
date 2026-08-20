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
    fn jack_to_joker(self) -> Card {
        match self {
            Card::Jack => Card::Joker,
            _ => self
        }
    }
}

impl TryFrom<char> for Card {
    type Error = char;
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '2' => Ok(Card::Two),
            '3' => Ok(Card::Three),
            '4' => Ok(Card::Four),
            '5' => Ok(Card::Five),
            '6' => Ok(Card::Six),
            '7' => Ok(Card::Seven),
            '8' => Ok(Card::Eight),
            '9' => Ok(Card::Nine),
            'T' => Ok(Card::Ten),
            'J' => Ok(Card::Jack),
            'Q' => Ok(Card::Queen),
            'K' => Ok(Card::King),
            'A' => Ok(Card::Ace),
            value => Err(value)
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
    hand_type: HandType
}

impl From<(&str, &str)> for Hand {
    fn from(value: (&str, &str)) -> Self {
        let cards: [Card; 5] = value.0.chars().map(Card::try_from).collect::<Result<Vec<Card>, _>>().expect("Invalid hand").try_into().expect("Invalid hand");
        let bid = value.1.parse::<i32>().expect("Invalid bid");
        Self::new(cards, bid)
    }
}

impl Hand {
    fn new(cards: [Card;5], bid: i32) -> Self {
        let hand_type = Self::get_type(&cards);
        Self {
            cards,
            bid,
            hand_type,
        }
    }

    fn get_type(cards: &[Card]) -> HandType {
        let mut counts = HashMap::new();

        for &card in cards {
            *counts.entry(card).or_insert(0) += 1;
        }

        let jokers = counts.remove(&Card::Joker).unwrap_or(0);

        let mut counts = counts.values().copied().collect::<Vec<_>>();

        if counts.is_empty() {
            counts.push(jokers);
        } else {
            let max = counts.iter_mut().max().unwrap();
            *max += jokers;
        }

        counts.sort_unstable_by(|a, b| b.cmp(a));

        match counts.as_slice() {
            [5] => HandType::FiveOfAKind,
            [4, 1] => HandType::FourOfAKind,
            [3, 2] => HandType::FullHouse,
            [3, 1, 1] => HandType::ThreeOfAKind,
            [2, 2, 1] => HandType::TwoPair,
            [2, 1, 1, 1] => HandType::OnePair,
            [1, 1, 1, 1, 1] => HandType::HighCard,
            _ => panic!("Invalid hand"),
        }
    }
}

impl Eq for Hand {}

impl PartialEq<Self> for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.cards == other.cards
    }
}

impl PartialOrd<Self> for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        self.hand_type
            .cmp(&other.hand_type)
            .then_with(|| self.cards.cmp(&other.cards))
    }
}


#[derive(Debug, Default)]
pub struct Day7Of2023 {
    data: Vec<Hand>,
}

impl Day7Of2023 {
     fn total_winnings(data: &[Hand]) -> i32 {
         data.iter().enumerate()
             .map(|(i, hand)| (i as i32 + 1) * hand.bid)
             .sum()
     }
}

impl Day for Day7Of2023 {
    fn parse(&mut self, data: String) {
        self.data = data.lines()
            .map(|line| line.split_once(' ').map(Hand::from).unwrap())
            .collect();
        self.data.sort();
    }

    fn task1(&self) -> String {
        Self::total_winnings(&self.data).to_string()
    }

    fn task2(&self) -> String {
        let mut data = self.data.iter().map(|hand|
            Hand::new(hand.cards.map(Card::jack_to_joker), hand.bid)
        ).collect::<Vec<_>>();
        data.sort();
        Self::total_winnings(&data).to_string()
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
