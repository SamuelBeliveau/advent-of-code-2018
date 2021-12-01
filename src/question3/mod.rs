use regex::Captures;
use regex::Regex;
use util::read_file;

pub fn solve_3_a() {
    println!("Solving question 3 a...");

    let contents = read_file("src\\question3\\input.txt");
    let lines: Vec<&str> = contents.lines().collect();

    let mut fabric = vec![vec![0; 1000]; 1000];
    fill_fabric(&mut fabric, &lines);

    let mut overlaps = 0;
    for y in 0..1000 {
        for x in 0..1000 {
            if fabric[y][x] > 1 {
                overlaps += 1;
            }
        }
    }

    print!("Number of overlaps = {}", overlaps);
}

pub fn solve_3_b() {
    println!("Solving question 3 b...");

    let contents = read_file("src\\question3\\input.txt");
    let lines: Vec<&str> = contents.lines().collect();

    let mut fabric = vec![vec![0; 1000]; 1000];
    fill_fabric(&mut fabric, &lines);

    let regex = Regex::new(r"^#(\d+) @ (\d+),(\d+): (\d+)x(\d+)$").unwrap();
    for i in 0..lines.len() {
        let line = lines[i];
        let captures = regex.captures(line).unwrap();
        if captures.len() != 6 {
            panic!("Unexpected line = {}", line);
        }

        let claim = Claim {
            id: extract_capture(&captures, 1),
            left: extract_capture(&captures, 2),
            top: extract_capture(&captures, 3),
            width: extract_capture(&captures, 4),
            height: extract_capture(&captures, 5),
        };

        let mut overlaps = false;
        for y in claim.top..claim.top + claim.height {
            for x in claim.left..claim.left + claim.width {
                if fabric[y][x] > 1 {
                    overlaps = true;
                }
            }
        }

        if !overlaps {
            println!("Claim id {} does not overlap!", claim.id);
        }
    }
}

fn fill_fabric(fabric: &mut Vec<Vec<i32>>, lines: &Vec<&str>) {
    let regex = Regex::new(r"^#(\d+) @ (\d+),(\d+): (\d+)x(\d+)$").unwrap();

    for i in 0..lines.len() {
        let line = lines[i];
        let captures = regex.captures(line).unwrap();
        if captures.len() != 6 {
            panic!("Unexpected line = {}", line);
        }

        let claim = Claim {
            id: extract_capture(&captures, 1),
            left: extract_capture(&captures, 2),
            top: extract_capture(&captures, 3),
            width: extract_capture(&captures, 4),
            height: extract_capture(&captures, 5),
        };

        for y in claim.top..claim.top + claim.height {
            for x in claim.left..claim.left + claim.width {
                fabric[y][x] += 1;
            }
        }
    }
}

fn extract_capture(captures: &Captures, position: usize) -> usize {
    captures
        .get(position)
        .unwrap()
        .as_str()
        .parse::<usize>()
        .unwrap()
}

#[derive(Debug)]
struct Claim {
    id: usize,
    left: usize,
    top: usize,
    width: usize,
    height: usize,
}
