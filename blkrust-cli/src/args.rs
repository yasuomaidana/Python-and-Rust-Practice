use clap::Parser;

#[derive(Parser, Debug)]
#[command(author = "Yo mero",version, about="A rust lsbk tool", long_about = None)]
pub struct Args {
    /// The device to query
    pub(crate) device: String,
}
