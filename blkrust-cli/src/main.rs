use crate::cli_parser::commands::{Command, Opts};
use clap::Parser;

mod cli_parser;
mod command_runner;
mod lsblk;

fn main() {
    let args = Opts::parse();
    match args.cmd {
        Command::Info(info) => {
            let device = &info.device;
            println!("Device: {}", device);
            match lsblk::run_lsblk(device) {
                Ok(device) => println!("{}", device),
                Err(error) => eprintln!("{}", error),
            }
        }
    }
}
