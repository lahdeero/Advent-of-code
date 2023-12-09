struct Engine {
  max_x: i32,
  max_y: i32,
  schematic: Vec<Vec<char>>,
}

#[derive(Debug)]
struct Gear {
  x: i32,
  y: i32,
  numbers: Vec<i32>,
}

fn add_to_gear(gears: &mut Vec<Gear>, x: i32, y: i32, number_string: &str) {
  let number = number_string.parse::<i32>().unwrap();
  for gear in gears.iter_mut() {
    if gear.x == x && gear.y == y {
      gear.numbers.push(number);
      return;
    }
  }

  let gear = Gear {
    x: x,
    y: y,
    numbers: vec![number],
  };
  gears.push(gear)
}

fn check_if_surrounded_by_symbol(engine: &Engine, x: i32, y: i32, number_string: &str, gears: &mut Vec<Gear>) -> () {
  // CHECK RIGHT
  let number_length = number_string.chars().count() as i32;

  // debug
  if number_string == "617" {
    println!("------------");
    println!("x+1: {}, y: {}, value: {}", x, y, engine.schematic[y as usize][(x+1) as usize]);
    println!("x: {}, y: {}, value: {}", x, y, engine.schematic[y as usize][(x) as usize]);
    println!("------------");
  }

  if x+1 < engine.max_x && engine.schematic[y as usize][(x+1) as usize] != '.' {
    if engine.schematic[y as usize][(x+1) as usize] == '*' {
      add_to_gear(gears, x+1, y, &number_string);
    }
    return;
  }
  // CHECK LEFT
  if x-number_length >= 0 && engine.schematic[y as usize][(x-number_length) as usize] != '.' {
    if engine.schematic[y as usize][(x-number_length) as usize] == '*' {
      add_to_gear(gears, x-number_length, y, &number_string);
    }
    return;
  }


  for i in 0..(number_length + 2) {
    let current_x: i32 = x + 1 - i;
    if current_x < 0 || current_x >= engine.max_x {
      continue;
    }

    // CHECK TOP
    if y-1 > 0 && engine.schematic[(y-1) as usize][current_x as usize] != '.' {
      if engine.schematic[(y-1) as usize][current_x as usize] == '*' {
        add_to_gear(gears, current_x, y-1, &number_string);
      }
      return;
    }
    // CHECK BOTTOM
    if y+1 < engine.max_y && engine.schematic[(y+1) as usize][current_x as usize] != '.' {
      if engine.schematic[(y+1) as usize][current_x as usize] == '*' {
        add_to_gear(gears, current_x, y+1, &number_string);
      }
      return;
    }
  }
}

fn update_gears_if_neede(engine: &Engine, x: i32, y: i32, number_string: &str, gears: &mut Vec<Gear>) -> () {
  let mut adjust_x: i32 = x;
  let value = engine.schematic[y as usize][x as usize];
  if !value.is_numeric() {
    adjust_x = x - 1;
  }

  check_if_surrounded_by_symbol(&engine, adjust_x, y as i32, &number_string, gears);
}

fn main() {
  let input = std::fs::read_to_string("input2.txt").unwrap();

  let engine = Engine {
    max_x: input.lines().next().unwrap().chars().count() as i32 - 1,
    max_y: input.lines().count() as i32 - 1,
    schematic: input.lines().map(|line| line.chars().collect()).collect()
  };

  let mut gears: Vec<Gear> = Vec::new();

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
        update_gears_if_neede(&engine, x as i32, y as i32, &number_string, &mut gears);
        number_string = String::new();
      }
    }
  }

  println!("gears: {:?}", gears);
  for gear in gears.iter() {
    if gear.numbers.len() == 1 {
      continue;
    }

    let mut product = 1;
    for number in gear.numbers.iter() {
      product *= number;
    }
    println!("product: {}", product);
    sum += product;
  }

  println!("sum: {}", sum);
}
