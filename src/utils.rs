use std::process::Command;

pub fn utils_fedora() {

    Command::new("dnf")
        .args(Some("install"))
        .args(Some("https://download1.rpmfusion.org/free/fedora/rpmfusion-free-release-35.noarch.rpm"))
        .args(Some("https://download1.rpmfusion.org/nonfree/fedora/rpmfusion-nonfree-release-35.noarch.rpm"))
        .args(Some("-y"))
        .status()
        .expect("Error enabling rpmfusion repository");

    Command::new("dnf")
        .args(Some("update"))
        .args(Some("-y"))
        .status()
        .expect("Error updating fedora 35");

    Command::new("dnf")
        .args(Some("install"))
        .args(Some("@base-x"))
        .args(Some("unrar"))
        .args(Some("p7zip"))
        .args(Some("zip"))
        .args(Some("unzip"))
        .args(Some("NetworkManager"))
        .args(Some("fedora-workstation-backgrounds"))
        .args(Some("firefox"))
        .args(Some("exfat-utils"))
        .args(Some("gvfs-mtp"))
        .args(Some("gvfs-goa"))
        .args(Some("system-config-printer"))
        .args(Some("-y"))
        .status()
        .expect("Error installing fedora 34 utilities");

    Command::new("systemctl")
        .args(Some("enable"))
        .args(Some("NetworkManager"))
        .args(Some("-f"))
        .status()
        .expect("Error starting NetworkManager at startup");
}

pub fn utils_debian() {

    Command::new("apt")
        .args(Some("install"))
        .args(Some("sudo"))
        .args(Some("zip"))
        .args(Some("unzip"))
        .args(Some("unrar-free"))
        .args(Some("network-manager"))
        .args(Some("xorg"))
        .args(Some("firefox-esr"))
        .args(Some("gvfs"))
        .args(Some("pulseaudio"))
        .args(Some("exfat-utils"))
        .args(Some("p7zip-full"))
        .args(Some("system-config-printer"))
        .args(Some("-y"))
        .status()
        .expect("Error installing debian utilities 11");

    Command::new("systemctl")
        .args(Some("enable"))
        .args(Some("NetworkManager"))
        .args(Some("-f"))
        .status()
        .expect("Error enabling NetworkManager on startup");

}

pub fn utils_archlinux() {

    Command::new("pacman")
        .args(Some("-Syu"))
        .args(Some("--noconfirm"))
        .status()
        .expect("Error updating system");

    Command::new("pacman")
        .args(Some("-Sy"))
        .args(Some("xorg"))
        .args(Some("networkmanager"))
        .args(Some("gvfs-mtp"))
        .args(Some("gvfs-goa"))
        .args(Some("gvfs-google"))
        .args(Some("exfat-utils"))
        .args(Some("p7zip"))
        .args(Some("firefox"))
        .args(Some("zip"))
        .args(Some("unzip"))
        .args(Some("unrar"))
        .args(Some("system-config-printer"))
        .args(Some("adwaita-icon-theme"))
        .args(Some("xf86-video-intel"))
        .args(Some("libgl"))
        .args(Some("mesa"))
        .args(Some("nvidia"))
        .args(Some("nvidia-libgl"))
        .args(Some("xf86-video-amdgpu"))
        .args(Some("--noconfirm"))
        .status()
        .expect("Error installing archlinux utilities");

    Command::new("systemctl")
        .args(Some("enable"))
        .args(Some("NetworkManager"))
        .args(Some("-f"))
        .status()
        .expect("Error starting NetworkManager at startup");

}