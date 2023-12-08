use std::fs::read_to_string;



#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
enum Card {
    Value(u8),
    T = 10,
    J = 11,
    Q = 12,
    K = 13,
    A = 14,
}


// implement from trait for card rank
impl From<char> for Card {
    fn from(c: char) -> Self {
        match c {
            '1'..='9' => Card::Value(c.to_digit(10).unwrap() as u8),
            'T' => Card::T,
            'J' => Card::J,
            'Q' => Card::Q,
            'K' => Card::K,
            'A' => Card::A,
            _ => panic!("Invalid card character"),
        }
    }
}

// implement fmt trait
impl std::fmt::Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Card::Value(value) => write!(f, "{}", value),
            Card::T => write!(f, "T"),
            Card::J => write!(f, "J"),
            Card::Q => write!(f, "Q"),
            Card::K => write!(f, "K"),
            Card::A => write!(f, "A"),
        }
    }
}

// A Hand of five cards
// First data field is biggest tuple, second is second highest etc.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum HandOfFive {
    HighCard(Vec<Card>),
    OnePair(Vec<Card>),
    TwoPair(Vec<Card>),
    ThreeOfAKind(Vec<Card>),
    FullHouse(Vec<Card>),
    FourOfAKind(Vec<Card>),
    FiveOfAKind(Vec<Card>),
}

impl std::fmt::Display for HandOfFive {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            HandOfFive::HighCard(cards) => write!(f, "HighCard: {}", cards.iter().map(|card| card.to_string()).collect::<Vec<String>>().join(" ")),
            HandOfFive::OnePair(cards) => write!(f, "OnePair: {}", cards.iter().map(|card| card.to_string()).collect::<Vec<String>>().join(" ")),
            HandOfFive::TwoPair(cards) => write!(f, "TwoPair: {}", cards.iter().map(|card| card.to_string()).collect::<Vec<String>>().join(" ")),
            HandOfFive::ThreeOfAKind(cards) => write!(f, "ThreeOfAKind: {}", cards.iter().map(|card| card.to_string()).collect::<Vec<String>>().join(" ")),
            HandOfFive::FullHouse(cards) => write!(f, "FullHouse: {}", cards.iter().map(|card| card.to_string()).collect::<Vec<String>>().join(" ")),
            HandOfFive::FourOfAKind(cards) => write!(f, "FourOfAKind: {}", cards.iter().map(|card| card.to_string()).collect::<Vec<String>>().join(" ")),
            HandOfFive::FiveOfAKind(cards) => write!(f, "FiveOfAKind: {}", cards.iter().map(|card| card.to_string()).collect::<Vec<String>>().join(" ")),
        }
    }
}

impl From<Vec<char>> for HandOfFive {
    fn from(cards: Vec<char>) -> Self {
        let cards: Vec<Card> = cards.into_iter().map(|c| Card::from(c)).collect();

        let mut card_counts: std::collections::HashMap<Card, u8> = std::collections::HashMap::new();

        for card in cards.iter() {
            let count = card_counts.entry(*card).or_insert(0);
            *count += 1;
        }

        let mut card_counts: Vec<(Card, u8)> = card_counts.into_iter().collect();
        card_counts.sort_by(|a, b| b.1.cmp(&a.1));

        let mut card_counts: Vec<u8> = card_counts.into_iter().map(|(_, count)| count).collect();
        card_counts.sort();
        card_counts.reverse();

        match card_counts.as_slice() {
            [1, 1, 1, 1, 1] => HandOfFive::HighCard(cards),
            [2, 1, 1, 1] => HandOfFive::OnePair(cards),
            [2, 2, 1] => HandOfFive::TwoPair(cards),
            [3, 1, 1] => HandOfFive::ThreeOfAKind(cards),
            [3, 2] => HandOfFive::FullHouse(cards),
            [4, 1] => HandOfFive::FourOfAKind(cards),
            [5] => HandOfFive::FiveOfAKind(cards),
            _ => panic!("Invalid card counts"),
        }
    }
}


#[derive(Debug, Clone, PartialEq, Eq, PartialOrd)]
struct BiddingHand {
    hand: HandOfFive,
    bid: u64,
}

impl Ord for BiddingHand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.hand.cmp(&other.hand)
    }
}

impl std::fmt::Display for BiddingHand {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "BiddingHand {{\n  {} \n  Bid:{}\n}}\n", self.hand, self.bid)
    }
}


fn main() -> std::io::Result<()> {

    let input_file = "input.txt";

    println!("Input file: {:?}", input_file);

    // read file line by line
    let file_str = read_to_string(input_file)?;

    let cards_bids_tuple_vec: Vec<(&str, &str)> = file_str
        .lines()
        .filter_map(|line| {
            let mut parts = line.split_whitespace();
            if let (Some(cards_str), Some(bid_str)) = (parts.next(), parts.next()) {
                Some((cards_str, bid_str))
            } else {
                None
            }
        })
        .collect();

    let (hands_str, bids_str): (Vec<&str>, Vec<&str>) = cards_bids_tuple_vec.into_iter().unzip();

    let mut bidding_hands: Vec<BiddingHand> = Vec::new();
    
    for (hand, bid) in hands_str.iter().zip(bids_str.iter()) {
        // create a BiddingHand for each line
        bidding_hands.push(
            BiddingHand {
                hand: HandOfFive::from(hand.chars().collect::<Vec<char>>()),
                bid: bid.parse::<u64>().unwrap(),
            }
        );
    }


    bidding_hands.sort();
    let game_ranks = 1..=bidding_hands.len();

    for bidding_hand in bidding_hands.iter() {
        println!("{}", bidding_hand);
    }

    // multiply bid by game rank
    let total_win: u64 = bidding_hands
        .iter()
        .zip(game_ranks)
        .map(|(bidding_hand, game_rank)| bidding_hand.bid * game_rank as u64)
        .sum();

    println!("Total winnings: {}", total_win);

    Ok(())
}

