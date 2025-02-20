mod command_runner;
mod lsblk;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} DEVICE", args[0]);
        std::process::exit(1);
    }
    println!("{:?}", args);
    let device = &args[1];
    println!("Device: {}", device);
    match lsblk::run_lsblk(device) {
        Ok(device) => println!("{}", device),
        Err(error) => eprintln!("{}", error),
    }
}
