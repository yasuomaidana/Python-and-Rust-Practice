use crate::command_runner::run_command;
use serde_json::Value;

pub fn run_lsblk(device: &str) -> Result<Value, String> {
    let command = "lsblk -J -o NAME,SIZE,TYPE,MOUNTPOINT";
    match run_command(command) {
        Ok(output) => match serde_json::from_str::<Value>(&output) {
            Ok(devices) => match devices["blockdevices"].as_array() {
                Some(blockdevices) => {
                    for parent in blockdevices {
                        if parent["name"].as_str().unwrap() == device {
                            return Ok(parent.clone());
                        }
                        if let Some(children) = parent["children"].as_array() {
                            for child in children {
                                if child["name"].as_str().unwrap() == device {
                                    return Ok(child.clone());
                                }
                            }
                        }
                    }
                    Err(format!("Device {} not found in lsblk output", device))
                }
                None => Err("Failed to parse blockdevices array".to_string()),
            },
            Err(error) => Err(format!("Failed to parse lsblk output: {}", error)),
        },
        Err(error) => Err(format!("Failed to run lsblk command 😭 {error}")),
    }
}
