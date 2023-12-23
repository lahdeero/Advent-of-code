// 293 414 701 was too high
// 288 703 710 was too high

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
    let input_path = "custom_test2.txt";
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
    for line in lines {
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
        mapping.values.sort_by(|a, b| a[1].cmp(&b[1]));
    }

    let splitted = split_range(seed_pairs[0], &mappings[0]);

    let mut current_pairs = seed_pairs.clone();
    for mapping in mappings {
        let mut new_pairs = Vec::new();
        for pair in current_pairs {
            let splitted = split_range(pair, &mapping);
            // insert splitted values to new pairs
            new_pairs.extend(splitted.iter());
        }
        current_pairs = new_pairs;
    }

    println!("Result:\n{:?}", current_pairs);
}

fn split_range(range: (i64, i64), map: &Mapping) -> Vec<(i64, i64)> {
    let mut ranges: Vec<(i64, i64)> = Vec::new();
    let mut start = range.0;
    let mut range_length = range.1;

    for line in map.values.iter() {
        if line[1] > start && line[1] - start > 0 {
            ranges.push((start, line[1] - start));
            range_length -= line[1] - start;
        }
        let overlapping_range = calculate_overlap((start, range_length), (line[1], line[2]));
        if overlapping_range.0 <= 0 {
            continue;
        }
        println!("overlapping: {:?}", overlapping_range);
        ranges.push((line[0] + overlapping_range.0 - line[1], overlapping_range.1));
        start = line[1] + line[2];
        range_length -= overlapping_range.1;
        println!("{:?}", ranges);
    }
    let leftover = range.1 - ranges.iter().fold(0, |acc, x| acc + x.1);
    if (leftover > 0) {
        ranges.push((start, leftover));
    }

    return ranges;
}
// [0] destination, [1] source, [2] length

// custom test 1
// (5, 30) -> (6, 4), (10, 11), (21, 9), (30, 1)
// (5, 30) -> (6, 4), (50, 11), (10, 9), (30, 1)

// custom test 2
// (6, 4), (50, 11), (10, 9), (30, 1) -> (6, 4), (50, 2), (52, 2), (54, 7), (10, 4), (15, 6), (30, 1)
// (6, 4), (50, 11), (10, 9), (30, 1) -> (39, 4), (0, 2), (37, 2), (54, 7), (39, 5), (0, 4), (15, 1)
