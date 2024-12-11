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
## Example Output


```plaintext




    ██████╗ ██╗  ██╗██╗███████╗███████╗████████╗ ██████╗██╗  ██╗
    ██╔═══██╗╚██╗██╔╝██║██╔════╝██╔════╝╚══██╔══╝██╔════╝██║  ██║
    ██║   ██║ ╚███╔╝ ██║█████╗  █████╗     ██║   ██║     ███████║
    ██║   ██║ ██╔██╗ ██║██╔══╝  ██╔══╝     ██║   ██║     ██╔══██║
    ╚██████╔╝██╔╝ ██╗██║██║     ███████╗   ██║   ╚██████╗██║  ██║
     ╚═════╝ ╚═╝  ╚═╝╚═╝╚═╝     ╚══════╝   ╚═╝    ╚═════╝╚═╝  ╚═╝
                                                                 
                                      
    
OS: Linux
Kernel: 5.4.0-72-generic
Distro: Ubuntu 20.04.2 LTS
Desktop Environment: KDE                                  |
Desktop Manager: plasma 
Uptime: 1 hour, 25 minutes
CPU Cores: 4
CPU Speed: 2600 MHz
CPU Type: AMD FX(tm)-6100 Six-Core Processor              |
GPU: NVIDIA Corporation GP107 [GeForce GTX 1050 Ti]
Memory: 3.45 GB / 8.00 GB
Hostname: my-machine
Shell: /bin/bash
Terminal: gnome-terminal
```
---
![OxiFetch Image](https://github.com/AK1R4S4T0H/OxiFetch/blob/main/pics/oxif.png?raw=true)
---