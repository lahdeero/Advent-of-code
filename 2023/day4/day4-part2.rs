#[derive(Debug)]
struct Card {
    winning_numbers: Vec<u32>,
    played_numbers: Vec<u32>,
}

fn string_numbers_to_numbers(string_numbers: &str) -> Vec<u32> {
    let number_strings = string_numbers.split(" ").collect::<Vec<&str>>();
    number_strings
        .iter()
        .filter(|x| !x.is_empty())
        .map(|x| x.parse::<u32>().unwrap())
        .collect::<Vec<u32>>()
}

fn create_card(raw_winning_numbers: &str, raw_played_numbers: &str) -> Card {
    let winning_numbers = string_numbers_to_numbers(raw_winning_numbers);
    let played_numbers = string_numbers_to_numbers(raw_played_numbers);

    Card {
        winning_numbers,
        played_numbers,
    }
}

fn main() {
    let input = std::fs::read_to_string("input2.txt").unwrap();
    let lines: Vec<&str> = input.split("\n").filter(|x| !x.is_empty()).collect();
    let mut cards: Vec<Card> = Vec::new();

    for line in lines {
        let number_section = line.split(":").collect::<Vec<&str>>()[1];
        let mut sections = number_section.split("|");
        let raw_winning_numbers = sections.next().unwrap_or("").trim();
        let raw_played_numbers = sections.next().unwrap_or("").trim();

        let card = create_card(raw_winning_numbers, raw_played_numbers);
        cards.push(card);
    }

    // initialize vector of 0s
    let mut instances = vec![1; cards.len()];

    for (i, card) in cards.iter().enumerate() {
        let winning_numbers = &card.winning_numbers;
        let played_numbers = &card.played_numbers;
        let mut points = 0;
        for number in winning_numbers {
            if played_numbers.contains(&number) {
                points += 1;
            }
        }

        let current_instances = instances[i];
        for _j in 0..current_instances {
            for k in 0..points {
                instances[i + k + 1] += 1;
            }
        }
    }

    println!("{:?}", instances);

    // sum of instances
    let sum: u32 = instances.iter().sum();

    println!("Sum: {}", sum);
}
