// OxiFetch
// By: AK1R4S4T0H

use std::env;
use std::error::Error;
use std::fs;
use std::process::Command;
use sys_info::{cpu_num, cpu_speed, hostname, mem_info, os_release, os_type};
use users::os::unix::UserExt;
use users::{get_current_uid, get_user_by_uid};

// ANSI Color Codes
#[derive(Clone)]
struct ColorTheme {
    primary: &'static str,
    secondary: &'static str,
    accent: &'static str,
    reset: &'static str,
}

impl ColorTheme {
    fn cyan() -> Self {
        Self {
            primary: "\x1b[38;5;51m",      // Bright cyan
            secondary: "\x1b[38;5;87m",    // Light cyan
            accent: "\x1b[38;5;123m",      // Pale cyan
            reset: "\x1b[0m",
        }
    }

    fn turquoise() -> Self {
        Self {
            primary: "\x1b[38;5;80m",      // Turquoise
            secondary: "\x1b[38;5;86m",    // Light turquoise
            accent: "\x1b[38;5;122m",      // Pale turquoise
            reset: "\x1b[0m",
        }
    }

    fn pink() -> Self {
        Self {
            primary: "\x1b[38;5;198m",     // Hot pink
            secondary: "\x1b[38;5;213m",   // Light pink
            accent: "\x1b[38;5;219m",      // Pale pink
            reset: "\x1b[0m",
        }
    }

    fn purple() -> Self {
        Self {
            primary: "\x1b[38;5;135m",     // Purple
            secondary: "\x1b[38;5;141m",   // Light purple
            accent: "\x1b[38;5;183m",      // Pale purple
            reset: "\x1b[0m",
        }
    }

    fn dark_pink() -> Self {
        Self {
            primary: "\x1b[38;5;162m",     // Dark pink
            secondary: "\x1b[38;5;168m",   // Medium pink
            accent: "\x1b[38;5;175m",      // Light pink
            reset: "\x1b[0m",
        }
    }

    fn get_theme(name: &str) -> Self {
        match name.to_lowercase().as_str() {
            "cyan" => Self::cyan(),
            "turquoise" | "teal" => Self::turquoise(),
            "pink" => Self::pink(),
            "purple" | "violet" => Self::purple(),
            "dark-pink" | "darkpink" => Self::dark_pink(),
            _ => Self::cyan(), // Default
        }
    }
}

/// System uptime in hours and minutes
fn get_uptime() -> Result<String, Box<dyn Error>> {
    let uptime_content = fs::read_to_string("/proc/uptime")?;
    let uptime_seconds = uptime_content
    .split_whitespace()
    .next()
    .ok_or("Invalid uptime format")?
    .parse::<f64>()?;

    let days = (uptime_seconds / 86400.0).floor();
    let hours = ((uptime_seconds % 86400.0) / 3600.0).floor();
    let minutes = ((uptime_seconds % 3600.0) / 60.0).floor();

    if days > 0.0 {
        Ok(format!("{}d {}h {}m", days, hours, minutes))
    } else if hours > 0.0 {
        Ok(format!("{}h {}m", hours, minutes))
    } else {
        Ok(format!("{}m", minutes))
    }
}

/// Get desktop manager type
fn get_desktop_manager() -> String {
    env::var("XDG_SESSION_DESKTOP")
    .or_else(|_| env::var("GDMSESSION"))
    .unwrap_or_else(|_| "Unknown".to_string())
}

/// Get desktop environment
fn get_desktop_environment() -> String {
    env::var("XDG_CURRENT_DESKTOP")
    .or_else(|_| env::var("DESKTOP_SESSION"))
    .or_else(|_| env::var("XDG_SESSION_TYPE"))
    .unwrap_or_else(|_| "Unknown".to_string())
}

/// Get CPU type from /proc/cpuinfo
fn get_cpu_type() -> Result<String, Box<dyn Error>> {
    let cpu_info = fs::read_to_string("/proc/cpuinfo")?;
    for line in cpu_info.lines() {
        if line.starts_with("model name") {
            if let Some(name) = line.split(':').nth(1) {
                let trimmed = name.trim();
                // Clean up CPU name
                let cleaned = trimmed
                .replace("(R)", "")
                .replace("(TM)", "")
                .replace("(tm)", "")
                .replace("  ", " ");
                return Ok(cleaned);
            }
        }
    }
    Ok("Unknown".to_string())
}

/// Distribution name from /etc/os-release
fn get_distro() -> Result<String, Box<dyn Error>> {
    let os_release_content = fs::read_to_string("/etc/os-release")?;
    for line in os_release_content.lines() {
        if line.starts_with("PRETTY_NAME") {
            if let Some(value) = line.split('=').nth(1) {
                return Ok(value.trim_matches('"').to_string());
            }
        }
    }
    Ok("Unknown".to_string())
}

/// Current shell
fn get_shell() -> String {
    get_user_by_uid(get_current_uid())
    .and_then(|user| {
        user.shell()
        .to_string_lossy()
        .split('/')
        .last()
        .map(|s| s.to_string())
    })
    .unwrap_or_else(|| "Unknown".to_string())
}

/// Get Terminal Name
fn get_terminal_name() -> String {
    env::var("TERM_PROGRAM")
    .or_else(|_| env::var("TERM"))
    .unwrap_or_else(|_| "Unknown".to_string())
}

/// Get GPU Information
fn get_gpu_info() -> Result<String, String> {
    let output = Command::new("lspci")
    .output()
    .map_err(|e| format!("Failed to execute lspci: {}", e))?;

    let output_str = String::from_utf8_lossy(&output.stdout);

    for line in output_str.lines() {
        if line.contains("VGA compatible controller") || line.contains("3D controller") {
            if let Some(gpu_info) = line.split(':').nth(2) {
                let cleaned = gpu_info
                .trim()
                .replace("Corporation", "")
                .replace("  ", " ");
                return Ok(cleaned);
            }
        }
    }

    Err("GPU information not found".to_string())
}

/// Get package count (attempts multiple package managers)
fn get_package_count() -> String {
    let managers = [
        ("dpkg", vec!["--get-selections"]),
        ("rpm", vec!["-qa"]),
        ("pacman", vec!["-Q"]),
        ("flatpak", vec!["list", "--app"]),
    ];

    for (cmd, args) in &managers {
        if let Ok(output) = Command::new(cmd).args(args).output() {
            if output.status.success() {
                let count = String::from_utf8_lossy(&output.stdout)
                .lines()
                .filter(|line| !line.trim().is_empty())
                .count();
                if count > 0 {
                    return format!("{} ({})", count, cmd);
                }
            }
        }
    }

    "Unknown".to_string()
}

/// Get disk usage for root partition
fn get_disk_usage() -> Result<(u64, u64), Box<dyn Error>> {
    let output = Command::new("df")
    .args(&["-B1", "/"])
    .output()?;

    let output_str = String::from_utf8_lossy(&output.stdout);

    for line in output_str.lines().skip(1) {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() >= 3 {
            let total = parts[1].parse::<u64>()?;
            let used = parts[2].parse::<u64>()?;
            return Ok((used, total));
        }
    }

    Err("Could not parse disk usage".into())
}

/// Draw a horizontal bar chart with color
fn draw_horizontal_bar(label: &str, used: u64, total: u64, width: usize, theme: &ColorTheme) -> String {
    let percentage = (used as f64 / total as f64 * 100.0) as usize;
    let filled = (percentage * width / 100).min(width);
    let empty = width - filled;

    format!(
        "{}{:<10}{}  {}{}{}{}{}  {:>3}%",
        theme.secondary,
        label,
        theme.reset,
        theme.primary,
        "━".repeat(filled),
            theme.accent,
            "━".repeat(empty),
            theme.reset,
            percentage
    )
}

/// Draw a box with content and colored borders
fn draw_box(content: &str, theme: &ColorTheme) {
    let width = content
    .lines()
    .map(|line| {
        // Strip ANSI codes for width calculation
        let stripped = strip_ansi_codes(line);
        stripped.chars().count()
    })
    .max()
    .unwrap_or(0);

    let border_top = format!("{}╭{}╮{}", theme.primary, "─".repeat(width + 2), theme.reset);
    let border_bottom = format!("{}╰{}╯{}", theme.primary, "─".repeat(width + 2), theme.reset);

    println!("{}", border_top);
    for line in content.lines() {
        let stripped = strip_ansi_codes(line);
        let padding = width.saturating_sub(stripped.chars().count());
        println!("{}│{} {}{} {}│{}",
                 theme.primary, theme.reset, line, " ".repeat(padding), theme.primary, theme.reset);
    }
    println!("{}", border_bottom);
}

/// Strip ANSI color codes for accurate length calculation
fn strip_ansi_codes(s: &str) -> String {
    let mut result = String::new();
    let mut chars = s.chars().peekable();

    while let Some(ch) = chars.next() {
        if ch == '\x1b' {
            if chars.peek() == Some(&'[') {
                chars.next(); // consume '['
                // Skip until we find a letter
                while let Some(c) = chars.next() {
                    if c.is_ascii_alphabetic() {
                        break;
                    }
                }
            }
        } else {
            result.push(ch);
        }
    }
    result
}

/// Display ASCII logo or custom ASCII art
fn display_ascii_logo(ascii_path: Option<&str>, theme: &ColorTheme) {
    if let Some(path) = ascii_path {
        if let Ok(content) = fs::read_to_string(path) {
            // Display custom ASCII art with color
            for line in content.lines() {
                println!("{}{}{}", theme.primary, line, theme.reset);
            }
            return;
        }
    }

    // Default logo with gradient colors
    let logo = format!(
        r#"
        {}███████╗ {}██╗  ██╗{}██╗{}███████╗{}███████╗{}████████╗{}███████╗{}██╗  ██╗{}
        {}██╔═══██╗{}╚██╗██╔╝{}██║{}██╔════╝{}██╔════╝{}╚══██╔══╝{}██╔════╝{}██║  ██║{}
        {}██║   ██║{} ╚███╔╝ {}██║{}█████╗  {}█████╗  {}   ██║   {}██║     {}███████║{}
        {}██║   ██║{} ██╔██╗ {}██║{}██╔══╝  {}██╔══╝  {}   ██║   {}██║     {}██╔══██║{}
        {}╚██████╔╝{}██╔╝ ██╗{}██║{}██║     {}███████╗{}   ██║   {}███████╗{}██║  ██║{}
        {} ╚═════╝ {}╚═╝  ╚═╝{}╚═╝{}╚═╝     {}╚══════╝{}   ╚═╝   {}╚══════╝{}╚═╝  ╚═╝{}
        "#,
        theme.primary, theme.secondary, theme.accent, theme.secondary, theme.accent, theme.secondary, theme.accent, theme.secondary, theme.reset,
        theme.primary, theme.secondary, theme.accent, theme.secondary, theme.accent, theme.secondary, theme.accent, theme.secondary, theme.reset,
        theme.primary, theme.secondary, theme.accent, theme.secondary, theme.accent, theme.secondary, theme.accent, theme.secondary, theme.reset,
        theme.primary, theme.secondary, theme.accent, theme.secondary, theme.accent, theme.secondary, theme.accent, theme.secondary, theme.reset,
        theme.primary, theme.secondary, theme.accent, theme.secondary, theme.accent, theme.secondary, theme.accent, theme.secondary, theme.reset,
        theme.primary, theme.secondary, theme.accent, theme.secondary, theme.accent, theme.secondary, theme.accent, theme.secondary, theme.reset,
    );

    println!("{}", logo);
}

fn print_help(theme: &ColorTheme) {
    let help_message = format!(
        "{}Created By: AK1R4S4T0H{}\n\
{}Usage:{} oxifetch [OPTIONS]\n\
\n\
If run with no options, displays all system information.\n\
\n\
{}Options:{}\n\
{}-t,  --os-type{}         Print the OS type\n\
{}-k,  --os-release{}      Print the OS release/kernel version\n\
{}-c,  --cpu-num{}         Print the number of CPU cores\n\
{}-s,  --cpu-speed{}       Print the CPU speed in MHz\n\
{}-m,  --mem-info{}        Print memory information with bar chart\n\
{}-hn, --hostname{}        Print the hostname\n\
{}-u,  --uptime{}          Print system uptime\n\
{}-l,  --shell{}           Print the current shell\n\
{}-g,  --gpu{}             Print GPU information\n\
{}-term, --terminal{}      Print the terminal name\n\
{}-d,  --desktop{}         Print the desktop environment\n\
{}-p,  --packages{}        Print package count\n\
{}-disk, --disk-usage{}    Print disk usage for root partition\n\
{}-a,  --ascii <PATH>{}    Display custom ASCII art from file\n\
{}-color, --theme <n>{}    Set color theme\n\
{}(cyan, turquoise, pink, purple, dark-pink){}\n\
{}-h,  --help{}            Show this help message\n\
{}--check{}                Show version information\n\
\n\
{}Examples:{}\n\
oxifetch                           {}# Show all info (default cyan){}\n\
oxifetch -c -m -g                  {}# Show CPU, memory, and GPU{}\n\
oxifetch -a logo.txt               {}# Use custom ASCII art{}\n\
oxifetch --theme purple            {}# Use purple theme{}\n\
oxifetch --theme pink -a logo.txt  {}# Custom art with pink theme{}",
theme.primary, theme.reset,
theme.secondary, theme.reset,
theme.secondary, theme.reset,
theme.accent, theme.reset,
theme.accent, theme.reset,
theme.accent, theme.reset,
theme.accent, theme.reset,
theme.accent, theme.reset,
theme.accent, theme.reset,
theme.accent, theme.reset,
theme.accent, theme.reset,
theme.accent, theme.reset,
theme.accent, theme.reset,
theme.accent, theme.reset,
theme.accent, theme.reset,
theme.accent, theme.reset,
theme.accent, theme.reset,
theme.accent, theme.reset,
theme.secondary, theme.reset,
theme.accent, theme.reset,
theme.accent, theme.reset,
theme.secondary, theme.reset,
theme.accent, theme.reset,
theme.accent, theme.reset,
theme.accent, theme.reset,
theme.accent, theme.reset,
theme.accent, theme.reset,
    );

    draw_box(&help_message, theme);
}

fn display_all_info(theme: &ColorTheme) -> Result<(), Box<dyn Error>> {
    let mut content = String::new();

    // OS Type
    if let Ok(os) = os_type() {
        content.push_str(&format!("{}OS:{} {}\n", theme.secondary, theme.reset, os));
    }

    // Kernel Version
    if let Ok(release) = os_release() {
        content.push_str(&format!("{}Kernel:{} {}\n", theme.secondary, theme.reset, release));
    }

    // Distro
    if let Ok(distro) = get_distro() {
        content.push_str(&format!("{}Distro:{} {}\n", theme.secondary, theme.reset, distro));
    }

    // Desktop Environment
    let de = get_desktop_environment();
    if de != "Unknown" {
        content.push_str(&format!("{}Desktop:{} {}\n", theme.secondary, theme.reset, de));
    }

    // Desktop Manager
    let dm = get_desktop_manager();
    if dm != "Unknown" && dm != de {
        content.push_str(&format!("{}WM:{} {}\n", theme.secondary, theme.reset, dm));
    }

    // Hostname
    if let Ok(name) = hostname() {
        content.push_str(&format!("{}Host:{} {}\n", theme.secondary, theme.reset, name));
    }

    // Uptime
    if let Ok(uptime) = get_uptime() {
        content.push_str(&format!("{}Uptime:{} {}\n", theme.secondary, theme.reset, uptime));
    }

    // Packages
    let packages = get_package_count();
    if packages != "Unknown" {
        content.push_str(&format!("{}Packages:{} {}\n", theme.secondary, theme.reset, packages));
    }

    // Shell
    content.push_str(&format!("{}Shell:{} {}\n", theme.secondary, theme.reset, get_shell()));

    // Terminal
    let term = get_terminal_name();
    if term != "Unknown" {
        content.push_str(&format!("{}Terminal:{} {}\n", theme.secondary, theme.reset, term));
    }

    content.push_str("\n");

    // CPU Information
    if let Ok(cpus) = cpu_num() {
        content.push_str(&format!("{}CPU Cores:{} {}\n", theme.secondary, theme.reset, cpus));
    }

    if let Ok(cpu) = get_cpu_type() {
        content.push_str(&format!("{}CPU:{} {}\n", theme.secondary, theme.reset, cpu));
    }

    if let Ok(speed) = cpu_speed() {
        content.push_str(&format!("{}CPU Speed:{} {} MHz\n", theme.secondary, theme.reset, speed));
    }

    // GPU
    if let Ok(gpu) = get_gpu_info() {
        content.push_str(&format!("{}GPU:{} {}\n", theme.secondary, theme.reset, gpu));
    }

    content.push_str("\n");

    // Memory Information
    if let Ok(mem) = mem_info() {
        let used = mem.total - mem.avail;
        content.push_str(&format!(
            "{}Memory:{} {:.2} GiB / {:.2} GiB\n",
            theme.secondary,
            theme.reset,
            (used as f64) / 1024.0 / 1024.0,
                                  (mem.total as f64) / 1024.0 / 1024.0
        ));

        content.push_str(&format!(
            "{}\n",
            draw_horizontal_bar("RAM", used, mem.total, 25, theme)
        ));
    }

    // Disk Usage
    if let Ok((used, total)) = get_disk_usage() {
        content.push_str(&format!(
            "{}Disk (/):{} {:.2} GiB / {:.2} GiB\n",
                                  theme.secondary,
                                  theme.reset,
                                  (used as f64) / 1024.0 / 1024.0 / 1024.0,
                                  (total as f64) / 1024.0 / 1024.0 / 1024.0
        ));

        content.push_str(&format!(
            "{}\n",
            draw_horizontal_bar("Disk", used, total, 25, theme)
        ));
    }

    draw_box(&content, theme);
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    // Parse theme flag
    let mut theme_name = "cyan";
    let mut ascii_path: Option<String> = None;
    let mut i = 1;

    while i < args.len() {
        match args[i].as_str() {
            "-color" | "--theme" if i + 1 < args.len() => {
                theme_name = &args[i + 1];
                i += 2;
            }
            "-a" | "--ascii" if i + 1 < args.len() => {
                ascii_path = Some(args[i + 1].clone());
                i += 2;
            }
            _ => i += 1,
        }
    }

    let theme = ColorTheme::get_theme(theme_name);

    display_ascii_logo(ascii_path.as_deref(), &theme);

    // Check if only theme/ascii flags were provided
    let non_special_args = args.iter()
    .skip(1)
    .filter(|arg| {
        !matches!(arg.as_str(), "-color" | "--theme" | "-a" | "--ascii")
        && !ascii_path.as_ref().map_or(false, |p| p == arg.as_str())
        && arg.as_str() != theme_name
    })
    .count();

    if non_special_args == 0 {
        return display_all_info(&theme);
    }

    // Collect flags
    let mut flags = std::collections::HashSet::new();
    for arg in &args[1..] {
        if arg != "-color" && arg != "--theme" && arg != "-a" && arg != "--ascii"
            && !ascii_path.as_ref().map_or(false, |p| p == arg)
            && arg != theme_name
            {
                flags.insert(arg.as_str());
            }
    }

    if flags.contains("--help") || flags.contains("-h") {
        print_help(&theme);
        return Ok(());
    }

    if flags.contains("--check") {
        const VERSION: &str = "0.2.0";
        println!("{}OxiFetch Version:{} {}", theme.primary, theme.reset, VERSION);
        return Ok(());
    }

    let mut content = String::new();

    // Process flags
    if flags.contains("-t") || flags.contains("--os-type") {
        if let Ok(os) = os_type() {
            content.push_str(&format!("{}OS:{} {}\n", theme.secondary, theme.reset, os));
        }
    }

    if flags.contains("-k") || flags.contains("--os-release") {
        if let Ok(release) = os_release() {
            content.push_str(&format!("{}Kernel:{} {}\n", theme.secondary, theme.reset, release));
        }
    }

    if flags.contains("-c") || flags.contains("--cpu-num") {
        if let Ok(cpus) = cpu_num() {
            content.push_str(&format!("{}CPU Cores:{} {}\n", theme.secondary, theme.reset, cpus));
        }
    }

    if flags.contains("-s") || flags.contains("--cpu-speed") {
        if let Ok(speed) = cpu_speed() {
            content.push_str(&format!("{}CPU Speed:{} {} MHz\n", theme.secondary, theme.reset, speed));
        }
    }

    if flags.contains("-g") || flags.contains("--gpu") {
        if let Ok(gpu) = get_gpu_info() {
            content.push_str(&format!("{}GPU:{} {}\n", theme.secondary, theme.reset, gpu));
        }
    }

    if flags.contains("-m") || flags.contains("--mem-info") {
        if let Ok(mem) = mem_info() {
            let used = mem.total - mem.avail;
            content.push_str(&format!(
                "{}Memory:{} {:.2} GiB / {:.2} GiB\n",
                theme.secondary,
                theme.reset,
                (used as f64) / 1024.0 / 1024.0,
                                      (mem.total as f64) / 1024.0 / 1024.0
            ));
            content.push_str(&format!(
                "{}\n",
                draw_horizontal_bar("RAM", used, mem.total, 25, &theme)
            ));
        }
    }

    if flags.contains("-disk") || flags.contains("--disk-usage") {
        if let Ok((used, total)) = get_disk_usage() {
            content.push_str(&format!(
                "{}Disk (/):{} {:.2} GiB / {:.2} GiB\n",
                                      theme.secondary,
                                      theme.reset,
                                      (used as f64) / 1024.0 / 1024.0 / 1024.0,
                                      (total as f64) / 1024.0 / 1024.0 / 1024.0
            ));
            content.push_str(&format!(
                "{}\n",
                draw_horizontal_bar("Disk", used, total, 25, &theme)
            ));
        }
    }

    if flags.contains("-hn") || flags.contains("--hostname") {
        if let Ok(name) = hostname() {
            content.push_str(&format!("{}Hostname:{} {}\n", theme.secondary, theme.reset, name));
        }
    }

    if flags.contains("-u") || flags.contains("--uptime") {
        if let Ok(uptime) = get_uptime() {
            content.push_str(&format!("{}Uptime:{} {}\n", theme.secondary, theme.reset, uptime));
        }
    }

    if flags.contains("-l") || flags.contains("--shell") {
        content.push_str(&format!("{}Shell:{} {}\n", theme.secondary, theme.reset, get_shell()));
    }

    if flags.contains("-term") || flags.contains("--terminal") {
        content.push_str(&format!("{}Terminal:{} {}\n", theme.secondary, theme.reset, get_terminal_name()));
    }

    if flags.contains("-d") || flags.contains("--desktop") {
        content.push_str(&format!("{}Desktop:{} {}\n", theme.secondary, theme.reset, get_desktop_environment()));
    }

    if flags.contains("-p") || flags.contains("--packages") {
        content.push_str(&format!("{}Packages:{} {}\n", theme.secondary, theme.reset, get_package_count()));
    }

    if !content.is_empty() {
        draw_box(&content, &theme);
    }

    Ok(())
}

//GPL 3.0 License
//AK1R4S4T0H
