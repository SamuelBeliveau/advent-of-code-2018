use std::fs::File;
use std::io::Read;
use std::cmp::Ordering;
use std::collections::{HashSet, HashMap};
use std::iter::FromIterator;
use std::borrow::{Borrow, BorrowMut};

pub fn solve_13() {
    let mut raw_map = String::new();
    File::open("src/question13/input.txt")
        .unwrap()
        .read_to_string(&mut raw_map)
        .expect("Failed to read file!");

    let (map, mut carts) = read_map(raw_map);
    println!("{:?}{:?}", map, carts);

    let mut crashed_cart_ids: HashSet<u8> = HashSet::new();

    while !tick(&map, &mut carts, &mut crashed_cart_ids) {}
}

fn tick(map: &Vec<Vec<Tile>>, carts: &mut Vec<Cart>, crashed_cart_ids: &mut HashSet<u8>) -> bool {
    carts.sort_by(|a, b| {
        match a.y.cmp(&b.y) {
            Ordering::Equal => a.x.cmp(&b.x),
            other => other
        }
    });

    let cloned_carts = carts.clone();

    let mut positions: HashMap<_, _> = cloned_carts.iter()
        .filter(|c| !crashed_cart_ids.contains(&c.id))
        .map(|c| ((c.x, c.y), c)).collect();

    println!("Remaining positions: {:?}", positions);

    if positions.len() == 1 {
        let (x, y) = positions.keys().last().unwrap();
        println!("This is the final cart position: {}, {}", x, y);
        return true;
    }

    for cart in carts {
        if crashed_cart_ids.contains(&cart.id) {
            continue;
        }

        positions.remove(&(cart.x, cart.y));

        let mut next_x = cart.x;
        let mut next_y = cart.y;

        match cart.direction {
            Direction::Right => {
                next_x = cart.x + 1;
            }
            Direction::Left => {
                next_x = cart.x - 1;
            }
            Direction::Down => {
                next_y = cart.y + 1;
            }
            Direction::Up => {
                next_y = cart.y - 1;
            }
        }

        let next_tile = map.get(next_y).unwrap().get(next_x).unwrap();

        match *next_tile {
            Tile::Horizontal => {
                cart.x = next_x;
            }
            Tile::Vertical => {
                cart.y = next_y;
            }
            Tile::Slash => {
                cart.x = next_x;
                cart.y = next_y;
                cart.direction = turn_in_slash(cart.direction);
            }
            Tile::Backslash => {
                cart.x = next_x;
                cart.y = next_y;
                cart.direction = turn_in_backslash(cart.direction);
            }
            Tile::Intersection => {
                cart.x = next_x;
                cart.y = next_y;
                cart.direction = turn_in_intersection(cart.direction, cart.visited_intersections);
                cart.visited_intersections += 1;
            }
            Tile::Empty => {}
        }

        match positions.insert((cart.x, cart.y), cart) {
            Some(other_cart) => {
                println!("Collision found at ({}, {})!", cart.x, cart.y);
                crashed_cart_ids.insert(cart.id);
                crashed_cart_ids.insert(other_cart.id);
                positions.remove(&(cart.x, cart.y));
            }
            None => {}
        }
    }

    println!("No collision found yet...");
    false
}

fn turn_left(current_direction: Direction) -> Direction {
    match current_direction {
        Direction::Right => Direction::Up,
        Direction::Up => Direction::Left,
        Direction::Left => Direction::Down,
        Direction::Down => Direction::Right,
    }
}

fn turn_right(current_direction: Direction) -> Direction {
    match current_direction {
        Direction::Right => Direction::Down,
        Direction::Down => Direction::Left,
        Direction::Left => Direction::Up,
        Direction::Up => Direction::Right,
    }
}

fn turn_in_slash(current_direction: Direction) -> Direction {
    match current_direction {
        Direction::Left | Direction::Right => turn_left(current_direction),
        Direction::Up | Direction::Down => turn_right(current_direction),
    }
}

fn turn_in_backslash(current_direction: Direction) -> Direction {
    match current_direction {
        Direction::Left | Direction::Right => turn_right(current_direction),
        Direction::Up | Direction::Down => turn_left(current_direction),
    }
}

fn turn_in_intersection(current_direction: Direction, visited_intersections: usize) -> Direction {
    match visited_intersections % 3 {
        0 => turn_left(current_direction),
        2 => turn_right(current_direction),
        _ => current_direction,
    }
}

fn read_map(raw_map: String) -> (Vec<Vec<Tile>>, Vec<Cart>) {
    let lines = raw_map.lines();
    let mut carts = Vec::new();
    let mut cart_id = 0u8;

    let map = lines.into_iter().enumerate().map(|(y, line)| {
        line.chars().into_iter().enumerate().map(|(x, char)| {
            match char {
                '-' => Tile::Horizontal,
                '<' => {
                    cart_id += 1;
                    carts.push(Cart::new(cart_id, Direction::Left, x, y));
                    Tile::Horizontal
                }
                '>' => {
                    cart_id += 1;
                    carts.push(Cart::new(cart_id, Direction::Right, x, y));
                    Tile::Horizontal
                }
                '|' => Tile::Vertical,
                'v' => {
                    cart_id += 1;
                    carts.push(Cart::new(cart_id, Direction::Down, x, y));
                    Tile::Vertical
                }
                '^' => {
                    cart_id += 1;
                    carts.push(Cart::new(cart_id, Direction::Up, x, y));
                    Tile::Vertical
                }
                '/' => Tile::Slash,
                '\\' => Tile::Backslash,
                '+' => Tile::Intersection,
                ' ' => Tile::Empty,
                other => panic!("Unknown value! {}", other)
            }
        }).collect()
    }).collect();
    (map, carts)
}

#[derive(Debug)]
#[derive(Clone)]
enum Tile {
    Empty,
    Horizontal,
    Vertical,
    Slash,
    Backslash,
    Intersection,
}

#[derive(Debug)]
#[derive(Copy)]
#[derive(Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug)]
#[derive(Copy)]
#[derive(Clone)]
struct Cart {
    id: u8,
    direction: Direction,
    x: usize,
    y: usize,
    visited_intersections: usize,
}

impl Cart {
    fn new(id: u8, direction: Direction, x: usize, y: usize) -> Cart {
        Cart {
            id,
            direction,
            x,
            y,
            visited_intersections: 0,
        }
    }
}

