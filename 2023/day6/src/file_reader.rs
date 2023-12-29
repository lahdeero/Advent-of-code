pub fn read_file(file: String) -> (Vec<i64>, Vec<i64>) {
    let input: String = std::fs::read_to_string(file).unwrap();
    let lines: Vec<&str> = input.split("\n").collect();
    let times: Vec<i64> = lines[0].split(":").collect::<Vec<&str>>()[1]
        .split_whitespace()
        .filter_map(|s| s.parse::<i64>().ok())
        .collect();
    let distances: Vec<i64> = lines[1].split(":").collect::<Vec<&str>>()[1]
        .split_whitespace()
        .filter_map(|s| s.parse::<i64>().ok())
        .collect();

    return (times, distances);
}
