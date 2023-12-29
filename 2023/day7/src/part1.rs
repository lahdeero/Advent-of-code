use std::cmp::Ordering;

#[derive(Debug, PartialEq, Clone)]
pub enum Rank {
    HighCard,
    OnePair,
    TwoPairs,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Debug)]
pub struct Player {
    pub hand: String,
    pub bid: i64,
    pub rank: Rank,
}

// 250_682_047 is too low
// 250_951_660

pub fn solve(hands: Vec<String>, bids: Vec<i64>) -> i64 {
    let mut players: Vec<Player> = Vec::new();
    for (i, hand) in hands.iter().enumerate() {
        let rank = evaluate(hand);
        let player = Player {
            hand: hand.to_string(),
            bid: bids[i],
            rank,
        };
        players.push(player);
    }

    players.sort_by(|a, b| compare(a, b));

    let mut result: i64 = 0;
    for (i, player) in players.iter().enumerate() {
        // println!("Player: {:?}", player);
        // println!(
        //     "{} * {} = {}",
        //     player.bid,
        //     i + 1,
        //     player.bid * (i + 1) as i64
        // );
        result += player.bid * (i + 1) as i64;
    }

    return result;
}

pub fn compare(player1: &Player, player2: &Player) -> Ordering {
    if player1.rank == player2.rank {
        for i in 0..player1.hand.len() {
            let player1_card = to_number_value(player1.hand.chars().nth(i).unwrap());
            let player2_card = to_number_value(player2.hand.chars().nth(i).unwrap());
            if player1_card > player2_card {
                return Ordering::Greater;
            } else if player1_card < player2_card {
                return Ordering::Less;
            }
        }
    }
    let cmp = player1.rank.clone() as i64 - player2.rank.clone() as i64;
    if cmp > 0 {
        return Ordering::Greater;
    } else if cmp < 0 {
        return Ordering::Less;
    }
    return Ordering::Equal;
}

fn to_number_value(char: char) -> i64 {
    // parse char to number
    // Attempt to convert the character to a digit
    let digit = char.to_digit(10);
    match digit {
        Some(d) => d as i64,
        None => match char {
            'T' => 10,
            'J' => 11,
            'Q' => 12,
            'K' => 13,
            'A' => 14,
            _ => panic!("Invalid character"),
        },
    }
}

pub fn evaluate(hand: &String) -> Rank {
    let mut card_pairs: Vec<(char, i32)> = vec![
        ('2', 0),
        ('3', 0),
        ('4', 0),
        ('5', 0),
        ('6', 0),
        ('7', 0),
        ('8', 0),
        ('9', 0),
        ('T', 0),
        ('J', 0),
        ('Q', 0),
        ('K', 0),
        ('A', 0),
    ];
    for char in hand.chars() {
        for card_pair in card_pairs.iter_mut() {
            if card_pair.0 == char {
                card_pair.1 += 1;
            }
        }
    }

    let mut current_rank = Rank::HighCard;
    for card_pair in card_pairs.iter() {
        if current_rank == Rank::HighCard {
            if card_pair.1 == 2 {
                current_rank = Rank::OnePair;
            } else if card_pair.1 == 3 {
                current_rank = Rank::ThreeOfAKind;
            } else if card_pair.1 == 4 {
                current_rank = Rank::FourOfAKind;
            } else if card_pair.1 == 5 {
                current_rank = Rank::FiveOfAKind;
            }
        } else if current_rank == Rank::OnePair {
            if card_pair.1 == 2 {
                current_rank = Rank::TwoPairs;
            } else if card_pair.1 == 3 {
                current_rank = Rank::FullHouse;
            }
        } else if current_rank == Rank::ThreeOfAKind {
            if card_pair.1 == 2 {
                current_rank = Rank::FullHouse;
            }
        }
    }
    return current_rank;
}
