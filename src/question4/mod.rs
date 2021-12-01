use chrono::naive::{NaiveDate, NaiveDateTime};
use chrono::Timelike;
use regex::{Captures, Regex};
use std::collections::HashMap;
use util::read_file;

pub fn solve_4_a() {
    println!("Solving question 4 a...");

    let contents = read_file("src\\question4\\input.txt");
    let lines: Vec<&str> = contents.lines().collect();

    let regexes = [
        (
            GuardEventType::BeginsShift,
            Regex::new(r"^\[(\d+)-(\d+)-(\d+) (\d+):(\d+)\] Guard #(\d+) begins shift$").unwrap(),
        ),
        (
            GuardEventType::FallsAsleep,
            Regex::new(r"^\[(\d+)-(\d+)-(\d+) (\d+):(\d+)\] falls asleep$").unwrap(),
        ),
        (
            GuardEventType::WakesUp,
            Regex::new(r"^\[(\d+)-(\d+)-(\d+) (\d+):(\d+)\] wakes up$").unwrap(),
        ),
    ];

    let mut events: Vec<GuardEvent> = lines
        .iter()
        .map(|line| parse_event(line, &regexes))
        .filter(|event_opt| event_opt.is_some())
        .map(|event_opt| event_opt.unwrap())
        .collect();

    events.sort_by(|a, b| a.timestamp.cmp(&b.timestamp));

    let mut guard_events = HashMap::new();
    let mut last_guard_id: u32 = 0;
    let mut last_sleep_start: u32 = 0;

    for event in events {
        if event.guard_id.is_some() {
            last_guard_id = event.guard_id.unwrap();
            last_sleep_start = 0;
            continue;
        }

        match event.event_type {
            GuardEventType::FallsAsleep => last_sleep_start = event.timestamp.minute(),
            GuardEventType::WakesUp => {
                let entry = guard_events.entry(last_guard_id).or_insert(vec![0; 60]);
                for i in last_sleep_start..event.timestamp.minute() {
                    entry[i as usize] += 1;
                }
            }
            _ => {}
        }
    }

    let mut max_sleep_minutes = 0;
    let mut sleeper_guard_id: u32 = 0;
    for (k, v) in &guard_events {
        let sleeping_minutes: u32 = v.iter().sum();
        if max_sleep_minutes < sleeping_minutes {
            max_sleep_minutes = sleeping_minutes;
            sleeper_guard_id = *k;
        }
    }

    let mut max_minute_value: u32 = 0;
    let mut max_minute_idx: u32 = 0;
    for i in 0..60 {
        if max_minute_value < guard_events[&sleeper_guard_id][i] {
            max_minute_value = guard_events[&sleeper_guard_id][i];
            max_minute_idx = i as u32;
        }
    }

    println!(
        "Guard Id {} at minute {} = {}",
        sleeper_guard_id,
        max_minute_idx,
        sleeper_guard_id * max_minute_idx
    );
}

pub fn solve_4_b() {
    println!("Solving question 4 b...");

    let contents = read_file("src\\question4\\input.txt");
    let lines: Vec<&str> = contents.lines().collect();

    let regexes = [
        (
            GuardEventType::BeginsShift,
            Regex::new(r"^\[(\d+)-(\d+)-(\d+) (\d+):(\d+)\] Guard #(\d+) begins shift$").unwrap(),
        ),
        (
            GuardEventType::FallsAsleep,
            Regex::new(r"^\[(\d+)-(\d+)-(\d+) (\d+):(\d+)\] falls asleep$").unwrap(),
        ),
        (
            GuardEventType::WakesUp,
            Regex::new(r"^\[(\d+)-(\d+)-(\d+) (\d+):(\d+)\] wakes up$").unwrap(),
        ),
    ];

    let mut events: Vec<GuardEvent> = lines
        .iter()
        .map(|line| parse_event(line, &regexes))
        .filter(|event_opt| event_opt.is_some())
        .map(|event_opt| event_opt.unwrap())
        .collect();

    events.sort_by(|a, b| a.timestamp.cmp(&b.timestamp));

    let mut guard_events = HashMap::new();
    let mut last_guard_id: u32 = 0;
    let mut last_sleep_start: u32 = 0;

    for event in events {
        if event.guard_id.is_some() {
            last_guard_id = event.guard_id.unwrap();
            last_sleep_start = 0;
            continue;
        }

        match event.event_type {
            GuardEventType::FallsAsleep => last_sleep_start = event.timestamp.minute(),
            GuardEventType::WakesUp => {
                let entry = guard_events.entry(last_guard_id).or_insert(vec![0; 60]);
                for i in last_sleep_start..event.timestamp.minute() {
                    entry[i as usize] += 1;
                }
            }
            _ => {}
        }
    }

    let mut stats = vec![(0, 0); 60];

    for (k, v) in &guard_events {
        for i in 0..60 {
            if stats[i].1 < v[i] {
                stats[i].0 = *k;
                stats[i].1 = v[i];
            }
        }
    }

    let mut max_minute_value: u32 = 0;
    let mut max_minute_idx: u32 = 0;
    let mut sleeper_guard_id: u32 = 0;
    for i in 0..60 {
        if max_minute_value < stats[i].1 {
            sleeper_guard_id = stats[i].0;
            max_minute_value = stats[i].1;
            max_minute_idx = i as u32;
        }
    }

    println!(
        "Guard Id {} at minute {} = {}",
        sleeper_guard_id,
        max_minute_idx,
        sleeper_guard_id * max_minute_idx
    );
}

fn parse_event(line: &str, patterns: &[(GuardEventType, Regex); 3]) -> Option<GuardEvent> {
    for pattern in patterns.iter() {
        let captures = pattern.1.captures(line);
        match captures {
            Some(value) => {
                return Some(GuardEvent {
                    event_type: pattern.0,
                    timestamp: NaiveDate::from_ymd(
                        extract_value::<i32>(&value, 1).unwrap(),
                        extract_value::<u32>(&value, 2).unwrap(),
                        extract_value::<u32>(&value, 3).unwrap(),
                    ).and_hms(
                        extract_value::<u32>(&value, 4).unwrap(),
                        extract_value::<u32>(&value, 5).unwrap(),
                        0,
                    ),
                    guard_id: if pattern.0 == GuardEventType::BeginsShift {
                        Some(value.get(6).unwrap().as_str().parse::<u32>().unwrap())
                    } else {
                        None
                    },
                });
            }
            None => {}
        };
    }
    None
}

fn extract_value<T: std::str::FromStr>(captures: &Captures, position: usize) -> Result<T, T::Err> {
    captures.get(position).unwrap().as_str().parse::<T>()
}

#[derive(Debug)]
struct GuardEvent {
    event_type: GuardEventType,
    timestamp: NaiveDateTime,
    guard_id: Option<u32>,
}

#[derive(PartialEq, Copy, Clone, Debug)]
enum GuardEventType {
    BeginsShift,
    FallsAsleep,
    WakesUp,
}

struct GuardStatistics {
    guard_id: u32,
    sleep_pattern: Vec<u32>,
}
