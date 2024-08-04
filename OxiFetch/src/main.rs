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

/// Draw a horizontal line for a chart
// fn draw_horizontal_bar(label: &str, value: u64, max: u64) {
//    let bar_length = 20; // Length of the bar
//    let filled_length = (value as f64 / max as f64 * bar_length as f64) as usize;
//    let empty_length = bar_length - filled_length;
//
//    let filled_bar = "=".repeat(filled_length);
//    let empty_bar = " ".repeat(empty_length);
//
//    println!("{:<12} [{}{}] {:>3}%", label, filled_bar, empty_bar, (value * 100 / max));
//}

/// Draw a box with content
fn draw_box(content: &str) {
    let width = content.lines().map(|line| line.len()).max().unwrap_or(0);
    let border = "+".to_string() + &"-".repeat(width + 2) + "+";

    println!("{}", border);
    for line in content.lines() {
        println!("| {}{} |", line, " ".repeat(width - line.len()));
    }
    println!("{}", border);
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
    let help_message = r#"
    Created By: AK1R4S4T0H
    Usage: oxifetch [OPTION]
    If run with No Options, then All Options will be used
    Options:
      -t, --os-type         Print the OS type.
      -k, --os-release      Print the OS release.
      -c, --cpu-num         Print the number of CPU cores.
      -s, --cpu-speed       Print the CPU speed in MHz.
      -m, --mem-info        Print memory information.
      -hn, --hostname       Print the hostname.
      -u, --uptime          Print system uptime.
      -l, --shell           Print the current shell.
      -h, --help            Show this help message.
    "#;

    draw_box(help_message);
}

fn display_all_info() -> Result<(), Box<dyn Error>> {
    let mut content = String::new();

    // OS Type and Kernel Version
    match os_type() {
        Ok(os) => content.push_str(&format!("OS: {}\n", os)),
        Err(e) => eprintln!("Failed to get OS type: {}", e),
    }

    match os_release() {
        Ok(release) => content.push_str(&format!("Kernel: {}\n", release)),
        Err(e) => eprintln!("Failed to get Kernel version: {}", e),
    }

    // Distro
    match get_distro() {
        Ok(distro) => content.push_str(&format!("Distro: {}\n", distro)),
        Err(e) => eprintln!("Failed to get distribution: {}", e),
    }

    // Uptime
    match get_uptime() {
        Ok(uptime) => content.push_str(&format!("Uptime: {}\n", uptime)),
        Err(e) => eprintln!("Failed to get uptime: {}", e),
    }

    // CPU Information
    match cpu_num() {
        Ok(cpus) => content.push_str(&format!("CPU Cores: {}\n", cpus)),
        Err(e) => eprintln!("Failed to get CPU cores: {}", e),
    }

    match cpu_speed() {
        Ok(speed) => content.push_str(&format!("CPU Speed: {} MHz\n", speed)),
        Err(e) => eprintln!("Failed to get CPU speed: {}", e),
    }

    // Memory Information
    match mem_info() {
        Ok(mem) => {
            content.push_str(&format!(
                "Memory: {:.2} GB / {:.2} GB\n",
                (mem.avail as f64) / 1024.0 / 1024.0,
                (mem.total as f64) / 1024.0 / 1024.0
            ));

            // Draw a memory usage chart
            // draw_horizontal_bar("Memory", mem.avail as u64, mem.total as u64);
        }
        Err(e) => eprintln!("Failed to get Memory info: {}", e),
    }

    // Hostname
    match hostname() {
        Ok(name) => content.push_str(&format!("Hostname: {}\n", name)),
        Err(e) => eprintln!("Failed to get Hostname: {}", e),
    }

    // Shell
    content.push_str(&format!("Shell: {}\n", get_shell()));

    draw_box(&content);

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
        let mut content = String::new();

        // Process each flag
        if flags.contains("--os-type") || flags.contains("-t") {
            match os_type() {
                Ok(os) => content.push_str(&format!("OS: {}\n", os)),
                Err(e) => eprintln!("Failed to get OS type: {}", e),
            }
        }

        if flags.contains("--os-release") || flags.contains("-k") {
            match os_release() {
                Ok(release) => content.push_str(&format!("Kernel: {}\n", release)),
                Err(e) => eprintln!("Failed to get Kernel version: {}", e),
            }
        }

        if flags.contains("--cpu-num") || flags.contains("-c") {
            match cpu_num() {
                Ok(cpus) => content.push_str(&format!("CPU Cores: {}\n", cpus)),
                Err(e) => eprintln!("Failed to get CPU cores: {}", e),
            }
        }

        if flags.contains("--cpu-speed") || flags.contains("-s") {
            match cpu_speed() {
                Ok(speed) => content.push_str(&format!("CPU Speed: {} MHz\n", speed)),
                Err(e) => eprintln!("Failed to get CPU speed: {}", e),
            }
        }

        if flags.contains("--mem-info") || flags.contains("-m") {
            match mem_info() {
                Ok(mem) => {
                    content.push_str(&format!(
                        "Memory: {:.2} GB / {:.2} GB\n",
                        (mem.avail as f64) / 1024.0 / 1024.0,
                        (mem.total as f64) / 1024.0 / 1024.0
                    ));

                    // memory usage bar
                    // draw_horizontal_bar("Memory", mem.avail as u64, mem.total as u64);
                }
                Err(e) => eprintln!("Failed to get Memory info: {}", e),
            }
        }

        if flags.contains("--hostname") || flags.contains("-hn") {
            match hostname() {
                Ok(name) => content.push_str(&format!("Hostname: {}\n", name)),
                Err(e) => eprintln!("Failed to get Hostname: {}", e),
            }
        }

        if flags.contains("--uptime") || flags.contains("-u") {
            match get_uptime() {
                Ok(uptime) => content.push_str(&format!("Uptime: {}\n", uptime)),
                Err(e) => eprintln!("Failed to get uptime: {}", e),
            }
        }

        if flags.contains("--shell") || flags.contains("-l") {
            content.push_str(&format!("Shell: {}\n", get_shell()));
        }

        if !content.is_empty() {
            draw_box(&content);
        }

        // no valid flags
        if flags.is_empty() {
            return display_all_info();
        }
    }

    Ok(())
}



















































//GPL 3.0 License
//AK1R4S4T0H
