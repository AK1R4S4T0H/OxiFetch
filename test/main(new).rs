// OxiFetch
// By: AK1R4S4T0H

use sys_info::{os_type, os_release, cpu_num, cpu_speed, mem_info, hostname};
use std::fs;
use std::error::Error;
use std::env;
use users::{get_user_by_uid, get_current_uid};
use users::os::unix::UserExt;

/// System uptime in hours and minutes
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

/// Distribution name from /etc/os-release
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

/// Current shell
fn get_shell() -> String {
    if let Some(user) = get_user_by_uid(get_current_uid()) {
        user.shell().to_string_lossy().into_owned()
    } else {
        "Unknown".to_string()
    }
}

fn display_ascii_logo() {
    let logo = r#"



    ██████╗ ██╗  ██╗██╗███████╗███████╗████████╗ ██████╗██╗  ██╗
    ██╔═══██╗╚██╗██╔╝██║██╔════╝██╔════╝╚══██╔══╝██╔════╝██║  ██║
    ██║   ██║ ╚███╔╝ ██║█████╗  █████╗     ██║   ██║     ███████║
    ██║   ██║ ██╔██╗ ██║██╔══╝  ██╔══╝     ██║   ██║     ██╔══██║
    ╚██████╔╝██╔╝ ██╗██║██║     ███████╗   ██║   ╚██████╗██║  ██║
     ╚═════╝ ╚═╝  ╚═╝╚═╝╚═╝     ╚══════╝   ╚═╝    ╚═════╝╚═╝  ╚═╝
                                                                 
    
                       
    
     "#;

    println!("{}", logo);
}

fn print_help() {
    println!("Created By: AK1R4S4T0H");
    println!("Usage: oxifetch [OPTION]");
    println!("If run with No Options, then All Options will be used");
    println!("Options:");
    println!("  -t, --os-type         Print the OS type.");
    println!("  -k, --os-release      Print the OS release.");
    println!("  -c, --cpu-num         Print the number of CPU cores.");
    println!("  -s, --cpu-speed       Print the CPU speed in MHz.");
    println!("  -m, --mem-info        Print memory information.");
    println!("  -hn, --hostname       Print the hostname.");
    println!("  -u, --uptime          Print system uptime.");
    println!("  -l, --shell           Print the current shell.");
    println!("  -h, --help            Show this help message.");
}

fn display_all_info() -> Result<(), Box<dyn Error>> {
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

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    display_ascii_logo();

    if args.len() < 2 {
        // No options provided, show all information
        return display_all_info();
    }

    // Collect flags
    let mut flags = std::collections::HashSet::new();
    for arg in &args[1..] {
        flags.insert(arg.as_str());
    }

    if flags.contains("--help") || flags.contains("-h") {
        print_help();
    } else {
        // Process each flag
        if flags.contains("--os-type") || flags.contains("-t") {
            match os_type() {
                Ok(os) => println!("OS: {}", os),
                Err(e) => eprintln!("Failed to get OS type: {}", e),
            }
        }

        if flags.contains("--os-release") || flags.contains("-k") {
            match os_release() {
                Ok(release) => println!("Kernel: {}", release),
                Err(e) => eprintln!("Failed to get Kernel version: {}", e),
            }
        }

        if flags.contains("--cpu-num") || flags.contains("-c") {
            match cpu_num() {
                Ok(cpus) => println!("CPU Cores: {}", cpus),
                Err(e) => eprintln!("Failed to get CPU cores: {}", e),
            }
        }

        if flags.contains("--cpu-speed") || flags.contains("-s") {
            match cpu_speed() {
                Ok(speed) => println!("CPU Speed: {} MHz", speed),
                Err(e) => eprintln!("Failed to get CPU speed: {}", e),
            }
        }

        if flags.contains("--mem-info") || flags.contains("-m") {
            match mem_info() {
                Ok(mem) => println!(
                    "Memory: {:.2} GB / {:.2} GB",
                    (mem.avail as f64) / 1024.0 / 1024.0,
                    (mem.total as f64) / 1024.0 / 1024.0
                ),
                Err(e) => eprintln!("Failed to get Memory info: {}", e),
            }
        }

        if flags.contains("--hostname") || flags.contains("-hn") {
            match hostname() {
                Ok(name) => println!("Hostname: {}", name),
                Err(e) => eprintln!("Failed to get Hostname: {}", e),
            }
        }

        if flags.contains("--uptime") || flags.contains("-u") {
            match get_uptime() {
                Ok(uptime) => println!("Uptime: {}", uptime),
                Err(e) => eprintln!("Failed to get uptime: {}", e),
            }
        }

        if flags.contains("--shell") || flags.contains("-l") {
            println!("Shell: {}", get_shell());
        }

        // no valid flags
        if flags.is_empty() {
            return display_all_info();
        }
    }

    Ok(())
}


