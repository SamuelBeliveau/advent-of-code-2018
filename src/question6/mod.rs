use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::Read;

pub fn solve_6_a() {
  let mut contents = String::new();
  File::open("src/question6/input.txt")
    .unwrap()
    .read_to_string(&mut contents)
    .expect("Failed to read file!");

  let lines = contents.lines();

  let mut min_x: i32 = 0;
  let mut min_y: i32 = 0;
  let mut max_x: i32 = 0;
  let mut max_y: i32 = 0;
  let mut pos_id: u32 = 0;

  let positions: Vec<Position> = lines
    .map(|line| {
      let split: Vec<&str> = line.split(", ").collect();
      let x = split[0].parse::<i32>().unwrap();
      let y = split[1].parse::<i32>().unwrap();
      if x < min_x {
        min_x = x
      }
      if x > max_x {
        max_x = x
      }
      if y < min_y {
        min_y = y
      }
      if y > max_y {
        max_y = y
      }
      pos_id += 1;
      Position { id: pos_id, x, y }
    }).collect();

  let mut map = vec![
    vec![
      Ownership {
        id: 0,
        edge: false,
        distance: std::u32::MAX
      };
      (max_x - min_x + 1) as usize
    ];
    (max_y - min_y + 1) as usize
  ];

  let mut ids_discarded: HashSet<u32> = HashSet::new();

  for y in min_y..max_y + 1 {
    for x in min_x..max_x + 1 {
      let curr_position = Position { id: 0, x, y };
      let curr_ownership = map
        .get_mut((y - min_y) as usize)
        .unwrap()
        .get_mut((x - min_x) as usize)
        .unwrap();
      curr_ownership.edge = x == min_x || x == max_x || y == min_y || y == max_y;

      for i in 0..positions.len() {
        let distance_between = get_distance(&curr_position, &positions[i]);
        if curr_ownership.distance > distance_between {
          curr_ownership.id = positions[i].id;
          curr_ownership.distance = distance_between;
        } else if curr_ownership.distance == distance_between {
          curr_ownership.id = 0;
        }
      }

      if curr_ownership.edge {
        ids_discarded.insert(curr_ownership.id);
      }
    }
  }

  let mut totals = HashMap::new();

  for y in 0..map.len() {
    for x in 0..map[y].len() {
      let current_ownership = &map[y][x];
      if ids_discarded.contains(&current_ownership.id) || current_ownership.id == 0 {
        continue;
      }

      let entry = totals.entry(&current_ownership.id).or_insert(0);
      *entry += 1;
    }
  }

  let mut max = 0;

  for (_, v) in totals {
    if v > max {
      max = v;
    }
  }

  println!("{}", max);
}

pub fn solve_6_b() {
  let mut contents = String::new();
  File::open("src/question6/input.txt")
    .unwrap()
    .read_to_string(&mut contents)
    .expect("Failed to read file!");

  let lines = contents.lines();

  let mut min_x: i32 = 0;
  let mut min_y: i32 = 0;
  let mut max_x: i32 = 0;
  let mut max_y: i32 = 0;
  let mut pos_id: u32 = 0;

  let positions: Vec<Position> = lines
    .map(|line| {
      let split: Vec<&str> = line.split(", ").collect();
      let x = split[0].parse::<i32>().unwrap();
      let y = split[1].parse::<i32>().unwrap();
      if x < min_x {
        min_x = x
      }
      if x > max_x {
        max_x = x
      }
      if y < min_y {
        min_y = y
      }
      if y > max_y {
        max_y = y
      }
      pos_id += 1;
      Position { id: pos_id, x, y }
    }).collect();

  let center_x = ((max_x - min_x) / 2) + min_x;
  let center_y = ((max_y - min_y) / 2) + min_y;

  let mut curr_pos = Position {
    id: 0,
    x: center_x,
    y: center_y,
  };

  // let mut curr_x = center_x;
  // let mut curr_y = center_y;

  let max_distance = 10000;
  let mut curr_direction = Direction::Top;
  let mut advanced_by: u32 = 0;
  let mut is_in_range: u32 = 0;
  let mut change_direction_increment = 1;
  let mut region_size: u32 = 0;
  let mut empty_direction = HashSet::new();

  loop {
    // println!("Looking at {} {}", curr_pos.x, curr_pos.y);

    let sum_of_distances: u32 = positions
      .iter()
      .map(|position| get_distance(&position, &curr_pos))
      .sum();

    if sum_of_distances < max_distance {
      region_size += 1;
      is_in_range += 1;
    }

    advanced_by += 1;
    match curr_direction {
      Direction::Top => {
        curr_pos.y -= 1;

        if advanced_by == change_direction_increment {
          if is_in_range == 0 {
            empty_direction.insert(curr_direction);
            println!("Direction {:?} is now empty", curr_direction);
          }
          curr_direction = Direction::Right;
          advanced_by = 0;
          is_in_range = 0;
        }
      }
      Direction::Right => {
        curr_pos.x += 1;

        if advanced_by == change_direction_increment {
          if is_in_range == 0 {
            empty_direction.insert(curr_direction);
            println!("Direction {:?} is now empty", curr_direction);
          }

          change_direction_increment += 1;
          curr_direction = Direction::Bottom;
          advanced_by = 0;
          is_in_range = 0;
        }
      }
      Direction::Bottom => {
        curr_pos.y += 1;

        if advanced_by == change_direction_increment {
          if is_in_range == 0 {
            empty_direction.insert(curr_direction);
            println!("Direction {:?} is now empty", curr_direction);
          }

          curr_direction = Direction::Left;
          advanced_by = 0;
          is_in_range = 0;
        }
      }
      Direction::Left => {
        curr_pos.x -= 1;

        if advanced_by == change_direction_increment {
          if is_in_range == 0 {
            empty_direction.insert(curr_direction);
            println!("Direction {:?} is now empty", curr_direction);
          }

          change_direction_increment += 1;
          curr_direction = Direction::Top;
          advanced_by = 0;
          is_in_range = 0;
        }
      }
    }

    if empty_direction.len() == 4 {
      println!("region size = {}", region_size);
      break;
    }
  }
}

fn get_distance(pos1: &Position, pos2: &Position) -> u32 {
  (pos1.x - pos2.x).abs() as u32 + (pos1.y - pos2.y).abs() as u32
}

struct Position {
  id: u32,
  x: i32,
  y: i32,
}

#[derive(Clone)]
struct Ownership {
  id: u32,
  distance: u32,
  edge: bool,
}

#[derive(Eq, PartialEq, Hash, Debug, Copy, Clone)]
enum Direction {
  Top,
  Right,
  Bottom,
  Left,
}
