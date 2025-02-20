use clap::Parser;

#[derive(Parser, Debug)]
pub struct InfoOpts {
    // #[clap(short, long, help = "The device to get info about")]
    #[clap(help = "The device to get info about")]
    pub device: String,
}
