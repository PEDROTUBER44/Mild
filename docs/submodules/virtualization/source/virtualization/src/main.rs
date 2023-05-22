use std::{
    process::{
        Command,
        exit
    },

    io::{
        Write,
        stdout,
        stdin
    }
};

use colored::Colorize;


pub fn system_command(command: &str) {
    let exec_command = Command::new("sh").arg("-c").arg(format!("{}",command)).status();
    match exec_command {
        Ok(_) => println!("Command Executed {}", "Successfully".green().bold()),
        Err(e) => {
            println!("{}: {} To Execute Command", e, "Error".red().bold());
            exit(1);
        }
    }
}

fn main() {
    println!("What's your system?");
    println!("");
    println!("1- Archlinux");
    println!("2- Debian");
    println!("3- Fedora");
    println!("");
    println!("");
    print!("Desired choice:");
    stdout().flush().unwrap();
    let mut input: String = String::new();
    stdin().read_line(&mut input).expect("Error To Read User Input");


    match &input.trim().to_lowercase()[..] {
        "1" => {
            system_command("sudo pacman -Syu qemu-full virt-manager qemu-desktop libvirt edk2-ovmf dnsmasq iptables-nft --noconfirm");
            system_command("sudo systemctl enable libvirtd"); // Enabling the daemon: libvirtd on startup
            system_command("sudo systemctl start libvirtd"); // Starting the daemon: libvirtd
            exit(0); // Exit the program
        },

        "2" => {
            system_command("sudo apt install virt-manager qemu-kvm libvirt-clients libvirt-daemon-system bridge-utils virtinst libvirt-daemon -y");
            system_command("sudo systemctl enable libvirtd"); // Enabling the daemon: libvirtd on startup
            system_command("sudo systemctl start libvirtd"); // Starting the daemon: libvirtd
            exit(0); // Exit the program
        },

        "3" => {
            system_command("sudo dnf install @virtualization -y");
            system_command("sudo systemctl enable libvirtd"); // Enabling the daemon: libvirtd on startup
            system_command("sudo systemctl start libvirtd"); // Starting the daemon: libvirtd
            exit(0); // Exit the program
        },

        _ => {
            println!("Internal {}: System Not Found", "Error".red().bold());
            exit(1); // Exit the program with error
        }
    }
}