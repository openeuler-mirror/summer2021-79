use clap::{Arg, App, SubCommand, ArgMatches};

mod tools;
use tools::{process_cc, process_loc, process_locf};


fn main() {
    let matches: ArgMatches = App::new("My Super Program")
        .version("0.1")
        .author("yancong")
        .about("A tool measure Rust code")
        .subcommand(
            SubCommand::with_name("cc")
            .about("Compute Complexity of the given Rust code")
            .arg(
                Arg::with_name("input")
                .help("Sets the input file to use")
                .required(true)
                .default_value("./")
            )
        )
        .subcommand(
            SubCommand::with_name("loc")
            .about("Compute LOC (Line Of Code) per function of the given Rust code")
            .arg(
                Arg::with_name("input")
                .help("Sets the input file to use")
                .required(true)
                .default_value("./")
            )
        )
        .subcommand(
            SubCommand::with_name("locf")
            .about("Compute LOC (Line Of Code) per file of the given Rust code")
            .arg(
                Arg::with_name("input")
                .help("Sets the input file to use")
                .required(true)
                .default_value("./")
            )
        )
        .get_matches();
    
    if let Some(matches) = matches.subcommand_matches("cc") {
        let path_str = matches.value_of("input").unwrap();
        process_cc(path_str)
    }

    if let Some(matches) = matches.subcommand_matches("loc") {
        let path_str = matches.value_of("input").unwrap();
        process_loc(path_str)
    }
    
    if let Some(matches) = matches.subcommand_matches("locf") {
        let path_str = matches.value_of("input").unwrap();
        process_locf(path_str)
    }
}

