mod file_reader;
mod part1;
mod part2;

fn main() {
    let (times, distances) = file_reader::read_file("input2.txt".to_string());

    let time: i64 = times
        .iter()
        .map(|n| n.to_string())
        .collect::<Vec<String>>()
        .join("")
        .parse()
        .unwrap();

    let distance: i64 = distances
        .iter()
        .map(|n| n.to_string())
        .collect::<Vec<String>>()
        .join("")
        .parse()
        .unwrap();

    let part1_result = part1::solve(&times, &distances);
    let part2_result = part2::solve(time, distance);

    println!("part1 result: {:?}", part1_result);
    println!("part2 result: {:?}", part2_result);
}
