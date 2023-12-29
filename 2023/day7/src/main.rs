mod file_reader;
mod part1;
mod part2;

fn main() {
    let (hands, bids) = file_reader::read_file_part1("input2.txt".to_string());

    // part1::solve(hands, bids);
    let part = 2;
    if part == 2 {
        let part2_result = part2::solve(hands.clone(), bids.clone());
        println!("Part2 result: {}", part2_result);
    } else if part == 1 {
        let part1_result = part1::solve(hands.clone(), bids.clone());
        println!("Part1 result: {}", part1_result);
    }
}
