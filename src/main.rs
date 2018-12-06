use clap::{App, Arg};

#[macro_use]
mod macros;
mod day1;

pub type Result<T = ()> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result {
    let matches = App::new("Advent of Code 2018")
        .version("1.0")
        .author("Peter Hall")
        .arg(
            Arg::with_name("day")
                .short("d")
                .long("day")
                .value_name("NUM")
                .help("The day of the month: 1-25")
                .required(true)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("input")
                .short("i")
                .long("input")
                .value_name("PATH")
                .help("Location of a file with input data")
                .takes_value(true),
        )
        .get_matches();

    let day: i32 = matches
        .value_of("day")
        .and_then(|s| s.parse().ok())
        .filter(|&d| d >= 1 && d <= 25)
        .expect("day should be an integer between 1 and 25");

    match day {
        1 => {
            println!("Day 1:");
            day1::run(matches.value_of("input"))?;
        }
        _ => println!("I have not completed day {} yet!", day),
    }

    Ok(())
}
