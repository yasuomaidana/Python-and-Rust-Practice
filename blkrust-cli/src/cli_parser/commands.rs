use crate::cli_parser::command::InfoOpts;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author = "Yo mero",version, about="A rust lsbk tool", long_about = None)]
pub struct Opts {
    #[clap(subcommand)]
    pub cmd: Command,
}

#[derive(Parser, Debug)]
pub(crate) enum Command {
    #[clap(name = "info", about = "Get info about a device")]
    Info(InfoOpts),
}
