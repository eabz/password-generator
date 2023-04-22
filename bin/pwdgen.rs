use clap::Parser;
use log::*;
use pwdgen::generator::random_password;
use simple_logger::SimpleLogger;
use std::process;

#[derive(Parser, Debug)]
#[command(
    name = "Password Generator",
    about = "Easy and readable password generator."
)]
pub struct GeneratorArgs {
    #[arg(
        long,
        short,
        help = "Amount of passwords to generate.",
        default_value_t = 10
    )]
    pub passwords: usize,

    #[arg(
        long,
        short,
        help = "Amount of characters for each password.",
        default_value_t = 14
    )]
    pub length: usize,
}

fn main() {
    let log: SimpleLogger = SimpleLogger::new();
    log.init().unwrap();

    let args = GeneratorArgs::parse();

    if args.length <= 3 {
        warn!("Password length must be more than 3 characters");
        process::exit(1);
    }

    for _ in 0..args.passwords {
        let generated = random_password(args.length);
        info!("{}", generated)
    }
}
