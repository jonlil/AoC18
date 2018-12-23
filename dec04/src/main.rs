extern crate dec04;
extern crate regex;

use std::process;

use dec04::Config;

fn main() {
    let args: &[String] = &[String::from("seed")];
    let config = dec04::Config::new(&args).unwrap_or_else(|err| {
        println!("Problem building config: {}", err);
        process::exit(1);
    });

    dec04::run(config);
}
