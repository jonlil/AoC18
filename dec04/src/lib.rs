extern crate regex;

use std::fs;
use regex::Regex;
use std::str::FromStr;
use std::collections::HashMap;
use std::ops::Range;

pub struct Config {
    pub filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        let filename = args[0].clone();
        Ok(Config { filename })
    }
}

#[derive(Hash, Eq, PartialEq, Debug, Clone)]
struct Event {
    year: u16,
    month: u8,
    day: u8,
    hour: u8,
    min: u8,
    message: String,
    kind: GuardEvent,
}

impl Event {
    fn new(value: &str) -> Result<Event, &'static str> {
        let event = parse_log_line(value)?;
        Ok(event)
    }

    fn get_type(message: &str) -> GuardEvent {
        if message.contains("#") {
            GuardEvent::StartsShift
        } else if message.contains("sleep") {
            GuardEvent::FallsAsleep
        } else {
            GuardEvent::WakesUp
        }
    }
}

#[derive(Hash, Eq, PartialEq, Debug, Clone)]
enum GuardEvent {
    WakesUp,
    FallsAsleep,
    StartsShift,
}

#[derive(Hash, Eq, PartialEq, Debug, Clone)]
struct Guard {
    id: u32,
    sleeping_minutes: Vec<i64>,
    total_sleep: u64,
    events: Vec<Event>,
}

impl Guard {
    fn new(id: u32) -> Guard {
        Guard {
            id: id,
            sleeping_minutes: vec![0i64; 59],
            total_sleep: 0_u64,
            events: Vec::new()
        }
    }

    fn calculate_sleep(&mut self) {
        let mut start: u8 = 70;
        let mut end: u8 = 70;

        for event in &self.events {
            match event.kind {
                GuardEvent::FallsAsleep => {
                    start = event.min.to_owned();
                },
                GuardEvent::WakesUp => {
                    end = event.min.to_owned();
                    for i in generate_sleep_range(start, end) {
                        self.sleeping_minutes[i as usize] += 1;
                    }
                    self.total_sleep += (end - start) as u64;
                },
                GuardEvent::StartsShift => {}
            }

            //println!("EventType: {:?} EventTime: {}:{}", event.kind, event.hour, event.min);
        }

        println!("GuardID: {}\n Total sleep: {}",
            self.id,
            self.total_sleep);
        for (key, value) in self.sleeping_minutes.iter().enumerate() {
            println!("{}: {}", key, value)
        }
    }
}

fn generate_sleep_range(start: u8, end: u8) -> Range<u8> {
    (start..end).into_iter()
}

fn parse_log_line(log: &str) -> Result<Event, &'static str> {
    // [1518-11-17 00:22] falls asleep
    let re = Regex::new(r"(?x)
            \[
                (?P<year>\d{4})-
                (?P<month>\d{2})-
                (?P<day>\d{2})
                \s{1}
                (?P<hour>\d{2})
                :
                (?P<min>\d{2})
            \]
            \s{1}
            (?P<message>.*)$
        ").unwrap();

    let cap = match re.captures(log) {
        Some(data) => data,
        None => panic!("Regex capture problem")
    };

    Ok( Event {
        year: FromStr::from_str(&cap["year"]).unwrap(),
        month: FromStr::from_str(&cap["month"]).unwrap(),
        day: FromStr::from_str(&cap["day"]).unwrap(),
        min: FromStr::from_str(&cap["min"]).unwrap(),
        hour: FromStr::from_str(&cap["hour"]).unwrap(),
        message: FromStr::from_str(&cap["message"]).unwrap(),
        kind: Event::get_type(&cap["message"]),
    })
}

fn parse_guard_from_message(message: &str) -> Result<Guard, &'static str> {
    let re = Regex::new(r"(?x)
        Guard
        \s{1}
        \#(\d*)
        \s{1}
        .*
    ").unwrap();

    let cap = match re.captures(message) {
        Some(data) => data,
        None => panic!("Failed extracting GuardID")
    };

    Ok( Guard::new(FromStr::from_str(&cap[1]).unwrap()) )
}

pub fn run(_config: Config) {
    let contents = fs::read_to_string("seed.sorted")
        .expect("something went wrong reading the file");

    let mut guards: HashMap<u32, Guard> = HashMap::new();
    let mut current_id: u32 = 0;

    // TODO: Simplify
    for event in contents.lines().into_iter().map(Event::new) {
        let e = event.unwrap();

        match e.kind {
            GuardEvent::StartsShift => {
                match parse_guard_from_message(&e.message) {
                    Ok(mut g) => {
                        current_id = g.id.to_owned();

                        match guards.get_mut(&current_id) {
                            Some(g) => g.events.push(e),
                            None => {
                                g.events.push(e);
                                guards.insert(g.id, g);
                            }
                        }
                    },
                    Err(_) => {
                        continue
                    }
                }
            },
            GuardEvent::FallsAsleep => {
                match guards.get_mut(&current_id) {
                    Some(g) => g.events.push(e),
                    None => continue
                }
            },
            GuardEvent::WakesUp => {
                match guards.get_mut(&current_id) {
                    Some(g) => g.events.push(e),
                    None => continue
                }
            }
        }
    };

    guards.get_mut(&2953_u32).unwrap().calculate_sleep();
    // for guard in guards.values_mut() {
    //     &guard.calculate_sleep();
    // }
}

fn sleep_time() {
    // 50 - 30 = 20
    // 9 - 0 = 9
    //end - start
}
