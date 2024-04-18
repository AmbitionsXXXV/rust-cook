use std::path::PathBuf;

use clap::{builder::PathBufValueParser, Arg, Command};

pub fn arg_parsing() {
    let matches = Command::new("First Clap Command line tool")
        .version("0.1.0")
        .about("Teaches argument parsing")
        .arg(
            Arg::new("file")
                .short('f')
                .long("file")
                .help("A cool file")
                .value_parser(PathBufValueParser::default()),
        )
        .arg(
            Arg::new("num")
                .short('n')
                .long("number")
                .help("Five less than your favorite number"),
        )
        .get_matches();

    let default_file = PathBuf::from("input.txt");
    let my_file: &PathBuf = matches.get_one("file").unwrap_or(&default_file);

    println!("The file is: {:?}", my_file.display());

    let num_str: Option<&String> = matches.get_one("num");

    match num_str {
        Some(s) => {
            let parsed = s.parse::<i32>();

            match parsed {
                Ok(n) => println!("Your favorite number is: {}", n + 5),
                Err(_) => println!("That's not a number! {}", s),
            }
        }
        None => println!("You didn't provide a number!"),
    }
}
