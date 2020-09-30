use clap::{App, Arg};
use ipa::error::IpaError;
use ipa::Ipa;
use std::path::Path;

fn main() -> Result<(), IpaError> {
    let matches = App::new("ipa")
        .arg(
            Arg::with_name("file")
                .long("file")
                .short("f")
                .required(false)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("group")
                .long("only")
                .required(false)
                .takes_value(true),
        )
        .get_matches();

    let config_file = match matches.value_of("file") {
        Some(f) => f,
        None => "ipa.yml",
    };

    let ipa = Ipa::from_file(Path::new(config_file))?;

    if let Some(group) = matches.value_of("group") {
        ipa.setup_group(group)?;
    } else {
        ipa.setup()?;
    }

    Ok(())
}
