use std::fs::File;
use std::io::Read;

pub fn solve_8_a() {
    let mut contents = String::new();
    File::open("src/question8/input.txt")
        .unwrap()
        .read_to_string(&mut contents)
        .expect("Failed to read file!");

    let mut state = State::HeaderChildren;
    let mut curr_children: u8 = 0;
    let mut acc: u32 = 0;

    let numbers: Vec<u8> = contents.split(" ").map(|val| val.parse::<u8>().unwrap()).collect();
    let mut queue: Vec<Header> = Vec::new();
    let mut i = 0;

    while i < numbers.len() {
        let number = numbers[i];
        match state {
            State::HeaderChildren => {
                while queue.last().map_or(false, |h| h.children == 0) {
                    queue.pop();
                }

                match queue.last_mut() {
                    Some(item) => {
                        item.children -= 1;
                    }
                    None => ()
                };
                curr_children = number;
                println!("{} is children", curr_children);
                state = State::HeaderMetadata;
                i += 1;
            }
            State::HeaderMetadata => {
                queue.push(Header { children: curr_children, metadata: number });
                println!("{} is metadata count", number);
                state = State::Body;
                i += 1;
            }
            State::Body => {
                let mut pop = false;
                {
                    let mut last = queue.last_mut().unwrap();
                    if last.children == 0 {
                        acc += number as u32;
                        println!("{} is metadata", number);
                        last.metadata -= 1;

                        if last.metadata == 0 {
                            pop = true;
                        }
                        i += 1;
                    } else {
                        state = State::HeaderChildren;
                    }
                }

                if pop {
                    queue.pop();
                }
            }
        }
    }

    println!("{}", acc);
}

#[derive(Debug)]
struct Header {
    children: u8,
    metadata: u8,
}

enum State {
    HeaderChildren,
    HeaderMetadata,
    Body,
}