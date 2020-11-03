use clap::{crate_authors, crate_description, crate_name, crate_version, App, Arg};
use std::path::PathBuf;

pub struct Options {
    pub config_file: PathBuf,
    pub only_group: Option<String>,
    pub except_group: Option<String>,
}

impl Default for Options {
    fn default() -> Self {
        Options {
            config_file: PathBuf::from("config.yml"),
            only_group: None,
            except_group: None,
        }
    }
}

impl Options {
    pub fn new() -> Self {
        let matches = App::new(crate_name!())
            .version(crate_version!())
            .author(crate_authors!("\n"))
            .about(crate_description!())
            .arg(
                Arg::with_name("file")
                    .long("file")
                    .short("f")
                    .required(false)
                    .takes_value(true),
            )
            .arg(
                Arg::with_name("only-group")
                    .value_name("group")
                    .long("only")
                    .required(false)
                    .takes_value(true),
            )
            .arg(
                Arg::with_name("except-group")
                    .value_name("group")
                    .long("except")
                    .required(false)
                    .takes_value(true),
            )
            .get_matches();

        let mut options = Options::default();

        if let Some(config_file) = matches.value_of("file") {
            options.config_file = PathBuf::from(config_file);
        }

        if let Some(only_group) = matches.value_of("only-group") {
            options.only_group = Some(only_group.to_owned());
        }

        if let Some(except_group) = matches.value_of("except-group") {
            options.except_group = Some(except_group.to_owned());
        }

        options
    }
}
