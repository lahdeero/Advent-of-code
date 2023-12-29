struct Engine {
  max_x: i32,
  max_y: i32,
  schematic: Vec<Vec<char>>,
}

fn check_if_surrounded_by_symbol(engine: &Engine, x: i32, y: i32, number_length: i32) -> bool {
  // CHECK RIGHT
  if x+1 < engine.max_x && engine.schematic[y as usize][(x+1) as usize] != '.' {
    return true
  }
  // CHECK LEFT
  if x-number_length >= 0 && engine.schematic[y as usize][(x-number_length) as usize] != '.' {
    return true
  }


  for i in 0..(number_length + 2) {
    let current_x: i32 = x + 1 - i;
    if current_x < 0 || current_x >= engine.max_x {
      continue;
    }

    // CHECK TOP
    if y-1 > 0 && engine.schematic[(y-1) as usize][current_x as usize] != '.' {
      return true
    }
    // CHECK BOTTOM
    if y+1 < engine.max_y && engine.schematic[(y+1) as usize][current_x as usize] != '.' {
      return true
    }
  }

  return false;
}

fn add_to_sum_if_needed(engine: &Engine, x: i32, y: i32, number_string: &str) -> i32 {
  let mut adjust_x: i32 = x;
  if engine.schematic[y as usize][x as usize] == '.' {
    adjust_x = x - 1;
  }

  if check_if_surrounded_by_symbol(&engine, adjust_x, y as i32, number_string.len() as i32) {
    let number = number_string.parse::<i32>().unwrap();
    println!("number: {}", number);
    return number
  }
  return 0
}

fn main() {
  let input = std::fs::read_to_string("input1.txt").unwrap();

  let engine = Engine {
    max_x: input.lines().next().unwrap().chars().count() as i32 - 1,
    max_y: input.lines().count() as i32 - 1,
    schematic: input.lines().map(|line| line.chars().collect()).collect()
  };

  println!("max_x: {}, max_y: {}", engine.max_x, engine.max_y);
  let mut sum = 0;
  for (y, line) in engine.schematic.iter().enumerate() {
    let mut collect_number = false;
    let mut number_string = String::new();
    for (x, char) in line.iter().enumerate() {
      let is_last_in_row = (x as i32) == engine.max_x;
      if char.is_numeric() {
        collect_number = true;
        number_string.push(*char);
        if !is_last_in_row {
          continue;
        }
      }

      if collect_number {
        collect_number = false;
        sum += add_to_sum_if_needed(&engine, x as i32, y as i32, &number_string);
        number_string = String::new();
      }
    }
  }
  println!("sum: {}", sum);
}
