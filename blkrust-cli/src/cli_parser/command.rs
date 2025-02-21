use clap::{Parser, ValueEnum};

#[derive(Parser, Debug)]
pub struct InfoOpts {
    // #[clap(short, long, help = "The device to get info about")]
    #[clap(help = "The device to get info about")]
    pub device: String,
}

#[derive(Parser, Debug)]
pub struct TimeOpts {
    #[clap(short, long, help = "Prints debug information", env = "TIME_FMT"
    , default_value = "human")]
    pub format: TimeFormat,
}

#[derive(Debug, Clone, ValueEnum)]
pub enum TimeFormat {
    Human,
    Unix,
}