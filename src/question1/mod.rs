use std::collections::HashSet;
use util::read_file;

pub fn solve_1_a() {
    println!("Solving question 1 a...");

    let contents = read_file("src\\question1\\input.txt");
    let split = contents.split("\r\n");

    let mut accumulator = 0;

    for s in split {
        let number = s.parse::<i32>().unwrap();
        accumulator += number;
    }

    println!("Answer is {}", accumulator);
}

pub fn solve_1_b() {
    println!("Solving question 1 b...");

    let contents = read_file("src\\question1\\input.txt");
    let split = contents.split("\r\n");

    let mut frequencies = HashSet::new();
    let mut accumulator = 0;

    loop {
        for s in split.clone() {
            let number = s.parse::<i32>().unwrap();
            accumulator += number;
            if !frequencies.insert(accumulator) {
                println!("Answer is {}", accumulator);
                return;
            }
        }
    }
}
