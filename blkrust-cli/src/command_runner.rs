use std::process::Command;

pub fn run_command(command: &str) -> Result<String, String> {
    let args: Vec<&str> = command.split_whitespace().collect();
    Command::new(args[0])
        .args(&args[1..])
        .output()
        .map(|output| String::from_utf8_lossy(&output.stdout).to_string())
        .map_err(|e| e.to_string())
}
