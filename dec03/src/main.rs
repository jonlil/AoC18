extern crate dec03;

use std::process;

use dec03::Config;

fn main() {
    let args: &[String] = &[String::from("seed")];
    let config = dec03::Config::new(&args).unwrap_or_else(|err| {
        println!("Problem building config: {}", err);
        process::exit(1);
    });

    dec03::run(config);
}
