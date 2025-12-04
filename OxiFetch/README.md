## Overview
OxiFetch is a 
Rust program that displays key system information, such as operating system details, uptime, CPU specs, memory usage, and more. The output includes an ASCII art logo and provides a quick overview of your machine's current status.
---
## Features

- **Operating System and Kernel**: Displays OS type and kernel version.
- **Distribution Name**: Identifies the Linux distribution from `/etc/os-release`.
- **Uptime**: Shows system uptime in hours and minutes.
- **CPU Information**: Lists the number of CPU cores, their speed in MHz, and CPU type.
- **Memory Usage**: Reports available and total memory in GB.
- **Hostname**: Displays the system's hostname.
- **Shell and Terminal Emulator**: Identifies the user's current shell and terminal emulator.
- **GPU Information**: Displays the GPU type and vendor information.
- **Desktop Manager**: Reports the desktop manager in use (e.g., GDM, LightDM).
- **Desktop Environment**: Identifies the desktop environment (e.g., GNOME, KDE Plasma).
- **ASCII Art Logo**: Presents an ASCII logo at the start of the output.

## Requirements

- **Rust**: Make sure Rust is installed. Get it from [rust-lang.org](https://www.rust-lang.org/).
---
## Installation

1. **Clone the repo**:

    ```bash
    git clone https://github.com/AK1R4S4T0H/OxiFetch
    cd oxifetch/OxiFetch
    ```

2. **Build the program**:

    ```bash
    chmod +x install.sh
    ./install.sh
    ```

3. **Run the program**:

    ```bash
    oxifetch
    ```
---
## Settings

```plaintext

╭───────────────────────────────────────────────────────────────────╮
│ Created By: AK1R4S4T0H                                            │
│ Usage: oxifetch [OPTIONS]                                         │
│                                                                   │
│ If run with no options, displays all system information.          │
│                                                                   │
│ Options:                                                          │
│ -t,  --os-type         Print the OS type                          │
│ -k,  --os-release      Print the OS release/kernel version        │
│ -c,  --cpu-num         Print the number of CPU cores              │
│ -s,  --cpu-speed       Print the CPU speed in MHz                 │
│ -m,  --mem-info        Print memory information with bar chart    │
│ -hn, --hostname        Print the hostname                         │
│ -u,  --uptime          Print system uptime                        │
│ -l,  --shell           Print the current shell                    │
│ -g,  --gpu             Print GPU information                      │
│ -term, --terminal      Print the terminal name                    │
│ -d,  --desktop         Print the desktop environment              │
│ -p,  --packages        Print package count                        │
│ -disk, --disk-usage    Print disk usage for root partition        │
│ -a,  --ascii <PATH>    Display custom ASCII art from file         │
│ -color, --theme <n>    Set color theme                            │
│ (cyan, turquoise, pink, purple, dark-pink)                        │
│ -h,  --help            Show this help message                     │
│ --check                Show version information                   │
│                                                                   │
│ Examples:                                                         │
│ oxifetch                           # Show all info (default cyan) │
│ oxifetch -c -m -g                  # Show CPU, memory, and GPU    │
│ oxifetch -a logo.txt               # Use custom ASCII art         │
│ oxifetch --theme purple            # Use purple theme             │
│ oxifetch --theme pink -a logo.txt  # Custom art with pink theme   │
╰───────────────────────────────────────────────────────────────────╯
```
---
## Example Output


```plaintext




███████╗ ██╗  ██╗██╗███████╗███████╗████████╗███████╗██╗  ██╗
██╔═══██╗╚██╗██╔╝██║██╔════╝██╔════╝╚══██╔══╝██╔════╝██║  ██║
██║   ██║ ╚███╔╝ ██║█████╗  █████╗     ██║   ██║     ███████║
██║   ██║ ██╔██╗ ██║██╔══╝  ██╔══╝     ██║   ██║     ██╔══██║
╚██████╔╝██╔╝ ██╗██║██║     ███████╗   ██║   ███████╗██║  ██║
╚═════╝ ╚═╝  ╚═╝╚═╝╚═╝     ╚══════╝   ╚═╝   ╚══════╝╚═╝  ╚═╝

╭──────────────────────────────────────────────────────╮
│ OS: Linux                                            │
│ Kernel: 6.17.9-zen1-1-zen                            │
│ Distro: Garuda Linux                                 │
│ Desktop: KDE                                         │
│ Host: Cyance                                         │
│ Uptime: 2d 15h 5m                                    │
│ Packages: 2064 (pacman)                              │
│ Shell: bash                                          │
│ Terminal: xterm-256color                             │
│                                                      │
│ CPU Cores: 12                                        │
│ CPU: AMD Ryzen 5 5600 6-Core Processor               │
│ CPU Speed: 3679 MHz                                  │
│ GPU: NVIDIA TU106 [GeForce RTX 2060 Rev. A] (rev a1) │
│                                                      │
│ Memory: 10.24 GiB / 46.97 GiB                        │
│ RAM         ━━━━━━━━━━━━━━━━━━━━━━━━━   21%          │
│ Disk (/): 373.62 GiB / 413.80 GiB                    │
│ Disk        ━━━━━━━━━━━━━━━━━━━━━━━━━   90%          │
╰──────────────────────────────────────────────────────╯
```
---
![OxiFetch Image](https://github.com/AK1R4S4T0H/OxiFetch/blob/main/pics/oxif.png?raw=true)
---