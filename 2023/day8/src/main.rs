use core::panic;

mod file_reader;

fn main() {
    part1();
}

// 72 not right

fn part1() {
    let (instructions, map) = file_reader::read_file_part1("input1.txt");
    let mut i: usize = 0;
    let mut steps: usize = 1;
    let first_key = "AAA";
    let last_key = "ZZZ";

    let mut current = map.get(first_key).unwrap();
    loop {
        if i >= instructions.len() {
            i = 0;
        }
        println!("current: {:?}", current);
        let direction = instructions.chars().nth(i).unwrap();
        println!("direction: {}", direction);

        if steps > 100_000_000 {
            panic!("Too many steps");
        }

        if direction == 'R' && current.1 == *last_key {
            break;
        } else if direction == 'L' && current.0 == *last_key {
            break;
        }

        if direction == 'L' {
            current = map.get(&current.0).unwrap();
        } else if direction == 'R' {
            current = map.get(&current.1).unwrap();
        } else {
            panic!("Unknown direction: {}", direction);
        }
        i += 1;
        steps += 1;
    }
    println!("Part 1: {} steps", steps);
}
