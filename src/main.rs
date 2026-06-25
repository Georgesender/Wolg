use std::io::Write;
use std:: process::{Command, exit};
use std::fs::OpenOptions;

fn main() {

    let interface_name = match get_os_interface_name() {
        Ok(name) => name,
        Err(err) => {
            eprintln!("{err}");
            exit(1);
        }
    };

    println!("Trying to execute ethtool with g option for wol");
    match  enable_wol_driver(&interface_name) {
        Ok(_) => {
            println!("Wake on lan enabled successfuly!");
            exit(0)
        },
        Err(err) => {
            println!("got error({err}) but we try second way to enable wol via ethtool util");
                match enable_wol_ethtool(&interface_name)  {
            Ok(_) => {
                println!("Wake on lan enabled successfuly!");
                exit(0);
            },
            Err(err) => {
                eprintln!("Still get error after second way: {err}");
                exit(1);
            }
}
        }
    }
}
fn get_os_interface_name() -> Result<String, String> {
    println!("Trying to get interface name");
    let output = Command::new("sh")
    .arg("-c")
    .arg("ip a | grep enp")
    .output()
    .map_err(|e| format!("Failed to execute cammand to get line with interface name: {e}"))?;

    if output.status.success() {
        let string_from_out = String::from_utf8_lossy(&output.stdout);
        let mut interface = String::new();
        let parts = string_from_out.split_whitespace().map(|part| part.trim().to_string());


        for part in parts {
            if part.starts_with("enp") || part.starts_with("eth") {
                let s = part.trim_end_matches(":");
                interface = s.to_string();
            }
        };

        if interface.is_empty() {
            eprintln!("interface is empty! abort...");
            exit(1);
        } else {
            println!("Got interface name: {interface}");
            Ok(interface)
        }
    } else {
        let err = String::from_utf8_lossy(&output.stderr);
        Err(format!("ip a return error after executing: \n {err}"))
    }
}

fn enable_wol_driver(interface: &str) -> Result<(), String> {
    let path =  format!("/sys/class/net/{interface}/device/power/wakeup");
    println!("Path to wakeup file - {path}");

    println!("Opening file with write access");
    let mut file = match OpenOptions::new().write(true).open(&path) {
        Ok(f) => f,
        Err(e) => return Err(e.to_string())
    };

    println!("Write into wakeup file string in bytes -> enabled");
    match file.write_all(b"enabled\n") {
        Ok(_) => Ok(()),
        Err(e) => Err(e.to_string())
    }
}


fn enable_wol_ethtool(interface: &str) -> Result<(), String> {
    let output = Command::new("ethtool")
    .args(["-s", interface, "wol", "g"])
    .output().map_err(|e| format!("failed to run command <ethtool>: {e}"))?;

    if output.status.success() {
        println!("Successfuly set wol g for {}", interface);
        Ok(())
    } else {
        let err = String::from_utf8_lossy(&output.stderr);
        Err(format!("ethtool return error after executing:\n {err}"))
    }
}
