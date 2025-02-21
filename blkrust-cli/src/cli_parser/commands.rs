use crate::cli_parser::command::{InfoOpts, TimeOpts};
use clap::{ArgAction, Parser};
use crate::lsblk;

#[derive(Parser, Debug)]
#[command(author = "Yo mero",version, about="A rust lsbk tool", long_about = None)]
pub struct Opts {
    #[clap(short, long, help = "Prints debug information", default_value_t = false)]
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
}

impl Command {
    pub fn run(&self) {
        match self {
            Command::Info(info) => {
                let device = &info.device;
                println!("Device: {}", device);
                match lsblk::run_lsblk(device) {
                    Ok(device) => println!("{}", device),
                    Err(error) => eprintln!("{}", error),
                }
            }
            Command::Time(time) => {
                println!("Now is: {}", time.format);
            }
        }
    }
}