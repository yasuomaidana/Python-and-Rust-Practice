use crate::cli_parser::commands::Opts;
use clap::Parser;

mod cli_parser;
mod command_runner;
mod lsblk;
use env_logger::Env;

fn main() {
    let args = Opts::parse();

    env_logger::Builder::from_env(Env::default().default_filter_or(match args.verbose_level {
        0 => "error",
        1 => "warn",
        2 => "info",
        3 => "debug",
        _ => "trace",
    }))
    .init();
    if args.debug{
        log::debug!("Debugging enabled");
    }

    args.cmd.run();
}
