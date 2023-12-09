use std::fs;

#[derive(Debug)]
struct Game {
    id: u32,
    rounds: Vec<Round>,
}

#[derive(Debug)]
struct Round {
    red: u32,
    green: u32,
    blue: u32,
}

const COLOR_RED: &str = "red";
const COLOR_GREEN: &str = "green";
const COLOR_BLUE: &str = "blue";

const MAX_RED_CUBES: u32 = 12;
const MAX_GREEN_CUBES: u32 = 13;
const MAX_BLUE_CUBES: u32 = 14;

fn amount_of_cubes_in_the_round(round: &str) -> Round {
    let colors = round.split(",").collect::<Vec<_>>();
    let mut red = 0;
    let mut green = 0;
    let mut blue = 0;

    for color in colors {
        let color = color.trim();
        let amount = color.split(" ").collect::<Vec<_>>()[0].parse().unwrap();
        let color = color.split(" ").collect::<Vec<_>>()[1];
        match color {
            COLOR_RED => red = amount,
            COLOR_GREEN => green = amount,
            COLOR_BLUE => blue = amount,
            _ => println!("Unknown color: {}", color),
        }
    }
    return Round { red, green, blue };
}

fn main() {
    let contents = fs::read_to_string("test1.txt").expect("Should have been able to read the file");

    let lines: Vec<&str> = contents.lines().collect();

    let games: Vec<Game> = lines
        .iter()
        .map(|line| {
            let parts: Vec<&str> = line.split(":").collect();
            let game_id: u32 = parts[0].split(" ").collect::<Vec<_>>()[1].parse().unwrap();

            let rounds: Vec<Round> = parts[1]
                .split(";")
                .map(|round| amount_of_cubes_in_the_round(round))
                .collect();

            return Game {
                id: game_id,
                rounds,
            };
        })
        .collect();

    // let mut id_sum = 0;
    // for game in games {
    //     let mut possible = true;
    //     for round in game.rounds {
    //         if possible == false {
    //             break;
    //         }

    //         let red = round.red;
    //         let green = round.green;
    //         let blue = round.blue;

    //         if red > MAX_RED_CUBES {
    //             possible = false;
    //         } else if green > MAX_GREEN_CUBES {
    //             possible = false;
    //         } else if blue > MAX_BLUE_CUBES {
    //             possible = false;
    //         }
    //     }
    //     if possible {
    //         id_sum += game.id;
    //     }
    // }
    println!("Sum of ids: {}", id_sum);
}
