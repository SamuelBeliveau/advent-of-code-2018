use regex::Regex;
use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::Read;

pub fn solve_7_a() {
    let re = Regex::new(r"^Step ([A-Z]) must be finished before step ([A-Z]) can begin.$").unwrap();

    let mut contents = String::new();
    File::open("src/question7/input.txt")
        .unwrap()
        .read_to_string(&mut contents)
        .expect("Failed to read file!");

    let lines = contents.lines();

    let mut steps_map: HashMap<String, RefCell<StepNode>> = HashMap::new();

    let dependencies: Vec<StepDependency> = lines
        .map(|line| {
            let captures = re.captures(line).unwrap();
            let name = captures.get(2).unwrap().as_str().to_string();
            let depends_on = captures.get(1).unwrap().as_str().to_string();

            StepDependency { name, depends_on }
        }).collect();

    for dependency in &dependencies {
        steps_map
            .entry(dependency.name.clone())
            .or_insert(RefCell::new(StepNode {
                name: (*dependency.name).to_string(),
                next_steps: HashSet::new(),
                depends_on: HashSet::new(),
            }));

        steps_map
            .entry(dependency.depends_on.clone())
            .or_insert(RefCell::new(StepNode {
                name: (*dependency.depends_on).to_string(),
                next_steps: HashSet::new(),
                depends_on: HashSet::new(),
            }));
    }

    for StepDependency { name, depends_on } in dependencies {
        let entry = steps_map.get(&name).unwrap();
        let depends_on_entry = steps_map.get(&depends_on).unwrap();

        let mut entry_cell = entry.borrow_mut();
        let mut depends_on_cell = depends_on_entry.borrow_mut();
        entry_cell.depends_on.insert(depends_on);
        depends_on_cell.next_steps.insert(name);
    }

    let mut root_step_name = String::new();

    for (_, v) in &steps_map {
        let borrowed = v.borrow();
        if borrowed.depends_on.len() == 0 {
            root_step_name = borrowed.name.clone();
            break;
        }
    }

    let mut workers = vec!(0; 5);
    let mut working_set = Vec::new();
    let mut result = String::new();


    working_set.push((root_step_name, 0));

    while working_set.len() > 0 {
        let curr = working_set[0].clone();
        result.push_str(&curr.0);

        let task_len = get_task_len(&curr.0);
        {
            let worker = workers.iter_mut().min().unwrap();
            *worker = task_len + curr.1;
            println!("Task {} should be done by {}", &curr.0, worker);
        }
        println!("Workers: {:?}", workers);

        working_set.retain(|s| s != &curr);

        let step = steps_map.get(&curr.0).unwrap().borrow();

        for next in &step.next_steps {
            let mut next_step = steps_map.get(next).unwrap().borrow_mut();
            next_step.depends_on.remove(&curr.0);
            if next_step.depends_on.len() == 0 {
                let tuple = (next.clone(), curr.1 + task_len);
                println!("{} -> {:?}", &curr.0, tuple);
                working_set.push(tuple);
            }
        }
        working_set.sort_by(|a, b| {
            let result = a.1.cmp(&b.1);
            match result {
                std::cmp::Ordering::Equal => {
                    a.0.cmp(&b.0)
                },
                _ => result
            }
        });
        println!("Sorted: {:?}", working_set);
    }

    println!("{}", result);
}

fn get_task_len(name: &String) -> u32 {
    let starting_char_idx = 'A'.to_digit(36).unwrap();
    let chars_vec: Vec<char> = name.chars().collect();
    return 60 + chars_vec[0].to_digit(36).unwrap() - starting_char_idx + 1;
}

#[derive(Debug)]
struct StepDependency {
    name: String,
    depends_on: String,
}

#[derive(Debug)]
struct StepNode {
    name: String,
    depends_on: HashSet<String>,
    next_steps: HashSet<String>,
}
