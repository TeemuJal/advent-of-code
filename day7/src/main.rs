use std::{collections::HashMap, fs::read_to_string};

const CARDS: [char; 13] = [
    '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K', 'A',
];
//const HAND_TYPE_VALUE_MAP: HashMap<String, u8> = HashMap::from([()])

struct Hand {
    cards: [char; 5],
    hand_type: u8,
}

fn main() {
    let input = read_to_string("input1.txt").unwrap();
    let handBidPairs: Vec<(Hand, usize)> = input.lines().map(parse_line_to_hand_bid_pair).collect();
}

fn parse_line_to_hand_bid_pair(str: &str) -> (Hand, usize) {
    let split = str.split_once(" ").unwrap();
    let bid: usize = split.1.parse().unwrap();

    let hand = split.0;
    assert_eq!(hand.len(), 5);
    let mut hand_chars = hand.chars();
    let cards: [char; 5] = [
        hand_chars.nth(0).unwrap(),
        hand_chars.nth(1).unwrap(),
        hand_chars.nth(2).unwrap(),
        hand_chars.nth(3).unwrap(),
        hand_chars.nth(4).unwrap(),
    ];
    
    let hand_type = get_hand_type(cards);

    return (Hand { cards, hand_type }, bid);
}

fn get_hand_type(cards: [char; 5]) -> u8 {
    let first = cards[0];
    let second = cards[1];
    let third = cards[2];
    // TODO HashSet to get unique card count
    // Five of a kind
    if cards.iter().all(|card| *card == first) {
        return 6;
    }
    // Four of a kind
    let four_first_chars = cards.iter().filter(|card| **card == first).count() == 4;
    let four_second_chars = cards.iter().filter(|card| **card == second).count() == 4;
    if four_first_chars || four_second_chars {
        return 5;
    }
    // Full house
    // Three of a kind
    let three_first_chars = cards.iter().filter(|card| **card == first).count() == 3;
    let three_second_chars = cards.iter().filter(|card| **card == second).count() == 3;
    let three_third_chars = cards.iter().filter(|card| **card == third).count() == 3;
    if three_first_chars || three_second_chars || three_third_chars {
        return 3;
    }
    return 0;
}
