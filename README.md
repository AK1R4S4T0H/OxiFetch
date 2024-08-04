# oxidize
---
## Overview

Rust program that displays key system information, such as operating system details, uptime, CPU specs, memory usage, and more. The output includes an ASCII art logo and provides a quick overview of your machine's current status.
---
## Features

- **Operating System and Kernel**: Displays OS type and kernel version.
- **Distribution Name**: Identifies the Linux distribution from `/etc/os-release`.
- **Uptime**: Shows system uptime in hours and minutes.
- **CPU Information**: Lists the number of CPU cores and their speed in MHz.
- **Memory Usage**: Reports available and total memory in GB.
- **Hostname**: Displays the system's hostname.
- **Shell and Terminal Emulator**: Identifies the user's current shell and terminal emulator.
- **ASCII Art Logo**: Presents an ASCII logo at the start of the output.
---
## Requirements

- **Rust**: Make sure Rust is installed. Get it from [rust-lang.org](https://www.rust-lang.org/).
---
## Installation

1. **Clone the repo**:

    ```bash
    git clone https://github.com/AK1R4S4T0H/oxidize
    cd oxidize/Oxidize
    ```

2. **Build the program**:

    ```bash
    chmod +x install.sh
    ./install.sh
    ```

3. **Run the program**:

    ```bash
    oxidize
    ```
---
## Example Output


```plaintext
    ██████╗ ██╗  ██╗██╗██████╗ ██╗███████╗███████╗
    ██╔═══██╗╚██╗██╔╝██║██╔══██╗██║╚══███╔╝██╔════╝
    ██║   ██║ ╚███╔╝ ██║██║  ██║██║  ███╔╝ █████╗  
    ██║   ██║ ██╔██╗ ██║██║  ██║██║ ███╔╝  ██╔══╝  
    ╚██████╔╝██╔╝ ██╗██║██████╔╝██║███████╗███████╗
     ╚═════╝ ╚═╝  ╚═╝╚═╝╚═════╝ ╚═╝╚══════╝╚══════╝ 
    
OS: Linux
Kernel: 5.4.0-72-generic
Distro: Ubuntu 20.04.2 LTS
Uptime: 1 hour, 25 minutes
CPU Cores: 4
CPU Speed: 2600 MHz
Memory: 3.45 GB / 8.00 GB
Hostname: my-machine
Shell: /bin/bash
Terminal: gnome-terminal
```
---