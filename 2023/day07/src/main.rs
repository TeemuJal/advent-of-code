use std::{cmp::Ordering, collections::HashSet, fs::read_to_string};

const JOKER: char = 'J';
const CARDS: [char; 13] = [
    JOKER, '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'Q', 'K', 'A',
];

struct Hand {
    cards: [char; 5],
    hand_type: u8,
}

fn main() {
    let input = read_to_string("input2.txt").unwrap();
    let mut hand_bid_pairs: Vec<(Hand, usize)> =
        input.lines().map(parse_line_to_hand_bid_pair).collect();
    hand_bid_pairs.sort_by(compare_hand_bid_pair);
    let total_winnings: usize = hand_bid_pairs
        .iter()
        .enumerate()
        .map(|(idx, (_, bid))| bid * (idx + 1))
        .sum();
    println!("Part two result is {total_winnings}");
}

fn compare_hand_bid_pair((hand_1, _): &(Hand, usize), (hand_2, _): &(Hand, usize)) -> Ordering {
    let Hand {
        cards: cards_1,
        hand_type: hand_type_1,
    } = *hand_1;
    let Hand {
        cards: cards_2,
        hand_type: hand_type_2,
    } = *hand_2;
    if hand_type_1 != hand_type_2 {
        return hand_type_1.cmp(&hand_type_2);
    }
    for (idx, card_1) in cards_1.iter().enumerate() {
        let card_1 = *card_1;
        let card_2 = cards_2[idx];
        let card_1_strength = CARDS.iter().position(|card| *card == card_1).unwrap();
        let card_2_strength = CARDS.iter().position(|card| *card == card_2).unwrap();
        if card_1_strength != card_2_strength {
            return card_1_strength.cmp(&card_2_strength);
        }
    }
    return Ordering::Equal;
}

fn parse_line_to_hand_bid_pair(str: &str) -> (Hand, usize) {
    let split = str.split_once(" ").unwrap();
    let bid: usize = split.1.parse().unwrap();
    let hand = split.0;
    assert_eq!(hand.len(), 5);
    let mut hand_chars = hand.chars();
    let cards: [char; 5] = [
        hand_chars.nth(0).unwrap(),
        hand_chars.nth(0).unwrap(),
        hand_chars.nth(0).unwrap(),
        hand_chars.nth(0).unwrap(),
        hand_chars.nth(0).unwrap(),
    ];
    let hand_type = get_hand_type(cards);
    return (Hand { cards, hand_type }, bid);
}

fn get_hand_type(cards: [char; 5]) -> u8 {
    let [first, second, third, fourth, fifth] = cards;
    let unique_cards_count = HashSet::from(cards).len();
    let joker_count = cards.iter().filter(|card| **card == JOKER).count();

    let first_card_count = cards.iter().filter(|card| **card == first).count();
    let second_card_count = cards.iter().filter(|card| **card == second).count();
    let third_card_count = cards.iter().filter(|card| **card == third).count();
    let fourth_card_count = cards.iter().filter(|card| **card == fourth).count();
    let fifth_card_count = cards.iter().filter(|card| **card == fifth).count();

    let three_same_chars = first_card_count == 3 || second_card_count == 3 || third_card_count == 3;

    let pairs_count = [
        first_card_count == 2,
        second_card_count == 2,
        third_card_count == 2,
        fourth_card_count == 2,
        fifth_card_count == 2,
    ]
    .iter()
    .filter(|boolean| **boolean)
    .count()
        / 2;

    // Five of a kind
    if unique_cards_count == 1 || (unique_cards_count == 2 && joker_count > 0) {
        return 6;
    }
    // Four of a kind
    if first_card_count == 4
        || second_card_count == 4
        || joker_count == 3
        || (pairs_count == 2 && joker_count == 2)
        || three_same_chars && joker_count == 1
    {
        return 5;
    }
    // Full house
    if unique_cards_count == 3 && joker_count > 0 || unique_cards_count == 2 {
        return 4;
    }
    // Three of a kind
    if three_same_chars || joker_count == 2 || (pairs_count == 1 && joker_count == 1) {
        return 3;
    }
    // Two pairs
    if pairs_count == 2 {
        return 2;
    }
    // One pair
    if pairs_count == 1 || joker_count == 1 {
        return 1;
    }
    return 0;
}
