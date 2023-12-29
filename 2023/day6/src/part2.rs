pub fn solve(time: i64, record: i64) -> i64 {
    let mut all_results: Vec<Vec<i64>> = Vec::new();

    let mut race_results: Vec<i64> = Vec::new();
    for i in 0..(time + 1) {
        race_results.push(race(&time, &i));
    }
    all_results.push(race_results);

    let mut ways_to_beat_record: Vec<i64> = Vec::new();
    println!("all results length: {:?}", all_results.len());
    for race_result in all_results {
        let mut can_beat_record: i64 = 0;
        for result in race_result {
            if result > record {
                can_beat_record += 1;
            }
        }
        ways_to_beat_record.push(can_beat_record);
    }
    return ways_to_beat_record
        .into_iter()
        .reduce(|a, b| a * b)
        .unwrap();
}

fn race(time: &i64, boost: &i64) -> i64 {
    let race_time: i64 = time - boost;
    return race_time * boost;
}
