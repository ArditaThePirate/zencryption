extern crate zencryption;

use std::env;
use std::process;

use zencryption::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("Your command was '{}'", config.command);
    println!("The file is named: {}", config.filename);

    if let Err(e) = zencryption::run(config) {
        println!("Application error: {}", e);

        process::exit(1);
    }
}