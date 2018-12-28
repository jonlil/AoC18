extern crate dec05;

use std::process;

fn main() {
    let args: &[String] = &[String::from("seed")];
    let config = dec05::Config::new(&args).unwrap_or_else(|err| {
        println!("Problem building config: {}", err);
        process::exit(1);
    });

    dec05::run(config);
}
