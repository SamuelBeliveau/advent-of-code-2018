use std::collections::HashMap;
use util::read_file;

pub fn solve_2_a() {
    println!("Solving question 2 a...");

    let contents = read_file("src\\question2\\input.txt");
    let lines = contents.split("\r\n");

    let mut two_identical_chars_count = 0;
    let mut three_identical_chars_count = 0;

    for line in lines {
        let mut has_two_identical_chars = false;
        let mut has_three_identical_chars = false;

        let mut char_count = HashMap::new();
        for character in line.chars() {
            char_count
                .entry(character)
                .and_modify(|c| *c += 1)
                .or_insert(1);
        }

        for (_, val) in char_count.iter() {
            if *val == 2 {
                has_two_identical_chars = true;
            } else if *val == 3 {
                has_three_identical_chars = true;
            }
        }

        if has_two_identical_chars {
            two_identical_chars_count += 1;
        }

        if has_three_identical_chars {
            three_identical_chars_count += 1;
        }
    }

    let checksum = two_identical_chars_count * three_identical_chars_count;

    println!("Answer is {}", checksum);
}

pub fn solve_2_b() {
    println!("Solving question 2 b...");

    let contents = read_file("src\\question2\\input.txt");
    let lines = contents.lines();
    let lines_vec: Vec<&str> = lines.collect();

    let mut candidate_index = 0;
    let mut candidate_line: Vec<char> = "".chars().collect();

    loop {
        for i in candidate_index..lines_vec.len() - 1 {
            let line = lines_vec[i];

            if i == candidate_index {
                candidate_line = line.chars().collect();
                println!("Setting candidate_line = {}", line);
                continue;
            }

            let mut matching_str = String::new();
            let mut mismatches = 0;

            for (j, current_char) in line.chars().enumerate() {
                let candidate_char = candidate_line[j];
                if current_char == candidate_char {
                    matching_str.push(current_char);
                } else {
                    mismatches += 1;
                }
                if mismatches >= 2 {
                    break;
                }
            }
            if mismatches == 1 {
                println!("Answer is {}", matching_str);
                return;
            }
        }
        candidate_index += 1;
        if candidate_index == lines_vec.len() {
            println!("Answer not found :(");
            return;
        }
    }
}
