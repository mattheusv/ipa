use ipa::{
    cli::Options,
    config::Config,
    pacman::pacman::Pacman,
    runner::{Error, Ipa},
};
use log::info;

fn main() {
    let options = Options::new();

    if let Err(err) = init_logger(&options) {
        eprintln!("Unrecoverable error to init logger: {}", err);
        std::process::exit(1);
    }

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
    info!("Finish with successfull, see you next time.");
}

fn init_logger(options: &Options) -> Result<(), log::SetLoggerError> {
    stderrlog::new()
        .module(module_path!())
        .quiet(options.quiet)
        .verbosity(options.verbose)
        .timestamp(stderrlog::Timestamp::Second)
        .init()
}

fn run(options: Options, config: Config) -> Result<(), Error> {
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
