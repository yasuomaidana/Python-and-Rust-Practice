use crate::cli_parser::commands::Opts;
use clap::Parser;
use std::fs::OpenOptions;

mod cli_parser;
mod command_runner;
mod custom_logger;
mod lsblk;

use crate::custom_logger::CustomLogger;
use env_logger::{Env, Target};

fn main() {
    let args = Opts::parse();

    let mut logger =
        env_logger::Builder::from_env(Env::default().default_filter_or(match args.verbose_level {
            0 => "error",
            1 => "warn",
            2 => "info",
            3 => "debug",
            _ => "trace",
        }));

    if args.logfile {
        println!("Writing to log file");
        let file_name = format!("blkrs-{}.log", chrono::Local::now().format("%Y-%m-%d"));
        let file = OpenOptions::new()
            .create(true)
            .append(true)
            // .write(true)
            // .truncate(true)
            .open(file_name)
            .expect("Failed to open log file");
        // logger.target(Target::Pipe(Box::new(file))); Only writes but does not print to stdout
        logger.target(Target::Pipe(Box::new(CustomLogger::new(file))));
    } else {
        logger.target(Target::Stdout);
    }
    logger.init();
    if args.debug {
        log::debug!("Debugging enabled");
    }

    args.cmd.run();
}
