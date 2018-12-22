extern crate regex;

use std::fs;
use regex::Regex;
use std::str::FromStr;
use std::collections::HashMap;

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
        event = parse_shift_log(value)?
        Ok(event)
    }

    fn getType(message: &str) -> GuardEvent {
        if message.contains("#") {
            GuardEvent::StartsShift
        } else if message.contains("sleep") {
            GuardEvent::FallsAsleep
        } else {
            GuardEvent::WakesUp
        }
    }
}

enum GuardEvent {
    WakesUp,
    FallsAsleep,
    StartsShift,
}

#[derive(Hash, Eq, PartialEq, Debug, Clone)]
struct Guard {
    id: u32,
    events: Vec<String>,
}

impl Guard {
    fn new(id: u32) -> Guard {
        Guard { id: id, events: Vec::new() }
    }
}

fn parse_shift_log(log: &str) -> Result<Event, &'static str> {
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
        kind: Event::getType(&cap["message"]),
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

pub fn run(config: Config) {
    let contents = fs::read_to_string("seed.sorted")
        .expect("something went wrong reading the file");

    let mut guards: HashMap<u32, Guard> = HashMap::new();
    let mut current_id: u32 = 0;

    println!("{:?}", guards);
    // TODO: Simplify
    for event in contents.lines().into_iter().map(parse_shift_log) {
        let e = event.unwrap();

        match e.getType() {
            GuardEvent::StartsShift => {
                match parse_guard_from_message(&e.message) {
                    Ok(mut g) => {
                        current_id = g.id.to_owned();

                        match guards.get_mut(&current_id) {
                            Some(mut g) => g.events.push(e),
                            None => {
                                g.events.push(e);
                                guards.insert(g);
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
                    Some(mut g) => g.events.push(e),
                    None => continue
                }
            },
            GuardEvent::WakesUp => {
                match guards.get_mut(&current_id) {
                    Some(mut g) => g.events.push(e),
                    None => continue
                }
            }
        }
    };

    for (key, guard) in &guards {
        println!("{:?}", guard);
    }
}
