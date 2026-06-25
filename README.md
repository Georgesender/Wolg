# Wolg 1.0

## Simple Rust WoL Activator for GNU/Linux Distros

This is a lightweight Rust utility for automatically enable Wake-on-LAN for your network interface, you just need to run it without extra args.

> **Note:** This util works with 85-90% of distros, maybe on some GNU/Linux OS with AppArmor or SELinux will not work due to "Permission denied" , but for standard desktop distros should work out of the box.

## How It Works

The program automates a simple two-step process:
1. **Interface Detection:** It scans the system's network interfaces (looking for standard prefixes like `eth*` or `enp*`) and extracts the active interface name.
2. **Execution:** The code provides two ways to enable Wake-on-LAN; if one doesn't work, try the other. First, trying manually change the value in “/sys/class/net/{interface}/device/power/wakeup” to “enabled”; if that doesn't work, try the second method using an additional CLI utility that is usually available on Linux server distributions—this is “ethtool.” If it’s not present, the utility will simply terminate with an error.

## Installation & Auto-start

Since network interface settings usually reset upon reboot, you should this util needs to be executed at every boot. In Linux, modifying network interface power states or executing utils like ethtool requires *root* privileges. In this case, you must use util with *root* privileges, you can use systemd service or openrc to automate this process every boot with *root* privileges

### 1. Build the project
you need rust installed on your system, recommending do this from official site https://rustup.rs/
```bash
cargo build --release
```

### 2. Create a systemd service for example

Create a file at /etc/systemd/system/wolg.service:

```toml 
[Unit]
Description=Enable Wake-on-LAN via Wolg
After=network.target

[Service]
Type=oneshot
ExecStart=/usr/local/bin/wolg
RemainAfterExit=yes

[Install]
WantedBy=multi-user.target
```

### 3. Move the binary and enable the service

```bash
sudo cp target/release/wolg /usr/local/bin/
sudo systemctl daemon-reload
sudo systemctl enable --now wolg.service
```

### Contributing

I am currently learning Rust! If you notice any anti-patterns, potential bugs, or ways to optimize the functions, please feel free to open an issue or submit a pull request. Feedback is highly appreciated!
