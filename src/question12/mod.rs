use std::fs::File;
use std::io::Read;
use regex::Regex;
use std::collections::BTreeMap;

pub fn solve_12_a() {
    let initial_state_str = "##..##....#.#.####........##.#.#####.##..#.#..#.#...##.#####.###.##...#....##....#..###.#...#.#.#.#";
    let mut current_state = initialize_state(initial_state_str);
    let mut next_state: BTreeMap<i32, bool> = BTreeMap::new();

    let mut raw_transformations = String::new();
    File::open("src/question12/input.txt")
        .unwrap()
        .read_to_string(&mut raw_transformations)
        .expect("Failed to read file!");

    let lines = raw_transformations.lines();
    let re = Regex::new(r"^([\.\#]{5}) => ([\.\#]{1})$").unwrap();

    let transformations: Vec<_> = lines.map(|line| {
        let captures = re.captures(line).unwrap();
        let condition_str = captures.get(1).unwrap().as_str();
        let result_str = captures.get(2).unwrap().as_str().to_string();

        let condition = flags_to_booleans(condition_str);
        let result = result_str == "#".to_string();

        Transformation { condition, result }
    }).collect();

    for generation in 1..=20 {
        let current_state_keys: Vec<i32> = current_state.keys().cloned().collect();
        let min_position = current_state_keys.first().unwrap();
        let max_position = current_state_keys.last().unwrap();

        for position in (*min_position - 2)..=(*max_position + 2) {
            'trans_loop: for transformation in &transformations {
                for offset in -2..=2 {
                    let pot_status = match current_state.get(&(position + offset)) {
                        Some(x) => *x,
                        None => false,
                    };

                    if pot_status != *transformation.condition.get((offset + 2) as usize).unwrap() {
                        continue 'trans_loop;
                    }
                }

                // If we end up here, we have a match
                next_state.insert(position, transformation.result);
                break;
            }
        }
        current_state = next_state.clone();

        let result: i32 = current_state.iter()
            .map(|(key, value)| { if *value {*key} else {0} })
            .sum();
        println!("{}: {}", generation, result);
    }

    // Part B
    let test = extrapolate(50000000000);
    println!("{:?}", test);
    println!("{}", test.iter().sum::<usize>());
}

fn extrapolate(generation: usize) -> Vec<usize> {
    let mut current = generation - 98;
    let mut vec = vec![current];
    for i in 0..39 {
        current += 1;
        vec.push(current);
        current += 4;
        vec.push(current);
    }
    current += 1;
    vec.push(current);
    vec
}

fn flags_to_booleans(flags: &str) -> Vec<bool> {
    flags.chars().map(|char| char == '#').collect()
}

fn initialize_state(state_str: &str) -> BTreeMap<i32, bool> {
    let mut map = BTreeMap::new();
    for (i, char) in state_str.chars().into_iter().enumerate() {
        map.insert(i as i32, char == '#');
    }
    map
}

#[derive(Debug)]
struct Transformation {
    condition: Vec<bool>,
    result: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_flags_to_booleans() {
        assert_eq!(flags_to_booleans("#.##."), vec![true, false, true, true, false]);
    }
}