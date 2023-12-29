// 293 414 701 was too high
// 288 703 710 was too high
// 5 628 069

#[derive(Debug)]
struct Mapping {
    #[allow(dead_code)]
    name: String,
    values: Vec<Vec<i64>>,
    // ranges: Vec<(i64, i64)>,
}

#[derive(Debug)]
struct Seed {
    #[allow(dead_code)]
    initial: i64,
    current: i64,
}

fn string_numbers_to_numbers(string_numbers: &str) -> Vec<i64> {
    let number_strings = string_numbers.split(" ").collect::<Vec<&str>>();
    number_strings
        .iter()
        .filter(|x| !x.is_empty())
        .map(|x| x.parse::<i64>().unwrap())
        .collect::<Vec<i64>>()
}

fn calculate_overlap(range1: (i64, i64), range2: (i64, i64)) -> (i64, i64) {
    let start = std::cmp::max(range1.0, range2.0);
    let end = std::cmp::min(range1.0 + range1.1, range2.0 + range2.1);

    if start < end {
        return (start, end - start);
    } else {
        return (0, 0);
    }
}

fn main() {
    // read input from file
    let input_path = "test1.txt";
    let input = std::fs::read_to_string(input_path).unwrap();
    let lines: Vec<&str> = input.split("\n").filter(|x| !x.is_empty()).collect();

    let mut seed_pairs = lines[0].split(":").collect::<Vec<&str>>()[1]
        .trim()
        .split(" ")
        .filter(|x| !x.is_empty())
        .map(|x| x.parse::<i64>().unwrap())
        .collect::<Vec<i64>>()
        .chunks(2)
        .map(|x| (x[0], x[1]))
        .collect::<Vec<(i64, i64)>>();

    // sort seed pairs by first value
    seed_pairs.sort_by(|a, b| a.0.cmp(&b.0));

    // println!("{:?}", seed_pairs);

    let mut collect = false;
    let mut mappings: Vec<Mapping> = Vec::new();
    for line in &lines {
        if line.contains("map:") {
            collect = true;
            mappings.push(Mapping {
                name: line.split(":").collect::<Vec<&str>>()[0].trim().to_string(),
                values: Vec::new(),
                // ranges: Vec::new(),
            });
        } else if line.is_empty() {
            collect = false;
        } else if collect {
            let last_index = mappings.len() - 1;
            mappings[last_index]
                .values
                .push(string_numbers_to_numbers(line));
        }
    }

    for mapping in &mut mappings {
        mapping.values.sort_by(|a, b| a[0].cmp(&b[0]));
    }

    mappings.reverse();

    println!("{:?}", mappings[0].values[0]);

    // we want to have lowest source between 6504921 and 242892183
    // let mut starting_range = (6504921, 242892183);

    // let starting_range = (0, 100);
    let mut result: i64 = i64::MAX;
    println!("starting result: {}", result);
    for vector_range in mappings[0].values.iter() {
        let starting_range = (vector_range[1], vector_range[2]);
        recursion(starting_range, &mappings, 1, &mut result, &lines);
    }

    println!("final result: {}", result);
}

fn recursion(
    range: (i64, i64),
    mappings: &Vec<Mapping>,
    level: usize,
    result: &mut i64,
    lines: &Vec<&str>,
) {
    if level == mappings.len() {
        if range.1 > 0 {
            println!("range candidate: {:?}", range);
            let location = first_part(range.0, lines);
            if location < *result {
                *result = location;
                println!("seed{} new lowest location: {}", range.0, location);
            }
        }
        return;
    }

    let mut found = false;
    for vector in mappings[level].values.iter() {
        let vector_range = (vector[0], vector[2]);
        // println!("----------------");
        // println!("level: {}", level);
        // println!("range: {:?}", range);
        // println!("vector_range: {:?}", vector_range);
        let overlap = calculate_overlap(range, vector_range);
        // println!("overlap: {:?}", overlap);
        // println!("----------------");
        if overlap.1 > 0 {
            found = true;
            recursion(overlap, mappings, level + 1, result, lines);
        }
    }

    if !found && range.1 > 0 {
        recursion(range, mappings, level + 1, result, lines);
    }
}

// [0] destination, [1] source, [2] length

// PART 1

fn first_part(initial_seed_value: i64, lines: &Vec<&str>) -> i64 {
    let mut seeds = vec![Seed {
        initial: initial_seed_value,
        current: initial_seed_value,
    }];

    let mut collect = false;
    let mut mappings: Vec<Mapping> = Vec::new();
    for line in lines {
        if line.contains("map:") {
            collect = true;
            mappings.push(Mapping {
                name: line.split(":").collect::<Vec<&str>>()[0].trim().to_string(),
                values: Vec::new(),
            });
        } else if line.is_empty() {
            collect = false;
        } else if collect {
            let last_index = mappings.len() - 1;
            mappings[last_index]
                .values
                .push(string_numbers_to_numbers(line));
        }
    }

    for mapping in &mut mappings {
        mapping.values.sort_by(|a, b| a[1].cmp(&b[1]));
    }

    for mapping in mappings.iter() {
        for seed in &mut seeds {
            for range_vector in mapping.values.iter() {
                if range_vector[1] <= seed.current
                    && range_vector[1] + range_vector[2] > seed.current
                {
                    let a: i64 = seed.current - range_vector[1];
                    seed.current = range_vector[0] + a;
                    break;
                }
            }
        }
    }
    return seeds.iter().min_by_key(|x| x.current).unwrap().current;
}
