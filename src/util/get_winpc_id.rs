use std::process::Command;

pub fn get_system_uuid() -> String {
    let output = Command::new("wmic")
        .args(&["csproduct", "get", "UUID"])
        .output()
        .expect("failed to execute process");

    let uuid = String::from_utf8_lossy(&output.stdout)
        .lines()
        .nth(1)
        .expect("failed to get UUID")
        .trim()
        .to_string();

    uuid
}
