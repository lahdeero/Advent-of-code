pub fn read_file_part1(file: String) -> (Vec<String>, Vec<i64>) {
    let input: String = std::fs::read_to_string(file).unwrap();
    let lines: Vec<&str> = input.split("\n").collect();

    let mut hands: Vec<String> = Vec::new();
    let mut bids: Vec<i64> = Vec::new();

    for line in lines {
        let line_split: Vec<&str> = line.split(" ").collect();
        if line_split.len() < 2 {
            continue;
        }
        hands.push(line_split[0].to_string());
        bids.push(line_split[1].parse::<i64>().unwrap());
    }

    return (hands, bids);
}
