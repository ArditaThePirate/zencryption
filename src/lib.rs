extern crate crypto;
extern crate rand;
extern crate bcrypt;

use bcrypt::{DEFAULT_COST, hash, verify};

use std::error::Error;
use std::fs;
use std::io::prelude::*;

mod symmetriccipher; //instead of cryptography.rs which must have been deprecated as it fucked the compiler up

use symmetriccipher::{encrypt, decrypt}

pub struct Config {
    pub command: String,
    pub filename: String,
    pub secret_key: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() != 4 {
            return Err("zencryption requires 3 arguments: command, filename, and secret key (in this order)");
        }

        let command = args[1].clone();
        let filename = args[2].clone();
        let secret_key = args[3].clone();

        if command.trim() != "encrypt" && command.trim() != "decrypt" {
            return Err("The command should indicate to either 'encrypt' or 'decrypt'");
        }

        if path_dne(&filename) {
            return Err("The path does not exist. Make sure the file is at the root level of this project!");
        }

        Ok(Config {command, filename, secret_key})
    }


}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    println!("With text:\n{}", contents);

    Ok(())
}

pub fn path_dne(path: &str) -> bool { // path does not exist
    let exists = fs::metadata(path).is_ok();
    !exists
}

// fn search<'a>(command: &str, contents: &'a str) -> Vec<&'a str> {
//     vec![]
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn one_result() {
//         let command = "duct";
//         let contents = "\
//         Rust:
//         safe, fast, productive.
//         Pick three.";

//         assert_eq!(
//             vec!["safe, fast, productive"],
//             search(command, contents)
//         )
//     }
// }