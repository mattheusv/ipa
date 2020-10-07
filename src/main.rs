use ipa::{config, error, Ipa};
mod cli;
mod pacman;

use cli::Options;
use config::Config;
use error::IpaError;
use pacman::Pacman;

fn main() {
    let options = Options::new();

    let config = match Config::load(options.config_file.as_path()) {
        Ok(config) => config,
        Err(err) => {
            eprintln!("Error: {}", err);
            std::process::exit(1);
        }
    };

    if let Err(err) = run(options, config) {
        eprintln!("Unrecoverable error: {}", err);
        std::process::exit(1);
    }
}

fn run(options: Options, config: Config) -> Result<(), IpaError> {
    let pacman = Pacman::new();
    let ipa = Ipa::new(config, &pacman);

    if let Some(only_group) = options.only_group {
        return ipa.setup_group(&only_group);
    }

    if let Some(except_group) = options.except_group {
        return ipa.setup_except_group(&except_group);
    }

    ipa.setup()
}
