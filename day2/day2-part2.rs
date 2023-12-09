use std::fs;

#[derive(Debug)]
struct Game {
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
    let contents = fs::read_to_string("input2.txt").expect("Should have been able to read the file");
    let lines: Vec<&str> = contents.lines().collect();

    let games: Vec<Game> = lines
        .iter()
        .map(|line| {
            let parts: Vec<&str> = line.split(":").collect();

            let rounds: Vec<Round> = parts[1]
                .split(";")
                .map(|round| amount_of_cubes_in_the_round(round))
                .collect();

            return Game {
                rounds,
            };
        })
        .collect();

    let mut power = 0;
    for game in games {
        let mut max_red = 0;
        let mut max_green = 0;
        let mut max_blue = 0;
        for round in game.rounds {
            let red = round.red;
            let green = round.green;
            let blue = round.blue;

            if red > max_red {
                max_red = red;
            }
            if green > max_green {
                max_green = green;
            }
            if blue > max_blue {
                max_blue = blue;
            }
        }
        power += max_red * max_green * max_blue;
    }

    println!("Power: {}", power);
}
