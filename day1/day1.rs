use std::fs;

const NUMBER_WORDS: [&'static str; 18] = [
  "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
  "1", "2", "3", "4", "5", "6", "7", "8", "9"
];


fn first_number_to_string(string: &str, reverse: bool) -> String {
  let mut dynamic = String::new();
  let str_to_iratete = if reverse {
    string.chars().rev().collect::<String>()
  } else {
    string.to_string()
  };


  for character in str_to_iratete.chars() {
    if reverse {
      dynamic.insert(0, character);
    } else {
      dynamic.push(character);
    }


    for (index, number_word) in NUMBER_WORDS.iter().enumerate() {
      if dynamic.contains(number_word) {
        if index < 9 {
          return (index + 1).to_string();
        }
        return number_word.to_string();
      }
    }
  }
  println!("Error @ {}", string);
  return "".to_string();
}

fn string_to_number(string: &str) -> i64 {
  let number = string.parse::<i64>();
  match number {
    Ok(number) => return number,
    Err(error) => {
      println!("Error @ {}: {}", string, error);
      return 0;
    }
  }
}

fn main() {
  let contents = fs::read_to_string("input2.txt")
    .expect("Should have been able to read the file");

  let lines: Vec<&str> = contents.lines().collect();

  let numbers: Vec<i64> = lines.iter().map(|line| {
    let first_number: String = first_number_to_string(&line, false);
    let last_number: String = first_number_to_string(&line, true);
    let number_string = format!("{}{}", first_number, last_number);
    return string_to_number(&number_string);
  }).collect();

  let sum: i64 = numbers.iter().sum();
  println!("{}", sum);
}
