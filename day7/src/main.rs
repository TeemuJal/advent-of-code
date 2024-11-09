use std::{cmp::Ordering, collections::HashSet, fs::read_to_string};

const CARDS: [char; 13] = [
    '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K', 'A',
];

#[derive(Debug)]
struct Hand {
    cards: [char; 5],
    hand_type: u8,
}

fn main() {
    let input = read_to_string("input2.txt").unwrap();
    let mut hand_bid_pairs: Vec<(Hand, usize)> =
        input.lines().map(parse_line_to_hand_bid_pair).collect();
    hand_bid_pairs.sort_by(compare_hand_bid_pair);
    let part_one_result: usize = hand_bid_pairs.iter().enumerate().map(|(idx, (_, bid))| bid * (idx + 1)).sum();
    println!("Part one result is {part_one_result}");
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
    Ordering::Equal
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
    let unique_cards_len = HashSet::from(cards).len();
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
    let three_first_chars = cards.iter().filter(|card| **card == first).count() == 3;
    let three_second_chars = cards.iter().filter(|card| **card == second).count() == 3;
    let three_third_chars = cards.iter().filter(|card| **card == third).count() == 3;
    if three_first_chars || three_second_chars || three_third_chars {
        // Full house
        if unique_cards_len == 2 {
            return 4;
        }
        // Three of a kind
        return 3;
    }
    let two_first_chars = cards.iter().filter(|card| **card == first).count() == 2;
    let two_second_chars = cards.iter().filter(|card| **card == second).count() == 2;
    let two_third_chars = cards.iter().filter(|card| **card == third).count() == 2;
    let two_fourth_chars = cards.iter().filter(|card| **card == fourth).count() == 2;
    let two_fifth_chars = cards.iter().filter(|card| **card == fifth).count() == 2;

    let pairs_count = [
        two_first_chars,
        two_second_chars,
        two_third_chars,
        two_fourth_chars,
        two_fifth_chars,
    ]
    .iter()
    .filter(|boolean| **boolean)
    .count()
        / 2;

    // Two pairs
    if pairs_count == 2 {
        return 2;
    }
    // One pair
    if pairs_count == 1 {
        return 1;
    }
    return 0;
}
