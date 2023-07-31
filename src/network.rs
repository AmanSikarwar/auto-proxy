use std::process::Command;
use std::str;

pub fn get_saved_wifi_networks() -> Result<Vec<String>, String> {
    let output = Command::new("sudo")
        .arg("nmcli")
        .args(["-t", "-f", "NAME", "connection", "show"])
        .output()
        .map_err(|e| format!("Failed to execute command: {}", e))?;

    let output_str =
        str::from_utf8(&output.stdout).map_err(|e| format!("Invalid UTF-8 data: {}", e))?;

    if !output.status.success() {
        return Err(output_str.to_string());
    }

    let networks: Vec<String> = output_str
        .lines()
        .map(|line| line.split(':').nth(0).unwrap_or("").trim().to_string())
        .filter(|ssid| !ssid.is_empty())
        .collect();

    Ok(networks)
}

pub fn get_current_wifi_network() -> Result<Option<String>, String> {
    let output = Command::new("nmcli")
        .args(&[
            "-t",
            "-f",
            "NAME,DEVICE,TYPE",
            "connection",
            "show",
            "--active",
        ])
        .output()
        .map_err(|e| format!("Failed to execute command: {}", e))?;

    if !output.status.success() {
        return Err("Failed to retrieve current WiFi network".to_string());
    }

    let output_str =
        str::from_utf8(&output.stdout).map_err(|e| format!("Invalid UTF-8 data: {}", e))?;

    let mut current_network = None;
    for line in output_str.lines() {
        let fields: Vec<&str> = line.split(':').collect();
        if fields.len() >= 3 && (fields[2] == "wifi" || fields[2] == "802-11-wireless") {
            current_network = Some(fields[0].trim().to_string());
            break;
        }
    }

    Ok(current_network)
}

pub fn print_sudoers_setup_instruction() {
    println!("To allow your program to execute nmcli with root permissions, follow these steps:");
    println!("1. Open a terminal and run the following command to edit the sudoers file:");
    println!("   sudo visudo");
    println!("2. Add the following line at the end of the sudoers file:");
    println!("   your_username ALL=(ALL) NOPASSWD: /path/to/your/rust/program");
    println!("   (Replace your_username with your actual username)");
    println!("   (Replace /path/to/your/rust/program with the actual path to your Rust program's binary)");
    println!("3. Save and exit the editor (e.g., in nano, press Ctrl + X, then Y, and finally Enter to save).");
    println!("Please make sure to replace your_username and /path/to/your/rust/program with the appropriate values.");
    println!("This will allow your program to execute the nmcli command with elevated permissions without prompting for a password.");
}
