use std::cmp::Ordering;

use day7::part1::compare;
use day7::part1::evaluate;
use day7::part1::Player;
use day7::part1::Rank;

#[test]
fn it_compares_when_same_ranks() {
    let player1 = Player {
        hand: "T55J5".to_string(),
        bid: 1,
        rank: Rank::ThreeOfAKind,
    };
    let player2 = Player {
        hand: "QQQJA".to_string(),
        bid: 1,
        rank: Rank::ThreeOfAKind,
    };
    assert_eq!(Ordering::Less, compare(&player1, &player2));
}

#[test]
fn it_compares_by_rank() {
    let player1 = Player {
        hand: "23456".to_string(),
        bid: 1,
        rank: Rank::HighCard,
    };
    let player2 = Player {
        hand: "AA234".to_string(),
        bid: 1,
        rank: Rank::OnePair,
    };
    assert_eq!(Ordering::Less, compare(&player1, &player2));
}

#[test]
fn it_evaluates_full_house_aces_full_of_deuces() {
    assert_eq!(Rank::FullHouse, evaluate(&String::from("A2A2A")));
}

#[test]
fn it_evaluates_full_house_sixs_full_of_queens() {
    assert_eq!(Rank::FullHouse, evaluate(&String::from("6QQ66")));
}
