use std::{cmp::Ordering, collections::HashMap, fmt::Error, str::FromStr};

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Card {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

impl From<char> for Card {
    fn from(card: char) -> Self {
        match card {
            'A' => Self::Ace,
            'K' => Self::King,
            'Q' => Self::Queen,
            'J' => Self::Jack,
            'T' => Self::Ten,
            '9' => Self::Nine,
            '8' => Self::Eight,
            '7' => Self::Seven,
            '6' => Self::Six,
            '5' => Self::Five,
            '4' => Self::Four,
            '3' => Self::Three,
            '2' => Self::Two,
            _ => panic!(),
        }
    }
}

impl From<[char; 5]> for HandType {
    fn from(cards: [char; 5]) -> Self {
        let mut card_map = HashMap::new();
        cards
            .iter()
            .for_each(|c| *card_map.entry(c).or_insert(0) += 1);
        if card_map.keys().len() == 1 {
            return HandType::FiveOfAKind;
        } else if card_map.keys().len() == 2 {
            if card_map.values().any(|&v| v == 4) {
                return HandType::FourOfAKind;
            } else {
                return HandType::FullHouse;
            }
        } else if card_map.keys().len() == 3 {
            if card_map.values().any(|&v| v == 3) {
                return HandType::ThreeOfAKind;
            } else {
                return HandType::TwoPair;
            }
        } else if card_map.keys().len() == 4 {
            return HandType::OnePair;
        } else {
            return HandType::HighCard;
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Hand {
    cards: [Card; 5],
    bid: u32,
    hand_type: HandType,
}

impl Hand {
    pub fn new(cards: [char; 5], bid: u32) -> Hand {
        Hand {
            cards: cards.map(|c| c.into()),
            bid,
            hand_type: HandType::from(cards),
        }
    }
}

impl FromStr for Hand {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (cards, bid) = s.split_once(' ').ok_or(Error)?;
        let cards_fromstr: [char; 5] =
            cards.chars().enumerate().fold([' '; 5], |mut acc, (i, x)| {
                acc[i] = x;
                acc
            });
        let bid_fromstr = bid.parse::<u32>().map_err(|_| Error)?;
        Ok(Hand::new(cards_fromstr, bid_fromstr))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.hand_type != other.hand_type {
            return self.hand_type.cmp(&other.hand_type);
        } else {
            for i in 0..5 {
                let comparison = self.cards[i].cmp(&other.cards[i]);
                if comparison != Ordering::Equal {
                    return comparison;
                }
            }
            return Ordering::Equal;
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub fn solution(input: &str) -> Result<u32, Error> {
    let mut hands: Vec<Hand> = input
        .lines()
        .map(|line| line.parse::<Hand>())
        .into_iter()
        .try_collect()
        .map_err(|e| e)?;

    // println!("{:?}", hands);
    hands.sort();
    let result = hands.iter().enumerate().fold(0, |acc, (i, h)| {
        // println!("{}: {:?}", i + 1, h);
        acc + (i as u32 + 1) * h.bid
    });
    Ok(result)
}
