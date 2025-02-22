use crate::cli_parser::command::info::InfoOpts;
use crate::cli_parser::command::print_file::PrintFileOpts;
use crate::cli_parser::command::time::TimeOpts;
use crate::lsblk;
use clap::{ArgAction, Parser};
use crate::lsblk::Error;
use log::{info, warn, error};

#[derive(Parser, Debug)]
#[command(author = "Yo mero",version, about="A rust lsbk tool", long_about = None)]
pub struct Opts {
    #[clap(
        short,
        long,
        help = "Prints debug information",
        default_value_t = false
    )]
    // #[clap(short, long, help = "Prints debug information", env = "BLKRS_DEBUG", default_value = "true")]
    pub debug: bool,

    #[clap(short, long, action = ArgAction::Count, help = "Defines the level of verbosity"
    )]
    // default value without defining it is zero
    // , default_value = "1")]
    pub verbose_level: u8,

    #[clap(subcommand)]
    pub cmd: Command,
}

#[derive(Parser, Debug)]
pub(crate) enum Command {
    #[clap(name = "info", about = "Get info about a device")]
    Info(InfoOpts),
    #[clap(name = "time", about = "Get the current time")]
    Time(TimeOpts),
    #[clap(name = "print", about = "Print the content of a file")]
    PrintFile(PrintFileOpts),
}

impl Command {
    pub fn run(&self) {
        match self {
            Command::Info(info) => {
                let device = &info.device;
                println!("Device: {}", device);
                match lsblk::run_lsblk(device) {
                    Ok(device) => println!("{}", device),
                    Err(error) => { 
                        match error {
                            Error::ParseError(message) => {
                                warn!("{}", message);
                            }
                            Error::NotFound(message) => {
                                info!("{}", message);
                            }
                            Error::CommandError(message) => {
                                error!("{}", message);
                            }
                        }
                    },
                }
            }
            Command::Time(time) => {
                println!("Now is: {}", time.format);
            }
            Command::PrintFile(file) => {
                if !file.file.exists() {
                    eprintln!("File {:?} does not exist", file.file);
                    return;
                } else {
                    let content = std::fs::read_to_string(&file.file);
                    match content {
                        Ok(content) => println!("{}", content),
                        Err(error) => eprintln!("Failed to read file {:?}: {}", file.file, error),
                    }
                }
            }
        }
    }
}
