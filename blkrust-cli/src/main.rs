use clap::Parser;

mod args;
mod command_runner;
mod lsblk;

fn main() {
    let args = args::Args::parse();
    let device = &args.device;
    println!("Device: {}", device);
    match lsblk::run_lsblk(device) {
        Ok(device) => println!("{}", device),
        Err(error) => eprintln!("{}", error),
    }
}
