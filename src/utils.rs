use std::{
    process::{
        Command, // Importing the standard command library for running operating system commands
        exit // Importing the standard exit library to exit the program
    }
};

pub fn remove_arch() {

    Command::new("sudo")
        .args(Some("pacman"))
        .args(Some("-Rsn"))
        .args(Some("lxde"))
        .args(Some("lightdm"))
        .args(Some("lightdm-gtk-greeter"))
        .args(Some("adwaita-icon-theme"))
        .args(Some("xarchiver"))
        .args(Some("lxqt"))
        .args(Some("xfce4-settings"))
        .args(Some("xfce4-pulseaudio-plugin"))
        .args(Some("exo"))
        .args(Some("garcon"))
        .args(Some("tumbler"))
        .args(Some("xfce4-panel"))
        .args(Some("xfce4-session"))
        .args(Some("xfce4-whiskermenu-plugin"))
        .args(Some("xfce4-terminal"))
        .args(Some("xfconf"))
        .args(Some("xfdesktop"))
        .args(Some("xfwm4"))
        .args(Some("thunar"))
        .args(Some("file-roller"))
        .args(Some("gdm"))
        .args(Some("weston"))
        .args(Some("gnome-session"))
        .args(Some("gnome-terminal"))
        .args(Some("nautilus-terminal"))
        .args(Some("nautilus"))
        .args(Some("gnome-control-center"))
        .args(Some("gedit"))
        .args(Some("eog"))
        .args(Some("evince"))
        .args(Some("cinnamon"))
        .args(Some("cinnamon-session"))
        .args(Some("cinnamon-desktop"))
        .args(Some("gnome-terminal"))
        .args(Some("cinnamon-control-center"))
        .args(Some("cinnamon-menus"))
        .args(Some("cinnamon-screensaver"))
        .args(Some("cinnamon-settings-daemon"))
        .args(Some("cinnamon-translations"))
        .args(Some("cjs"))
        .args(Some("muffin"))
        .args(Some("nemo"))
        .args(Some("nemo-fileroller"))
        .args(Some("mate-control-center"))
        .args(Some("mate-desktop"))
        .args(Some("mate-power-manager"))
        .args(Some("mate-screensaver"))
        .args(Some("mate-common"))
        .args(Some("mate-session-manager"))
        .args(Some("mate-settings-daemon"))
        .args(Some("mate-terminal"))
        .args(Some("mate-panel"))
        .args(Some("marco"))
        .args(Some("caja"))
        .args(Some("sddm"))
        .args(Some("plasma-desktop"))
        .args(Some("plasma-nm"))
        .args(Some("konsole"))
        .args(Some("plasma-wayland-session"))
        .args(Some("kcm-fcitx"))
        .args(Some("kscreen"))
        .args(Some("ksysguard"))
        .args(Some("spectacle"))
        .args(Some("dolphin"))
        .args(Some("discover"))
        .args(Some("cutefish"))
        .args(Some("--noconfirm"))
        .status()
        .expect("Error removing all graphical environments");

    Command::new("sudo")
        .args(Some("systemctl"))
        .args(Some("disable"))
        .args(Some("gdm"))
        .args(Some("-f"))
        .status()
        .expect("Error disabling gdm on startup");

    Command::new("sudo")
        .args(Some("systemctl"))
        .args(Some("disable"))
        .args(Some("lightdm"))
        .args(Some("-f"))
        .status()
        .expect("Error disabling lightdm on startup");

    Command::new("sudo")
        .args(Some("systemctl"))
        .args(Some("disable"))
        .args(Some("sddm"))
        .args(Some("-f"))
        .status()
        .expect("Error disabling sddm on startup");

}

pub fn utils_archlinux() {

    Command::new("sudo")
        .args(Some("pacman"))
        .args(Some("-Syu"))
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

    Command::new("sudo")
        .args(Some("systemctl"))
        .args(Some("enable"))
        .args(Some("NetworkManager"))
        .args(Some("-f"))
        .status()
        .expect("Error starting NetworkManager at startup");

}

pub fn remove_files_archlinux() {

    Command::new("sudo")
        .args(Some("mv"))
        .args(Some("/usr/share/applications/avahi-discover.desktop"))
        .args(Some("/usr/share/applications/avahi-discover.backup"))
        .status()
        .expect("Error to rename file: avahi-discover.desktop");

    Command::new("sudo")
        .args(Some("mv"))
        .args(Some("/usr/share/applications/bssh.desktop"))
        .args(Some("/usr/share/applications/bssh.backup"))
        .status()
        .expect("Error to rename file: bssh.desktop");

    Command::new("sudo")
        .args(Some("mv"))
        .args(Some("/usr/share/applications/bvnc.desktop"))
        .args(Some("/usr/share/applications/bvnc.backup"))
        .status()
        .expect("Error to rename file: bvnc.desktop");

    Command::new("sudo")
        .args(Some("mv"))
        .args(Some("/usr/share/applications/nm-connection-editor.desktop"))
        .args(Some("/usr/share/applications/nm-connection-editor.backup"))
        .status()
        .expect("Error to rename file: nm-connection-editor.desktop");

    Command::new("sudo")
        .args(Some("mv"))
        .args(Some("/usr/share/applications/qv4l2.desktop"))
        .args(Some("/usr/share/applications/qv4l2.backup"))
        .status()
        .expect("Error to rename file: qv4l2.desktop");

    Command::new("sudo")
        .args(Some("mv"))
        .args(Some("/usr/share/applications/qvidcap.desktop"))
        .args(Some("/usr/share/applications/qvidcap.backup"))
        .status()
        .expect("Error to rename file: qvidcap.desktop");

}

pub fn clean_arch() {

    Command::new("sudo")
        .args(Some("pacman"))
        .args(Some("-Rsn"))
        .args(Some("$(pacman -Qdtq)"))
        .args(Some("--noconfirm"))
        .status()
        .expect("Error to removing entire list of orphaned packages");

    Command::new("sudo")
        .args(Some("pacman"))
        .args(Some("-Scc"))
        .args(Some("--noconfirm"))
        .status()
        .expect("Error to clearing pacman cache");

    Command::new("sudo")
        .args(Some("flatpak"))
        .args(Some("uninstall"))
        .args(Some("--unused"))
        .status()
        .expect("Error to cleaning unused flatpaks");
        
    Command::new("sudo")
        .args(Some("rm"))
        .args(Some("-rf"))
        .args(Some("/var/lib/systemd/coredump/"))
        .status()
        .expect("Error to remove folder: /var/lib/systemd/coredump/, folder not found");

    Command::new("sudo")
        .args(Some("journalctl"))
        .args(Some("--vacuum-time=2d"))
        .status()
        .expect("Error to limiting systemd logs to 2 days");
        
    Command::new("journalctl")
        .args(Some("--vacuum-size=500M"))
        .status()
        .expect("Error limiting systemd logs to 500M");
        
    exit(0);

}

pub fn install_arch_lxde() {
    
    Command::new("sudo")
        .args(Some("pacman"))
        .args(Some("-Syu"))
        .args(Some("lxde"))
        .args(Some("lightdm"))
        .args(Some("lightdm-gtk-greeter"))
        .args(Some("adwaita-icon-theme"))
        .args(Some("xarchiver"))
        .args(Some("--noconfirm"))
        .status()
        .expect("Error installing minimal lxde on archlinux");

    Command::new("sudo")
        .args(Some("systemctl"))
        .args(Some("enable"))
        .args(Some("lightdm"))
        .args(Some("-f"))
        .status()
        .expect("Error enabling lightdm on startup");

}

pub fn install_arch_lxqt() {

    Command::new("sudo")
        .args(Some("pacman"))
        .args(Some("-Syu"))
        .args(Some("lxqt"))
        .args(Some("lightdm"))
        .args(Some("lightdm-gtk-greeter"))
        .args(Some("adwaita-icon-theme"))
        .args(Some("xarchiver"))
        .args(Some("--noconfirm"))
        .status()
        .expect("Error installing minimal lxqt on archlinux");

    Command::new("sudo")
        .args(Some("systemctl"))
        .args(Some("enable"))
        .args(Some("lightdm"))
        .args(Some("-f"))
        .status()
        .expect("Error enabling lightdm on startup");

}

pub fn install_arch_xfce() {

    Command::new("sudo")
        .args(Some("pacman"))
        .args(Some("-Syu"))
        .args(Some("lightdm"))
        .args(Some("lightdm-gtk-greeter"))
        .args(Some("xfce4-settings"))
        .args(Some("xfce4-pulseaudio-plugin"))
        .args(Some("adwaita-icon-theme"))
        .args(Some("exo"))
        .args(Some("garcon"))
        .args(Some("tumbler"))
        .args(Some("xfce4-panel"))
        .args(Some("xfce4-session"))
        .args(Some("xfce4-whiskermenu-plugin"))
        .args(Some("xfce4-terminal"))
        .args(Some("xfconf"))
        .args(Some("xfdesktop"))
        .args(Some("xfwm4"))
        .args(Some("thunar"))
        .args(Some("file-roller"))
        .args(Some("--noconfirm"))
        .status()
        .expect("Error installing xfce minimal on archlinux");

    Command::new("sudo")
        .args(Some("systemctl"))
        .args(Some("enable"))
        .args(Some("lightdm"))
        .args(Some("-f"))
        .status()
        .expect("Error enabling lightdm on startup");

}

pub fn install_arch_gnome() {

    Command::new("sudo")
        .args(Some("pacman"))
        .args(Some("-Syu"))
        .args(Some("gdm"))
        .args(Some("weston"))
        .args(Some("gnome-session"))
        .args(Some("gnome-terminal"))
        .args(Some("nautilus-terminal"))
        .args(Some("nautilus"))
        .args(Some("file-roller"))
        .args(Some("gnome-control-center"))
        .args(Some("gedit"))
        .args(Some("adwaita-icon-theme"))
        .args(Some("eog"))
        .args(Some("evince"))
        .args(Some("seahorse"))
        .args(Some("gnome-screenshot"))
        .args(Some("--noconfirm"))
        .status()
        .expect("Error installing gnome minimal on archlinux");
        
    Command::new("sudo")
        .args(Some("systemctl"))
        .args(Some("enable"))
        .args(Some("gdm"))
        .args(Some("-f"))
        .status()
        .expect("Error enabling gdm on startup");
    
    Command::new("gsettings")
        .args(Some("set"))
        .args(Some("org.gnome.desktop.interface"))
        .args(Some("enable-animations"))
        .args(Some("false"))
        .status()
        .expect("Error to disable animations on gnome");

}

pub fn install_arch_cinnamon() {

    Command::new("sudo")
        .args(Some("pacman"))
        .args(Some("-Syu"))
        .args(Some("lightdm"))
        .args(Some("lightdm-gtk-greeter"))
        .args(Some("cinnamon"))
        .args(Some("cinnamon-session"))
        .args(Some("cinnamon-desktop"))
        .args(Some("gnome-terminal"))
        .args(Some("cinnamon-control-center"))
        .args(Some("cinnamon-menus"))
        .args(Some("cinnamon-screensaver"))
        .args(Some("cinnamon-settings-daemon"))
        .args(Some("cinnamon-translations"))
        .args(Some("adwaita-icon-theme"))
        .args(Some("cjs"))
        .args(Some("muffin"))
        .args(Some("nemo"))
        .args(Some("nemo-fileroller"))
        .args(Some("file-roller"))
        .args(Some("--noconfirm"))
        .status()
        .expect("Error installing cinnamon minimal on archlinux");

    Command::new("sudo")
        .args(Some("systemctl"))
        .args(Some("enable"))
        .args(Some("lightdm"))
        .args(Some("-f"))
        .status()
        .expect("Error enabling lightdm on startup");

}

pub fn install_arch_mate() {

    Command::new("sudo")
        .args(Some("pacman"))
        .args(Some("-Syu"))
        .args(Some("lightdm"))
        .args(Some("lightdm-gtk-greeter"))
        .args(Some("mate-control-center"))
        .args(Some("adwaita-icon-theme"))
        .args(Some("mate-desktop"))
        .args(Some("mate-power-manager"))
        .args(Some("mate-screensaver"))
        .args(Some("mate-common"))
        .args(Some("mate-session-manager"))
        .args(Some("mate-settings-daemon"))
        .args(Some("mate-terminal"))
        .args(Some("network-manager-applet"))
        .args(Some("mate-panel"))
        .args(Some("marco"))
        .args(Some("caja"))
        .args(Some("--noconfirm"))
        .status()
        .expect("Error installing minimal mate on archlinux");

    Command::new("sudo")
        .args(Some("systemctl"))
        .args(Some("enable"))
        .args(Some("lightdm"))
        .args(Some("-f"))
        .status()
        .expect("Error enabling lightdm on startup");

}

pub fn install_arch_kdeplasma() {

    Command::new("sudo")
        .args(Some("pacman"))
        .args(Some("-Syu"))
        .args(Some("sddm"))
        .args(Some("plasma-desktop"))
        .args(Some("plasma-nm"))
        .args(Some("konsole"))
        .args(Some("plasma-wayland-session"))
        .args(Some("kcm-fcitx"))
        .args(Some("kscreen"))
        .args(Some("ksysguard"))
        .args(Some("adwaita-icon-theme"))
        .args(Some("spectacle"))
        .args(Some("dolphin"))
        .args(Some("discover"))
        .args(Some("--noconfirm"))
        .status()
        .expect("Error installing cinnamon minimal on archlinux");

    Command::new("sudo")
        .args(Some("systemctl"))
        .args(Some("enable"))
        .args(Some("sddm"))
        .args(Some("-f"))
        .status()
        .expect("Error enabling lightdm on startup");

}

pub fn install_arch_bspwm() {

    println!("Coming soon");
    
}

pub fn install_arch_cutefish() {

    println!("Coming soon");
    
}

pub fn remove_debian() {

    Command::new("sudo")
        .args(Some("apt"))
        .args(Some("remove"))
        .args(Some("lightdm"))
        .args(Some("lightdm-gtk-greeter"))
        .args(Some("lxde-core"))
        .args(Some("lxterminal"))
        .args(Some("lxappearance"))
        .args(Some("pavucontrol"))
        .args(Some("lxsession-default-apps"))
        .args(Some("xscreensaver"))
        .args(Some("policykit-1"))
        .args(Some("xarchiver"))
        .args(Some("lxqt-core"))
        .args(Some("pavucontrol"))
        .args(Some("thunar"))
        .args(Some("xfce4-panel"))
        .args(Some("xfce4-pulseaudio-plugin"))
        .args(Some("xfce4-whiskermenu-plugin"))
        .args(Some("xfce4-session"))
        .args(Some("xfce4-settings"))
        .args(Some("xfce4-terminal"))
        .args(Some("xfconf"))
        .args(Some("xfdesktop4"))
        .args(Some("xfwm4"))
        .args(Some("adwaita-qt"))
        .args(Some("qt5ct"))
        .args(Some("gdm3"))
        .args(Some("gnome-session"))
        .args(Some("gnome-control-center"))
        .args(Some("gnome-terminal"))
        .args(Some("gnome-tweaks"))
        .args(Some("nautilus"))
        .args(Some("adwaita-icon-theme"))
        .args(Some("cinnamon-core"))
        .args(Some("marco"))
        .args(Some("mate-desktop-environment-core"))
        .args(Some("dolphin"))
        .args(Some("kwrite"))
        .args(Some("ark"))
        .args(Some("kde-spectacle"))
        .args(Some("okular"))
        .args(Some("ksysguard"))
        .args(Some("plasma-discover"))
        .args(Some("kscreen"))
        .args(Some("konsole"))
        .args(Some("sddm"))
        .args(Some("kde-plasma-desktop"))
        .args(Some("plasma-nm"))
        .args(Some("plasma-workspace-wayland"))
        .args(Some("-y"))
        .status()
        .expect("Error removing all graphical environments");

    Command::new("sudo")
        .args(Some("systemctl"))
        .args(Some("disable"))
        .args(Some("gdm"))
        .args(Some("-f"))
        .status()
        .expect("Error disabling gdm on startup");

    Command::new("sudo")
        .args(Some("systemctl"))
        .args(Some("disable"))
        .args(Some("lightdm"))
        .args(Some("-f"))
        .status()
        .expect("Error disabling lightdm on startup");

    Command::new("sudo")
        .args(Some("systemctl"))
        .args(Some("disable"))
        .args(Some("sddm"))
        .args(Some("-f"))
        .status()
        .expect("Error disabling sddm on startup");

}

pub fn utils_debian() {

    Command::new("sudo")
        .args(Some("apt"))
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
        .args(Some("adwaita-icon-theme"))
        .args(Some("-y"))
        .status()
        .expect("Error installing debian utilities 11");

    Command::new("sudo")
        .args(Some("systemctl"))
        .args(Some("enable"))
        .args(Some("NetworkManager"))
        .args(Some("-f"))
        .status()
        .expect("Error enabling NetworkManager on startup");

}

pub fn remove_files_debian() {

    Command::new("sudo")
        .args(Some("mv"))
        .args(Some("/usr/share/applications/vim.desktop"))
        .args(Some("/usr/share/applications/vim.backup"))
        .status()
        .expect("Error to rename file: vim.desktop");

}

pub fn clean_debian() {

    Command::new("sudo")
        .args(Some("apt"))
        .args(Some("clean"))
        .status()
        .expect("Error to clearing apt cache");

    Command::new("sudo")
        .args(Some("apt"))
        .args(Some("autoclean"))
        .status()
        .expect("Error to cleaning dead packages");

    Command::new("sudo")
        .args(Some("apt"))
        .args(Some("autoremove"))
        .args(Some("-y"))
        .status()
        .expect("Error cleaning orphaned packages");

    Command::new("sudo")
        .args(Some("apt"))
        .args(Some("install"))
        .args(Some("deborphan"))
        .args(Some("-y"))
        .status()
        .expect("Error to installing deborphan");

    Command::new("sudo")
        .args(Some("apt"))
        .args(Some("remove"))
        .args(Some("$(deborphan)"))
        .args(Some("-y"))
        .status()
        .expect("Error to cleaning orphaned packages");

    Command::new("sudo")
        .args(Some("apt"))
        .args(Some("remove"))
        .args(Some("$(deborphan)"))
        .args(Some("-y"))
        .status()
        .expect("Error to cleaning 2 time orphaned packages");

    Command::new("sudo")
        .args(Some("apt"))
        .args(Some("remove"))
        .args(Some("$(deborphan)"))
        .args(Some("-y"))
        .status()
        .expect("Error to cleaning 3 time orphaned packages");

    Command::new("sudo")
        .args(Some("apt"))
        .args(Some("remove"))
        .args(Some("$(deborphan)"))
        .args(Some("-y"))
        .status()
        .expect("Error to cleaning 4 time orphaned packages");

    Command::new("sudo")
        .args(Some("apt"))
        .args(Some("remove"))
        .args(Some("deborphan"))
        .args(Some("-y"))
        .status()
        .expect("Error to remove deborphan from system");

    Command::new("sudo")
        .args(Some("apt"))
        .args(Some("autoremove"))
        .args(Some("-y"))
        .status()
        .expect("Error to removing deborphan dependencies");

    Command::new("sudo")
        .args(Some("rm"))
        .args(Some("-rf"))
        .args(Some("/var/lib/systemd/coredump/"))
        .status()
        .expect("Error to remove folder: /var/lib/systemd/coredump/, folder not found");

    Command::new("sudo")
        .args(Some("journalctl"))
        .args(Some("--vacuum-time=2d"))
        .status()
        .expect("Error to limiting systemd logs to 2 days");

    Command::new("journalctl")
        .args(Some("--vacuum-size=500M"))
        .status()
        .expect("Error limiting systemd logs to 500M");

    Command::new("sudo")
        .args(Some("flatpak"))
        .args(Some("--unused"))
        .status()
        .expect("Error to cleaning unused flatpaks");

    exit(0);

}

pub fn install_debian_lxde() {

    Command::new("sudo")
        .args(Some("apt"))
        .args(Some("install"))
        .args(Some("lightdm"))
        .args(Some("lightdm-gtk-greeter"))
        .args(Some("lxde-core"))
        .args(Some("lxterminal"))
        .args(Some("lxappearance"))
        .args(Some("pavucontrol"))
        .args(Some("lxsession-default-apps"))
        .args(Some("xscreensaver"))
        .args(Some("policykit-1"))
        .args(Some("xarchiver"))
        .args(Some("-y"))
        .status()
        .expect("Error installing minimal lxde on debian 11");

    Command::new("sudo")
        .args(Some("systemctl"))
        .args(Some("enable"))
        .args(Some("lightdm"))
        .args(Some("-f"))
        .status()
        .expect("Error enabling lightdm on startup");
        
}

pub fn install_debian_lxqt() {

    Command::new("sudo")
        .args(Some("apt"))
        .args(Some("install"))
        .args(Some("lxqt-core"))
        .args(Some("lightdm"))
        .args(Some("lightdm-gtk-greeter"))
        .args(Some("pavucontrol"))
        .args(Some("-y"))
        .status()
        .expect("Error installing lxqt minimal on debian 11");

    Command::new("sudo")
        .args(Some("systemctl"))
        .args(Some("enable"))
        .args(Some("lightdm"))
        .args(Some("-f"))
        .status()
        .expect("Error enabling lightdm on startup");

}

pub fn install_debian_xfce() {

    Command::new("sudo")
        .args(Some("apt"))
        .args(Some("install"))
        .args(Some("lightdm"))
        .args(Some("lightdm-gtk-greeter"))
        .args(Some("thunar"))
        .args(Some("xfce4-panel"))
        .args(Some("xfce4-pulseaudio-plugin"))
        .args(Some("xfce4-whiskermenu-plugin"))
        .args(Some("xfce4-session"))
        .args(Some("xfce4-settings"))
        .args(Some("xfce4-terminal"))
        .args(Some("pavucontrol"))
        .args(Some("xfconf"))
        .args(Some("xfdesktop4"))
        .args(Some("xfwm4"))
        .args(Some("adwaita-qt"))
        .args(Some("qt5ct"))
        .args(Some("--no-install-recommends"))
        .args(Some("-y"))
        .status()
        .expect("Error installing xfce4 minimal on debian 11");

    Command::new("sudo")
        .args(Some("systemctl"))
        .args(Some("enable"))
        .args(Some("lightdm"))
        .args(Some("-f"))
        .status()
        .expect("Error enabling lightdm on startup");

}

pub fn install_debian_gnome() {

    Command::new("sudo")
        .args(Some("apt"))
        .args(Some("install"))
        .args(Some("gdm3"))
        .args(Some("gnome-session"))
        .args(Some("gnome-control-center"))
        .args(Some("gnome-terminal"))
        .args(Some("gnome-tweaks"))
        .args(Some("nautilus"))
        .args(Some("adwaita-icon-theme"))
        .args(Some("seahorse"))
        .args(Some("--no-install-recommends"))
        .args(Some("-y"))
        .status()
        .expect("Error installing gnome minimal on debian 11");

    Command::new("sudo")
        .args(Some("systemctl"))
        .args(Some("enable"))
        .args(Some("gdm3"))
        .args(Some("-f"))
        .status()
        .expect("Error enabling gdm3 on startup");

    Command::new("gsettings")
        .args(Some("set"))
        .args(Some("org.gnome.desktop.interface"))
        .args(Some("enable-animations"))
        .args(Some("false"))
        .status()
        .expect("Error to disable animations on gnome");

}

pub fn install_debian_cinnamon() {

    Command::new("sudo")
        .args(Some("apt"))
        .args(Some("install"))
        .args(Some("cinnamon-core"))
        .args(Some("lightdm"))
        .args(Some("lightdm-gtk-greeter"))
        .args(Some("-y"))
        .status()
        .expect("Error installing cinnamon minimal on debian 11");

    Command::new("sudo")
        .args(Some("systemctl"))
        .args(Some("enable"))
        .args(Some("lightdm"))
        .args(Some("-f"))
        .status()
        .expect("Error enabling lightdm on startup");

}

pub fn install_debian_mate() {

    Command::new("sudo")
        .args(Some("apt"))
        .args(Some("install"))
        .args(Some("mate-desktop-environment-core"))
        .args(Some("lightdm"))
        .args(Some("lightdm-gtk-greeter"))
        .args(Some("marco"))
        .args(Some("-y"))
        .status()
        .expect("Error installing minimal mate on debian 11");

    Command::new("sudo")
        .args(Some("systemctl"))
        .args(Some("enable"))
        .args(Some("lightdm"))
        .args(Some("-f"))
        .status()
        .expect("Error enabling lightdm on startup");

}

pub fn install_debian_kdeplasma() {

    Command::new("sudo")
        .args(Some("apt"))
        .args(Some("install"))
        .args(Some("sddm"))
        .args(Some("kde-plasma-desktop"))
        .args(Some("plasma-nm"))
        .args(Some("plasma-workspace-wayland"))
        .args(Some("dolphin"))
        .args(Some("kwrite"))
        .args(Some("ark"))
        .args(Some("kde-spectacle"))
        .args(Some("okular"))
        .args(Some("ksysguard"))
        .args(Some("plasma-discover"))
        .args(Some("kscreen"))
        .args(Some("konsole"))
        .args(Some("--no-install-recommends"))
        .args(Some("-y"))
        .status()
        .expect("Error installing minimal kde plasma on debian 11");

    Command::new("sudo")
        .args(Some("systemctl"))
        .args(Some("enable"))
        .args(Some("sddm"))
        .args(Some("-f"))
        .status()
        .expect("Error enabling sddm on startup");

}

pub fn install_debian_bspwm() {

    println!("Coming soon");
    
}

pub fn install_debian_cutefish() {

    println!("Coming soon");
    
}

pub fn remove_fedora() {

    Command::new("sudo")
        .args(Some("dnf"))
        .args(Some("remove"))
        .args(Some("@lxde-desktop"))
        .args(Some("@plasma-desktop"))
        .args(Some("@gnome-desktop"))
        .args(Some("@cinnamon-desktop"))
        .args(Some("@lxqt-desktop"))
        .args(Some("@mate-desktop"))
        .args(Some("@xfce-desktop"))
        .args(Some("lxappearance"))
        .args(Some("lxde-common"))
        .args(Some("lxdm"))
        .args(Some("lxinput"))
        .args(Some("lxmenu-data"))
        .args(Some("lxpanel"))
        .args(Some("lxpolkit"))
        .args(Some("lxrandr"))
        .args(Some("xcompmgr"))
        .args(Some("xarchiver"))
        .args(Some("lxsession"))
        .args(Some("lxtask"))
        .args(Some("pcmanfm"))
        .args(Some("lxterminal"))
        .args(Some("network-manager-applet"))
        .args(Some("openbox"))
        .args(Some("obconf"))
        .args(Some("lightdm-gtk-greeter"))
        .args(Some("lightdm"))
        .args(Some("sddm"))
        .args(Some("plasma-desktop"))
        .args(Some("plasma-nm"))
        .args(Some("konsole"))
        .args(Some("kcm_colors"))
        .args(Some("kcm-fcitx"))
        .args(Some("kscreen"))
        .args(Some("ksysguard"))
        .args(Some("spectacle"))
        .args(Some("plasma-user-manager"))
        .args(Some("dolphin"))
        .args(Some("plasma-discover"))
        .args(Some("gdm"))
        .args(Some("gnome-shell"))
        .args(Some("nautilus"))
        .args(Some("gnome-terminal"))
        .args(Some("fedora-workstation-backgrounds"))
        .args(Some("file-roller"))
        .args(Some("gnome-terminal-nautilus"))
        .args(Some("cinnamon"))
        .args(Some("cinnamon-control-center"))
        .args(Some("cinnamon-desktop"))
        .args(Some("cinnamon-menus"))
        .args(Some("cinnamon-screensaver"))
        .args(Some("cinnamon-session"))
        .args(Some("nemo"))
        .args(Some("nemo-fileroller"))
        .args(Some("cinnamon-translations"))
        .args(Some("cjs"))
        .args(Some("muffin"))
        .args(Some("gnome-terminal"))
        .args(Some("breeze-cursor-theme"))
        .args(Some("breeze-gtk"))
        .args(Some("breeze-icon-theme"))
        .args(Some("firewall-config"))
        .args(Some("network-manager-applet"))
        .args(Some("notification-daemon"))
        .args(Some("obconf"))
        .args(Some("openbox"))
        .args(Some("pcmanfm-qt"))
        .args(Some("qterminal"))
        .args(Some("lxqt-about"))
        .args(Some("lxqt-archiver"))
        .args(Some("lxqt-config"))
        .args(Some("lxqt-notificationd"))
        .args(Some("lxqt-openssh-askpass"))
        .args(Some("lxqt-panel"))
        .args(Some("lxqt-policykit"))
        .args(Some("lxqt-powermanagement"))
        .args(Some("lxqt-qtplugin"))
        .args(Some("lxqt-session"))
        .args(Some("lxqt-themes"))
        .args(Some("lxqt-themes-fedora"))
        .args(Some("network-manager-applet"))
        .args(Some("xfwm4"))
        .args(Some("xfce4-power-manager"))
        .args(Some("xfce4-session"))
        .args(Some("xfce4-settings"))
        .args(Some("xfce4-whiskermenu-plugin"))
        .args(Some("xfdesktop"))
        .args(Some("xfce4-terminal"))
        .args(Some("mate-control-center"))
        .args(Some("mate-desktop"))
        .args(Some("mate-power-manager"))
        .args(Some("mate-screensaver"))
        .args(Some("mate-screenshot"))
        .args(Some("mate-session-manager"))
        .args(Some("mate-settings-daemon"))
        .args(Some("mate-terminal"))
        .args(Some("network-manager-applet"))
        .args(Some("mate-panel"))
        .args(Some("marco"))
        .args(Some("caja"))
        .args(Some("gstreamer1-plugins-base"))    
        .args(Some("gstreamer1-plugins-good"))
        .args(Some("gstreamer1-plugins-ugly"))
        .args(Some("gstreamer1-plugins-bad-free"))
        .args(Some("gstreamer1-plugins-bad-free"))
        .args(Some("gstreamer1-plugins-bad-freeworld"))
        .args(Some("gstreamer1-plugins-bad-free-extras"))
        .args(Some("ffmpeg"))
        .args(Some("-y"))
        .status()
        .expect("Error removing graphical environments and their dependencies");

    Command::new("sudo")
        .args(Some("systemctl"))
        .args(Some("disable"))
        .args(Some("sddm"))
        .args(Some("-f"))
        .status()
        .expect("Error removing sddm login screen");

    Command::new("sudo")
        .args(Some("systemctl"))
        .args(Some("disable"))
        .args(Some("lightdm"))
        .args(Some("-f"))
        .status()
        .expect("Error removing sddm login screen");

    Command::new("sudo")
        .args(Some("systemctl"))
        .args(Some("disable"))
        .args(Some("gdm"))
        .args(Some("-f"))
        .status()
        .expect("Error removing sddm login screen");

}

pub fn utils_fedora() {

    Command::new("sudo")
        .args(Some("dnf"))
        .args(Some("install"))
        .args(Some("https://download1.rpmfusion.org/free/fedora/rpmfusion-free-release-35.noarch.rpm"))
        .args(Some("https://download1.rpmfusion.org/nonfree/fedora/rpmfusion-nonfree-release-35.noarch.rpm"))
        .args(Some("-y"))
        .status()
        .expect("Error enabling rpmfusion repository");

    Command::new("sudo")
        .args(Some("dnf"))
        .args(Some("update"))
        .args(Some("-y"))
        .status()
        .expect("Error updating fedora 35");

    Command::new("sudo")
        .args(Some("dnf"))
        .args(Some("install"))
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
        .args(Some("gstreamer1-plugins-base"))    
        .args(Some("gstreamer1-plugins-good"))
        .args(Some("gstreamer1-plugins-ugly"))
        .args(Some("gstreamer1-plugins-bad-free"))
        .args(Some("gstreamer1-plugins-bad-free"))
        .args(Some("gstreamer1-plugins-bad-freeworld"))
        .args(Some("gstreamer1-plugins-bad-free-extras"))
        .args(Some("ffmpeg"))
        .args(Some("-y"))
        .status()
        .expect("Error installing fedora 35 utilities");

    Command::new("sudo")
        .args(Some("systemctl"))
        .args(Some("enable"))
        .args(Some("NetworkManager"))
        .args(Some("-f"))
        .status()
        .expect("Error starting NetworkManager at startup");

}

pub fn install_fedora_lxde() {

    Command::new("sudo")
        .args(Some("dnf"))
        .args(Some("install"))
        .args(Some("lxappearance"))
        .args(Some("lxde-common"))
        .args(Some("lxdm"))
        .args(Some("lxinput"))
        .args(Some("lxmenu-data"))
        .args(Some("lxpanel"))
        .args(Some("lxpolkit"))
        .args(Some("lxrandr"))
        .args(Some("xcompmgr"))
        .args(Some("xarchiver"))
        .args(Some("lxsession"))
        .args(Some("lxtask"))
        .args(Some("pcmanfm"))
        .args(Some("lxterminal"))
        .args(Some("network-manager-applet"))
        .args(Some("openbox"))
        .args(Some("obconf"))
        .args(Some("lightdm-gtk-greeter"))
        .args(Some("lightdm"))
        .args(Some("-y"))
        .status()
        .expect("Error installing minimal lxde on fedora 35");

    Command::new("sudo")
        .args(Some("systemctl"))
        .args(Some("enable"))
        .args(Some("lightdm"))
        .args(Some("-f"))
        .status()
        .expect("Error enabling lightdm on startup");

    Command::new("sudo")
        .args(Some("systemctl"))
        .args(Some("set-default"))
        .args(Some("graphical.target"))
        .status()
        .expect("Error enabling graphical mode boot");

}

pub fn install_fedora_lxqt() {
    
    Command::new("sudo")
        .args(Some("dnf"))
        .args(Some("install"))
        .args(Some("lightdm"))
        .args(Some("lightdm-gtk-greeter"))
        .args(Some("breeze-cursor-theme"))
        .args(Some("breeze-gtk"))
        .args(Some("breeze-icon-theme"))
        .args(Some("firewall-config"))
        .args(Some("network-manager-applet"))
        .args(Some("notification-daemon"))
        .args(Some("obconf"))
        .args(Some("openbox"))
        .args(Some("pcmanfm-qt"))
        .args(Some("qterminal"))
        .args(Some("lxqt-about"))
        .args(Some("lxqt-archiver"))
        .args(Some("lxqt-config"))
        .args(Some("lxqt-notificationd"))
        .args(Some("lxqt-openssh-askpass"))
        .args(Some("lxqt-panel"))
        .args(Some("lxqt-policykit"))
        .args(Some("lxqt-powermanagement"))
        .args(Some("lxqt-qtplugin"))
        .args(Some("lxqt-session"))
        .args(Some("lxqt-themes"))
        .args(Some("lxqt-themes-fedora"))
        .args(Some("-y"))
        .status()
        .expect("Error installing lxqt minimal on fedora 35");

    Command::new("sudo")
        .args(Some("systemctl"))
        .args(Some("enable"))
        .args(Some("lightdm"))
        .args(Some("-f"))
        .status()
        .expect("Error enabling lightdm on startup");

    Command::new("sudo")
        .args(Some("systemctl"))
        .args(Some("set-default"))
        .args(Some("graphical.target"))
        .status()
        .expect("Error enabling graphical mode boot");

}

pub fn install_fedora_xfce() {

    Command::new("sudo")
        .args(Some("dnf"))
        .args(Some("install"))
        .args(Some("network-manager-applet"))
        .args(Some("xfwm4"))
        .args(Some("xfce4-power-manager"))
        .args(Some("xfce4-session"))
        .args(Some("xfce4-settings"))
        .args(Some("xfce4-whiskermenu-plugin"))
        .args(Some("xfdesktop"))
        .args(Some("lightdm"))
        .args(Some("lightdm-gtk-greeter"))
        .args(Some("xfce4-terminal"))
        .args(Some("-y"))
        .status()
        .expect("Error installing xfce4 minimal on fedora 35");

    Command::new("sudo")
        .args(Some("systemctl"))
        .args(Some("enable"))
        .args(Some("lightdm"))
        .args(Some("-f"))
        .status()
        .expect("Error enabling lightdm on startup");

    Command::new("sudo")
        .args(Some("systemctl"))
        .args(Some("set-default"))
        .args(Some("graphical.target"))
        .status()
        .expect("Error enabling graphical mode boot");

}

pub fn install_fedora_gnome() {

    Command::new("sudo")
        .args(Some("dnf"))
        .args(Some("install"))
        .args(Some("gdm"))
        .args(Some("gnome-shell"))
        .args(Some("nautilus"))
        .args(Some("gnome-terminal"))
        .args(Some("fedora-workstation-backgrounds"))
        .args(Some("file-roller"))
        .args(Some("gnome-terminal-nautilus"))
        .args(Some("seahorse"))
        .args(Some("-y"))
        .status()
        .expect("Error installing gnome on fedora 35");

    Command::new("sudo")
        .args(Some("systemctl"))
        .args(Some("enable"))
        .args(Some("gdm"))
        .args(Some("-f"))
        .spawn()
        .expect("Error enabling gdm on startup");

    Command::new("sudo")
        .args(Some("systemctl"))
        .args(Some("set-default"))
        .args(Some("graphical.target"))
        .status()
        .expect("Error enabling graphical mode boot");

    Command::new("gsettings")
        .args(Some("set"))
        .args(Some("org.gnome.desktop.interface"))
        .args(Some("enable-animations"))
        .args(Some("false"))
        .status()
        .expect("Error to disable animations on gnome");

}

pub fn install_fedora_cinnamon() {

    Command::new("sudo")
        .args(Some("dnf"))
        .args(Some("install"))
        .args(Some("cinnamon"))
        .args(Some("cinnamon-control-center"))
        .args(Some("cinnamon-desktop"))
        .args(Some("cinnamon-menus"))
        .args(Some("cinnamon-screensaver"))
        .args(Some("cinnamon-session"))
        .args(Some("nemo"))
        .args(Some("nemo-fileroller"))
        .args(Some("cinnamon-translations"))
        .args(Some("cjs"))
        .args(Some("muffin"))
        .args(Some("gnome-terminal"))
        .args(Some("lightdm"))
        .args(Some("lightdm-gtk-greeter"))
        .args(Some("-y"))
        .status()
        .expect("Error installing cinnamon on fedora 35");

    Command::new("sudo")
        .args(Some("systemctl"))
        .args(Some("enable"))
        .args(Some("lightdm"))
        .args(Some("-f"))
        .status()
        .expect("Error enabling lightdm on startup");

    Command::new("sudo")
        .args(Some("systemctl"))
        .args(Some("set-default"))
        .args(Some("graphical.target"))
        .status()
        .expect("Error enabling graphical mode boot");

}

pub fn install_fedora_mate() {
    
    Command::new("sudo")
        .args(Some("dnf"))
        .args(Some("install"))
        .args(Some("lightdm"))
        .args(Some("lightdm-gtk-greeter"))
        .args(Some("mate-control-center"))
        .args(Some("mate-desktop"))
        .args(Some("mate-power-manager"))
        .args(Some("mate-screensaver"))
        .args(Some("mate-screenshot"))
        .args(Some("mate-session-manager"))
        .args(Some("mate-settings-daemon"))
        .args(Some("mate-terminal"))
        .args(Some("network-manager-applet"))
        .args(Some("mate-panel"))
        .args(Some("marco"))
        .args(Some("caja"))
        .args(Some("-y"))
        .status()
        .expect("Error installing mate in fedora 35");

    Command::new("sudo")
        .args(Some("systemctl"))
        .args(Some("enable"))
        .args(Some("lightdm"))
        .args(Some("-f"))
        .status()
        .expect("Error enabling lightdm on startup");

    Command::new("sudo")
        .args(Some("systemctl"))
        .args(Some("set-default"))
        .args(Some("graphical.target"))
        .status()
        .expect("Error enabling graphical mode boot");

}

pub fn install_fedora_kdeplasma() {
    
    Command::new("sudo")
        .args(Some("dnf"))
        .args(Some("install"))
        .args(Some("sddm"))
        .args(Some("plasma-desktop"))
        .args(Some("plasma-nm"))
        .args(Some("konsole"))
        .args(Some("kcm_colors"))
        .args(Some("kcm-fcitx"))
        .args(Some("kscreen"))
        .args(Some("ksysguard"))
        .args(Some("spectacle"))
        .args(Some("plasma-user-manager"))
        .args(Some("dolphin"))
        .args(Some("plasma-discover"))
        .args(Some("-y"))
        .status()
        .expect("Error installing kde plasma on fedora 35");

    Command::new("sudo")
        .args(Some("systemctl"))
        .args(Some("enable"))
        .args(Some("sddm"))
        .args(Some("-f"))
        .status()
        .expect("Error enabling sddm on startup");

    Command::new("sudo")
        .args(Some("systemctl"))
        .args(Some("set-default"))
        .args(Some("graphical.target"))
        .status()
        .expect("Error enabling graphical mode boot");

}

pub fn install_fedora_bspwm() {

    println!("Coming soon");

}

pub fn install_fedora_cutefish() {
    
    println!("Coming soon");

}

pub fn clean_fedora() {

    Command::new("sudo")
        .args(Some("dnf"))
        .args(Some("clean"))
        .args(Some("all"))
        .status()
        .expect("Error to clean dnf cache");

    Command::new("sudo")
        .args(Some("dnf"))
        .args(Some("autoremove"))
        .args(Some("-y"))
        .status()
        .expect("Error removing orphaned dnf packages");

    Command::new("sudo")
        .args(Some("rm"))
        .args(Some("-rf"))
        .args(Some("/var/lib/systemd/coredump/"))
        .status()
        .expect("Error to remove folder: /var/lib/systemd/coredump/, folder not found");

    Command::new("sudo")
        .args(Some("journalctl"))
        .args(Some("--vacuum-time=2d"))
        .status()
        .expect("Error to limiting systemd logs to 2 days");

    Command::new("journalctl")
        .args(Some("--vacuum-size=500M"))
        .status()
        .expect("Error limiting systemd logs to 500M");

    Command::new("sudo")
        .args(Some("flatpak"))
        .args(Some("--unused"))
        .status()
        .expect("Error to cleaning unused flatpaks");

    exit(0);

}

pub fn configure_dnf_fedora() {
    Command::new("sudo")
        .args(Some("rm"))
        .args(Some("-r"))
        .args(Some("/etc/dnf/dnf.conf"))
        .status()
        .expect("Error to remove /etc/dnf/dnf.conf");

    Command::new("sudo")
        .args(Some("echo"))
        .args(Some(r#""[main]""#))
        .args(Some(">>"))
        .args(Some("/etc/dnf/dnf.conf"))
        .status()
        .expect("File: /etc/dnf/dnf.conf not found");

    Command::new("sudo")
        .args(Some("echo"))
        .args(Some(r#""gpgcheck=1""#))
        .args(Some(">>"))
        .args(Some("/etc/dnf/dnf.conf"))
        .status()
        .expect("File: /etc/dnf/dnf.conf not found");

    Command::new("sudo")
        .args(Some("echo"))
        .args(Some(r#""installonly_limit=3""#))
        .args(Some(">>"))
        .args(Some("/etc/dnf/dnf.conf"))
        .status()
        .expect("File: /etc/dnf/dnf.conf not found");

    Command::new("sudo")
        .args(Some("echo"))
        .args(Some(r#""clean_requirements_on_remove=True""#))
        .args(Some(">>"))
        .args(Some("/etc/dnf/dnf.conf"))
        .status()
        .expect("File: /etc/dnf/dnf.conf not found");

    Command::new("sudo")
        .args(Some("echo"))
        .args(Some(r#""best=False""#))
        .args(Some(">>"))
        .args(Some("/etc/dnf/dnf.conf"))
        .status()
        .expect("File: /etc/dnf/dnf.conf not found");

    Command::new("sudo")
        .args(Some("echo"))
        .args(Some(r#""skip_if_unavailable=True""#))
        .args(Some(">>"))
        .args(Some("/etc/dnf/dnf.conf"))
        .status()
        .expect("File: /etc/dnf/dnf.conf not found");

    Command::new("sudo")
        .args(Some("echo"))
        .args(Some(r#""fastestmirror=True""#))
        .args(Some(">>"))
        .args(Some("/etc/dnf/dnf.conf"))
        .status()
        .expect("File: /etc/dnf/dnf.conf not found");
        
    Command::new("sudo")
        .args(Some("echo"))
        .args(Some(r#""max_parallel_downloads=7""#))
        .args(Some(">>"))
        .args(Some("/etc/dnf/dnf.conf"))
        .status()
        .expect("File: /etc/dnf/dnf.conf not found");

    Command::new("sudo")
        .args(Some("echo"))
        .args(Some(r#""defaultyes=True""#))
        .args(Some(">>"))
        .args(Some("/etc/dnf/dnf.conf"))
        .status()
        .expect("File: /etc/dnf/dnf.conf not found");

    Command::new("sudo")
        .args(Some("echo"))
        .args(Some(r#""install_weak_deps=false""#))
        .args(Some(">>"))
        .args(Some("/etc/dnf/dnf.conf"))
        .status()
        .expect("File: /etc/dnf/dnf.conf not found");

}