mod year_2015;

use clap::{value_parser, Arg, ArgMatches, Command};
use std::{collections::HashMap, path::Path};

fn main() {
    parse_cli();
}

fn parse_cli() {
    let matches: ArgMatches = cli().get_matches();

    match matches.subcommand() {
        Some(("run", sub_matches)) => {
            if let Some(year) = sub_matches.get_one::<u32>("year") {
                if let Some(day) = sub_matches.get_one::<u32>("day") {
                    run_year_day(*year, *day);
                } else {
                    run_year(*year);
                }
            }
        }
        _ => {}
    }
}

fn run_year(year: u32) {
    for d in 01..=25 {
        run_year_day(year, d);
    }
}

fn run_year_day(year: u32, day: u32) {
    let challanges = get_function_map();

    match challanges.get(&format!("{}-{:02}-01", year, day)) {
        Some((path, func)) => {
            println!(
                "Result of challange {}-{:02}-01: '{}'",
                year,
                day,
                func(path.to_string())
            )
        }
        None => println!("Challange {}-{:02}-01 was not found", year, day),
    }
    match challanges.get(&format!("{}-{:02}-02", year, day)) {
        Some((path, func)) => {
            println!(
                "Result of challange {}-{:02}-02: '{}'",
                year,
                day,
                func(path.to_string())
            )
        }
        None => println!("Challange {}-{:02}-02 was not found", year, day),
    }
}

fn cli() -> Command {
    Command::new("aoc").version("0.1.0").subcommand(
        Command::new("run")
            .about("run solutions")
            .arg(
                Arg::new("year")
                    .long("year")
                    .short('y')
                    .value_parser(value_parser!(u32).range(2015..=2022))
                    .help("specify the year of which you want the solutions to be run"),
            )
            .arg(
                Arg::new("day")
                    .long("day")
                    .short('d')
                    .requires("year")
                    .value_parser(value_parser!(u32).range(1..=25))
                    .help("specify the day of which you want the solutions to be run"),
            ),
    )
}

fn get_function_map() -> HashMap<String, (String, fn(String) -> String)> {
    let mut challanges: HashMap<String, (String, fn(String) -> String)> = HashMap::new();
    challanges.insert(
        "2015-01-01".to_owned(),
        (
            Path::new("./src/year_2015/day_01/input.txt")
                .to_str()
                .unwrap()
                .to_owned(),
            year_2015::day_01::part1::challange,
        ),
    );
    challanges.insert(
        "2015-01-02".to_owned(),
        (
            Path::new("./src/year_2015/day_01/input.txt")
                .to_str()
                .unwrap()
                .to_owned(),
            year_2015::day_01::part2::challange,
        ),
    );

    challanges
}
