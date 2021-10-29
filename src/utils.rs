use std::process::Command;

pub fn utils_fedora() {

    Command::new("dnf")
        .args(Some("install"))
        .args(Some("https://download1.rpmfusion.org/free/fedora/rpmfusion-free-release-34.noarch.rpm"))
        .args(Some("https://download1.rpmfusion.org/nonfree/fedora/rpmfusion-nonfree-release-34.noarch.rpm"))
        .args(Some("-y"))
        .status()
        .expect("Erro ao habilitar o repositorio rpmfusion");

    Command::new("dnf")
        .args(Some("update"))
        .args(Some("-y"))
        .status()
        .expect("Erro ao atualizar o fedora 34");

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
        .expect("Erro ao instalar os utilitarios do fedora 34");

    Command::new("systemctl")
        .args(Some("enable"))
        .args(Some("NetworkManager"))
        .args(Some("-f"))
        .status()
        .expect("Erro ao iniciar o NetworkManager na inicializacao");
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
        .expect("Erro ao instalar os utilitarios do debian 11");

    Command::new("systemctl")
        .args(Some("enable"))
        .args(Some("NetworkManager"))
        .args(Some("-f"))
        .status()
        .expect("Erro ao habilitar o NetworkManager na inicializacao");

}

pub fn utils_archlinux() {

    Command::new("pacman")
        .args(Some("-Syu"))
        .args(Some("--noconfirm"))
        .status()
        .expect("Erro ao atualizar o sistema");

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
        .args(Some("--noconfirm"))
        .status()
        .expect("Erro ao instalar os utilitarios do archlinux");

    Command::new("systemctl")
        .args(Some("enable"))
        .args(Some("NetworkManager"))
        .args(Some("-f"))
        .status()
        .expect("Erro ao iniciar o NetworkManager na inicializacao");

}