use crate::cli_parser::commands::{Command, Opts};
use clap::Parser;

mod cli_parser;
mod command_runner;
mod lsblk;

fn main() {
    let args = Opts::parse();
    println!("Debug: {}", args.debug);
    
    match args.verbose_level { 
        0 => println!("Verbose level: None"),
        1 => println!("Verbose level: Low"),
        2 => println!("Verbose level: Medium"),
        3 => println!("Verbose level: High"),
        _ => println!("Verbose level: Too much"),
    }
    
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
