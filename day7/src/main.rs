use std::fs::read_to_string;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
// enum Card {
//     Value(u8),
//     T = 10,
//     J = 11,
//     Q = 12,
//     K = 13,
//     A = 14,
// } // part 1
enum Card {
    J = 1,
    Value(u8), // 2..9
    T = 10,
    Q = 11,
    K = 12,
    A = 13,
} // part 2

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
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd)]
enum HandOfFive {
    HighCard(Vec<Card>),
    OnePair(Vec<Card>),
    TwoPair(Vec<Card>),
    ThreeOfAKind(Vec<Card>),
    FullHouse(Vec<Card>),
    FourOfAKind(Vec<Card>),
    FiveOfAKind(Vec<Card>),
}

impl HandOfFive {
    fn cards(&self) -> &Vec<Card> {
        match self {
            HandOfFive::HighCard(cards) => cards,
            HandOfFive::OnePair(cards) => cards,
            HandOfFive::TwoPair(cards) => cards,
            HandOfFive::ThreeOfAKind(cards) => cards,
            HandOfFive::FullHouse(cards) => cards,
            HandOfFive::FourOfAKind(cards) => cards,
            HandOfFive::FiveOfAKind(cards) => cards,
        }
    }

    fn apply_jokers(&self) -> HandOfFive {
        let hand_cards = self.cards();
        // vec of all cards except joker
        let variants: Vec<Card> = vec![
            Card::Value(2),
            Card::Value(3),
            Card::Value(4),
            Card::Value(5),
            Card::Value(6),
            Card::Value(7),
            Card::Value(8),
            Card::Value(9),
            Card::T,
            Card::Q,
            Card::K,
            Card::A,
        ];
        let possible_hands: Vec<HandOfFive> = replace_jokers(hand_cards.clone(), variants, 0)
            .into_iter()
            .map(HandOfFive::from)
            .collect();
        if possible_hands.is_empty() {
            return self.clone();
        } else {
            possible_hands.iter().max().unwrap().clone()
        }
    }
}


impl From<Vec<char>> for HandOfFive {
    fn from(cards: Vec<char>) -> Self {
        let cards: Vec<Card> = cards.into_iter().map(|c| Card::from(c)).collect();
        Self::from(cards)
    }
}

impl From<Vec<Card>> for HandOfFive {
    fn from(cards: Vec<Card>) -> Self {
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

impl Ord for HandOfFive {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (HandOfFive::HighCard(cards), HandOfFive::HighCard(other_cards)) => cards.cmp(other_cards),
            (HandOfFive::HighCard(_), _) => std::cmp::Ordering::Less,
            (_, HandOfFive::HighCard(_)) => std::cmp::Ordering::Greater,

            (HandOfFive::OnePair(cards), HandOfFive::OnePair(other_cards)) => cards.cmp(other_cards),
            (HandOfFive::OnePair(_), _) => std::cmp::Ordering::Less,
            (_, HandOfFive::OnePair(_)) => std::cmp::Ordering::Greater,

            // Repeat for other hand types...
            (HandOfFive::TwoPair(cards), HandOfFive::TwoPair(other_cards)) => cards.cmp(other_cards),
            (HandOfFive::TwoPair(_), _) => std::cmp::Ordering::Less,
            (_, HandOfFive::TwoPair(_)) => std::cmp::Ordering::Greater,

            (HandOfFive::ThreeOfAKind(cards), HandOfFive::ThreeOfAKind(other_cards)) => cards.cmp(other_cards),
            (HandOfFive::ThreeOfAKind(_), _) => std::cmp::Ordering::Less,
            (_, HandOfFive::ThreeOfAKind(_)) => std::cmp::Ordering::Greater,

            (HandOfFive::FullHouse(cards), HandOfFive::FullHouse(other_cards)) => cards.cmp(other_cards),
            (HandOfFive::FullHouse(_), _) => std::cmp::Ordering::Less,
            (_, HandOfFive::FullHouse(_)) => std::cmp::Ordering::Greater,

            (HandOfFive::FourOfAKind(cards), HandOfFive::FourOfAKind(other_cards)) => cards.cmp(other_cards),
            (HandOfFive::FourOfAKind(_), _) => std::cmp::Ordering::Less,
            (_, HandOfFive::FourOfAKind(_)) => std::cmp::Ordering::Greater,

            (HandOfFive::FiveOfAKind(cards), HandOfFive::FiveOfAKind(other_cards)) => cards.cmp(other_cards),
            //(HandOfFive::FiveOfAKind(_), _) => std::cmp::Ordering::Greater,
            //(_, HandOfFive::FiveOfAKind(_)) => std::cmp::Ordering::Less,
        }
    }
}



#[derive(Debug, Clone, PartialEq, Eq, PartialOrd)]
struct BiddingHand {
    hand: HandOfFive,
    strongest_variation: HandOfFive,
    bid: u64,
}

impl Ord for BiddingHand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.strongest_variation.cmp(&other.strongest_variation) {
            std::cmp::Ordering::Equal => self.hand.cmp(&other.hand),
            other => other,
        }
    }
}

impl std::fmt::Display for BiddingHand {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "BiddingHand {{\n  Cards: {:?} \n  Best: {:?}\n  Bid: {}\n}}\n",
            self.hand, self.strongest_variation, self.bid
        )
    }
}

fn main() -> std::io::Result<()> {
    let input_file = "sample.txt";

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
        let hand = HandOfFive::from(hand.chars().collect::<Vec<char>>());
        bidding_hands.push(BiddingHand {
            hand: hand.clone(),
            strongest_variation: hand.apply_jokers(),
            bid: bid.parse::<u64>().unwrap(),
        });
    }

    bidding_hands.sort();
    let game_ranks = 1..=bidding_hands.len();


    let ranked_hands: Vec<(usize, BiddingHand)> = game_ranks
        .into_iter()
        .zip(bidding_hands.into_iter())
        .collect();

    let mut total_win: u64 = 0;
    for (rank, hand) in ranked_hands {
        println!("{}: {}", rank, hand);
        total_win += rank as u64 * hand.bid;
    }

    println!("Total winnings: {}", total_win);

    Ok(())
}


fn replace_jokers(hand_cards: Vec<Card>, variants: Vec<Card>, i: usize) -> Vec<Vec<Card>> {
    if i >= hand_cards.len() {
        return vec![hand_cards];
    }

    if hand_cards[i] != Card::J {
        return replace_jokers(hand_cards, variants, i + 1);
    }

    let mut hands = Vec::new();
    for x in 2..=13 {
        let mut new_hand = hand_cards[0..i].to_vec();
        new_hand.push(variants[x-2].clone());
        new_hand.extend_from_slice(&hand_cards[i+1..]);
        hands.extend(replace_jokers(new_hand, variants.clone(), i + 1));
    }
    hands
}