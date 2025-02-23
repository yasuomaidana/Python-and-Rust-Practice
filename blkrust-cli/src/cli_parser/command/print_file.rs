use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
pub struct PrintFileOpts {
    #[clap(short, long, help = "The file to print", default_value = "Cargo.toml")]
    pub file: PathBuf,
}
