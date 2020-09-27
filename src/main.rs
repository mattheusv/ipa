use clap::{App, Arg};
use ipa::Ipa;
use std::path::Path;
use std::process;

fn main() {
    let matches = App::new("ipa")
        .arg(
            Arg::with_name("file")
                .long("file")
                .short("f")
                .required(false)
                .takes_value(true),
        )
        .get_matches();

    let config_file = match matches.value_of("file") {
        Some(f) => f,
        None => "ipa.yml",
    };

    let ipa = Ipa::from_file(Path::new(config_file));

    let ipa = match ipa {
        Ok(i) => i,
        Err(e) => {
            eprintln!("Error: {:?}", e);
            process::exit(257);
        }
    };

    match ipa.process() {
        Ok(_) => {},
        Err(e) => {
            eprintln!("Error: {:?}", e);
            process::exit(257);
        }
    };

}
