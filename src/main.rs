extern crate clap;

use clap::{App, Arg, ArgMatches, SubCommand};

fn main() {

    let matches = parse_args();

    if let Some(ref matches) = matches.subcommand_matches("week") {
        let day = match matches.value_of("day") {
            Some(day) => day, 
            None => {
                println!("day command requires day of week");
                return;
            }
        };

        // todo: write some stupid library, stupid ...
        println!("{} of week", day);
    }

    if let Some(ref matches) = matches.subcommand_matches("month") {
        let day = match matches.value_of("day") {
            Some(day) => day, 
            None => {
                println!("month command requires day of week and ordinal");
                return;
            }
        };

        let ordinal = match matches.value_of("ord") {
            Some(ordinal) => ordinal, 
            None => {
                println!("month command requires day of week and ordinal");
                return;
            }
        };

        // todo: do actual repeated date code here!
        println!("{} {}", day, ordinal);
    }

    if let Some(ref matches) = matches.subcommand_matches("year") {
        let day = match matches.value_of("day") {
            Some(day) => day, 
            None => {
                println!("year command requires day of year");
                return;
            }
        };

        // todo: actual repeated date code here! ... No, probably not ...
        println!("{} day of year", day);
    }   
}

fn parse_args<'a>() -> ArgMatches<'a> {
    // todo args need ranges
    App::new("reldate")
        .version("0.0.1")
        .author("Christian Sigl <subjectdenied@gmail.com>")
        .about("Prints relative dates.")
        .subcommand(SubCommand::with_name("month")
            .about("Allows creation of month-relative date streams.")
            .arg(Arg::with_name("day")
                .short("d")
                .long("day")
                .help("day of week")
                .takes_value(true))
            .arg(Arg::with_name("ord")
                .short("o")
                .long("ord")
                .help("ordinal value for repeated date.")
                .takes_value(true)))
        .subcommand(SubCommand::with_name("week")
            .about("Allows creation of week-relative date streams.")
            .arg(Arg::with_name("day")
                .short("d")
                .long("day")
                .help("day of week")
                .takes_value(true)))
        .subcommand(SubCommand::with_name("year")
            .about("Allows immortals to sort out their calendars.")
            .arg(Arg::with_name("day")
                .short("d")
                .long("day")
                .help("day of year")
                .takes_value(true)))
        .get_matches()

}