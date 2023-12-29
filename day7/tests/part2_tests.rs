use std::cmp::Ordering;

use day7::part2::compare;
use day7::part2::evaluate;
use day7::part2::Player;
use day7::part2::Rank;

#[test]
fn it_compares_by_rank() {
    let player1 = Player {
        hand: "JJ234".to_string(),
        bid: 1,
        rank: Rank::ThreeOfAKind,
    };
    let player2 = Player {
        hand: "22234".to_string(),
        bid: 1,
        rank: Rank::ThreeOfAKind,
    };
    assert_eq!(Ordering::Less, compare(&player1, &player2));
}

#[test]
fn it_compares_joker_is_less_than_deuce() {
    let player1 = Player {
        hand: "JJJJJ".to_string(),
        bid: 1,
        rank: Rank::FiveOfAKind,
    };
    let player2 = Player {
        hand: "22222".to_string(),
        bid: 1,
        rank: Rank::FiveOfAKind,
    };
    assert_eq!(Ordering::Less, compare(&player1, &player2));
}

#[test]
fn it_evaluates_high_card() {
    assert_eq!(Rank::HighCard, evaluate(&String::from("9TQKA")));
}

#[test]
fn it_evaluates_pair_with_joker() {
    assert_eq!(Rank::OnePair, evaluate(&String::from("2345J")));
}

#[test]
fn it_evaluates_three_of_kind_with_joker() {
    assert_eq!(Rank::ThreeOfAKind, evaluate(&String::from("232JQ")));
}

#[test]
fn it_evaluates_three_of_kind_with_two_jokers() {
    assert_eq!(Rank::ThreeOfAKind, evaluate(&String::from("TQJJK")));
}

#[test]
fn it_evaluates_full_house_with_joker() {
    assert_eq!(Rank::FullHouse, evaluate(&String::from("2AJA2")));
}

#[test]
fn it_evaluates_four_of_kind_with_joker() {
    assert_eq!(Rank::FourOfAKind, evaluate(&String::from("7767J")));
}

#[test]
fn it_evaluates_four_of_kind_with_two_jokers() {
    assert_eq!(Rank::FourOfAKind, evaluate(&String::from("JJ898")));
}

#[test]
fn it_evaluates_four_of_kind_with_three_jokers() {
    assert_eq!(Rank::FourOfAKind, evaluate(&String::from("4JTJJ")));
}

#[test]
fn it_evaluates_five_of_kind_with_joker() {
    assert_eq!(Rank::FiveOfAKind, evaluate(&String::from("TTJTT")));
}

#[test]
fn it_evaluates_five_of_kind_with_jokers() {
    assert_eq!(Rank::FiveOfAKind, evaluate(&String::from("JJ5JJ")));
}

#[test]
fn it_evaluates_five_of_kind_with_only_jokers() {
    assert_eq!(Rank::FiveOfAKind, evaluate(&String::from("JJJJJ")));
}
