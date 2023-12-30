use std::collections::HashMap;

pub fn read_file_part1(file: &str) -> (String, HashMap<String, (String, String)>) {
    let input: String = std::fs::read_to_string(file).unwrap();
    let lines: Vec<&str> = input.split("\n").collect();

    let mut nodes: HashMap<String, (String, String)> = HashMap::new();
    for (i, line) in lines.iter().enumerate() {
        if i < 2 || line.len() == 0 {
            continue;
        }
        let first_split: Vec<&str> = line.split("=").collect();
        let key = first_split[0].trim().to_string();
        let second_split: Vec<&str> = first_split[1].split(",").collect();
        let left = second_split[0].trim().replace("(", "").to_string();
        let right = second_split[1].trim().replace(")", "").to_string();
        nodes.insert(key.to_string(), (left, right));
    }

    return (lines[0].to_string(), nodes);
}
