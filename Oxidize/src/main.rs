use sys_info::{os_type, os_release, cpu_num, cpu_speed, mem_info, hostname};
use std::fs;
use std::error::Error;
use std::process::Command;
use users::{get_user_by_uid, get_current_uid};
use users::os::unix::UserExt; // Import the UserExt trait for Unix-specific extensions


/// Get the system uptime in hours and minutes
fn get_uptime() -> Result<String, Box<dyn Error>> {
    let uptime_content = fs::read_to_string("/proc/uptime")?;
    let uptime_seconds = uptime_content
        .split_whitespace()
        .next()
        .unwrap_or("0")
        .parse::<f64>()
        .unwrap_or(0.0);
    let hours = (uptime_seconds / 3600.0).floor();
    let minutes = ((uptime_seconds % 3600.0) / 60.0).floor();
    Ok(format!("{:.0} hours, {:.0} minutes", hours, minutes))
}

/// Retrieve the distribution name from /etc/os-release
fn get_distro() -> Result<String, Box<dyn Error>> {
    let os_release_content = fs::read_to_string("/etc/os-release")?;
    for line in os_release_content.lines() {
        if line.starts_with("PRETTY_NAME") {
            return Ok(line
                .split('=')
                .nth(1)
                .unwrap_or("Unknown")
                .replace("\"", ""));
        }
    }
    Ok("Unknown".to_string())
}

/// Get the current shell for the user
fn get_shell() -> String {
    if let Some(user) = get_user_by_uid(get_current_uid()) {
        user.shell().to_string_lossy().into_owned()
    } else {
        "Unknown".to_string()
    }
}

/// Determine the terminal emulator by examining the parent process
fn get_terminal() -> Result<String, Box<dyn Error>> {
    let tty_output = Command::new("ps").arg("-p").arg("$(echo $PPID)").arg("-o").arg("comm=").output()?;
    let output = String::from_utf8_lossy(&tty_output.stdout);
    let terminal = output.trim();

    if terminal.is_empty() {
        Ok("Unknown".to_string())
    } else {
        Ok(terminal.to_string())
    }
}


/// Display ASCII Art of the tool's name
fn display_ascii_logo() {
    let logo = r#"
    ________     ___    ___ ___  ________  ___  ________  _______      
    |\   __  \   |\  \  /  /|\  \|\   ___ \|\  \|\_____  \|\  ___ \     
    \ \  \|\  \  \ \  \/  / | \  \ \  \_|\ \ \  \\|___/  /\ \   __/|    
     \ \  \\\  \  \ \    / / \ \  \ \  \ \\ \ \  \   /  / /\ \  \_|/__  
      \ \  \\\  \  /     \/   \ \  \ \  \_\\ \ \  \ /  /_/__\ \  \_|\ \ 
       \ \_______\/  /\   \    \ \__\ \_______\ \__\\________\ \_______\
        \|_______/__/ /\ __\    \|__|\|_______|\|__|\|_______|\|_______|
                 |__|/ \|__|                                            
    "#;

    println!("{}", logo);
}

/// Main function to display system information
fn main() -> Result<(), Box<dyn Error>> {
    // Display ASCII logo for Oxidize
    display_ascii_logo();

    // OS Type and Kernel Version
    match os_type() {
        Ok(os) => println!("OS: {}", os),
        Err(e) => eprintln!("Failed to get OS type: {}", e),
    }

    match os_release() {
        Ok(release) => println!("Kernel: {}", release),
        Err(e) => eprintln!("Failed to get Kernel version: {}", e),
    }

    // Distro
    match get_distro() {
        Ok(distro) => println!("Distro: {}", distro),
        Err(e) => eprintln!("Failed to get distribution: {}", e),
    }

    // Uptime
    match get_uptime() {
        Ok(uptime) => println!("Uptime: {}", uptime),
        Err(e) => eprintln!("Failed to get uptime: {}", e),
    }

    // CPU Information
    match cpu_num() {
        Ok(cpus) => println!("CPU Cores: {}", cpus),
        Err(e) => eprintln!("Failed to get CPU cores: {}", e),
    }

    match cpu_speed() {
        Ok(speed) => println!("CPU Speed: {} MHz", speed),
        Err(e) => eprintln!("Failed to get CPU speed: {}", e),
    }

    // Memory Information
    match mem_info() {
        Ok(mem) => println!(
            "Memory: {:.2} GB / {:.2} GB",
            (mem.avail as f64) / 1024.0 / 1024.0,
            (mem.total as f64) / 1024.0 / 1024.0
        ),
        Err(e) => eprintln!("Failed to get Memory info: {}", e),
    }

    // Hostname
    match hostname() {
        Ok(name) => println!("Hostname: {}", name),
        Err(e) => eprintln!("Failed to get Hostname: {}", e),
    }

    // Shell
    println!("Shell: {}", get_shell());

    // Terminal Emulator
    match get_terminal() {
        Ok(terminal) => println!("Terminal: {}", terminal),
        Err(e) => eprintln!("Failed to get Terminal info: {}", e),
    }

    Ok(())
}
