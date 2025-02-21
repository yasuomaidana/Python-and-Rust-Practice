use std::fmt::Display;
use clap::{Parser, ValueEnum};

#[derive(Parser, Debug)]
pub struct TimeOpts {
    #[clap(
        short,
        long,
        help = "Prints debug information",
        env = "TIME_FMT",
        default_value = "default"
    )]
    pub format: TimeFormat,
}

#[derive(Debug, Clone, ValueEnum)]
pub enum TimeFormat {
    RFC3339,
    Default,
}

impl Display for TimeFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let time = chrono::Local::now();
        match self {
            TimeFormat::RFC3339 => {
                write!(f, "{}", time.to_rfc3339())
            }
            TimeFormat::Default => {
                write!(f, "{}", time.format("%Y-%m-%d %H:%M:%S").to_string())
            }
        }
    }
}