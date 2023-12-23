#[derive(Debug)]
struct Mapping {
    #[allow(dead_code)]
    name: String,
    values: Vec<Vec<i64>>,
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

fn main() {
    let input = std::fs::read_to_string("test1_added_seed.txt").unwrap();
    let lines: Vec<&str> = input.split("\n").filter(|x| !x.is_empty()).collect();

    let mut seeds: Vec<Seed> = lines[0].split(":").collect::<Vec<&str>>()[1]
        .trim()
        .split(" ")
        .filter(|x| !x.is_empty())
        .map(|x| Seed {
            initial: x.parse::<i64>().unwrap(),
            current: x.parse::<i64>().unwrap(),
        })
        .collect();

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
                // println!(
                //     "rv[1]={:?}, current={}, rv[2]={:?}",
                //     range_vector[1], seed.current, range_vector[2]
                // );
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

    for seed in &seeds {
        println!("{:?}", seed);
    }

    println!(
        "lowest location: {}",
        seeds.iter().min_by_key(|x| x.current).unwrap().current
    );
}
