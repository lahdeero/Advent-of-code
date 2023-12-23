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
    let starting_range = (75, 37); // range between 75 and 111 (75, 76, ..., 111)
    let vector_range = (100, 100); // range between 100 and 199 (100, 101, ..., 199)

    println!("{:?}", calculate_overlap(starting_range, vector_range)); // 12 overlapping numbers (100, 101, ..., 111)

    let test_2_starting_range = (46, 1);
    let test_2_vector_range = (1, 69);

    println!(
        "{:?}",
        calculate_overlap(test_2_starting_range, test_2_vector_range) // should be 41, 1
    );
}
