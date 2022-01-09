use std::process::Command;
use std::process;
use std::env;
use std::io;
mod texts;
mod lib;
mod utils;

fn main() {

    let args: Vec<String> = env::args().collect();
    let opcao = &args[1].trim();

    match &opcao[..] {

        "--clean-arch" => {

            Command::new("pacman")
                .args(Some("-Rsn"))
                .args(Some("$(pacman -Qdtq)"))
                .args(Some("--noconfirm"))
                .status()
                .expect("Error removing entire list of orphaned packages");

            Command::new("pacman")
                .args(Some("-Scc"))
                .args(Some("--noconfirm"))
                .status()
                .expect("Error clearing pacman cache");

            Command::new("flatpak")
                .args(Some("uninstall"))
                .args(Some("--unused"))
                .status()
                .expect("Error cleaning unused flatpaks");

            lib::dir("/var/lib/systemd/coredump/", "Folder /var/lib/systemd/coredump/ not found");

            Command::new("journalctl")
                .args(Some("--vacuum-time=2d"))
                .status()
                .expect("Error limiting systemd logs to 2 days");

            Command::new("journalctl")
                .args(Some("--vacuum-size=500M"))
                .status()
                .expect("Error limiting systemd logs to 500M");

            process::exit(0);
        },

        "--clean-fedora" => {

            Command::new("dnf")
                .args(Some("clean"))
                .args(Some("all"))
                .status()
                .expect("Error to clean dnf cache");
            
            Command::new("dnf")
                .args(Some("autoremove"))
                .args(Some("-y"))
                .status()
                .expect("error removing orphaned dnf packages");

            lib::dir("/var/lib/systemd/coredump/", "Folder /var/lib/systemd/coredump/ not found");

            Command::new("journalctl")
                .args(Some("--vacuum-time=2d"))
                .status()
                .expect("Error limiting systemd logs to 2 days");

            Command::new("journalctl")
                .args(Some("--vacuum-size=500M"))
                .status()
                .expect("Error limiting systemd logs to 500M");

            Command::new("flatpak")
                .args(Some("uninstall"))
                .args(Some("--unused"))
                .status()
                .expect("Error cleaning unused flatpaks");

            process::exit(0);
        },

        "--clean-debian" => {

            Command::new("apt")
                .args(Some("clean"))
                .status()
                .expect("Error clearing apt cache");

            Command::new("apt")
                .args(Some("autoclean"))
                .status()
                .expect("Error cleaning dead packages");

            Command::new("apt")
                .args(Some("autoremove"))
                .args(Some("-y"))
                .status()
                .expect("Error cleaning orphaned packages");

            Command::new("apt")
                .args(Some("install"))
                .args(Some("deborphan"))
                .args(Some("-y"))
                .status()
                .expect("Error installing deborphan");

            Command::new("apt")
                .args(Some("remove"))
                .args(Some("$(deborphan)"))
                .args(Some("-y"))
                .status()
                .expect("Error cleaning orphaned packages");
        
            Command::new("apt")
                .args(Some("remove"))
                .args(Some("$(deborphan)"))
                .args(Some("-y"))
                .status()
                .expect("Error cleaning 2 time orphaned packages");
        
            Command::new("apt")
                .args(Some("remove"))
                .args(Some("$(deborphan)"))
                .args(Some("-y"))
                .status()
                .expect("Error cleaning 3 time orphaned packages");

            Command::new("apt")
                .args(Some("remove"))
                .args(Some("$(deborphan)"))
                .args(Some("-y"))
                .status()
                .expect("Error cleaning 4 time orphaned packages");
        
            Command::new("apt")
                .args(Some("remove"))
                .args(Some("deborphan"))
                .args(Some("-y"))
                .status()
                .expect("Error to remove deborphan");

            Command::new("apt")
                .args(Some("autoremove"))
                .args(Some("-y"))
                .status()
                .expect("Error removing deborphan dependencies");
        
            lib::dir("/var/lib/systemd/coredump/", "Folder /var/lib/systemd/coredump/ not found");

            Command::new("journalctl")
                .args(Some("--vacuum-time=2d"))
                .status()
                .expect("Error limiting systemd logs to 2 days");

            Command::new("journalctl")
                .args(Some("--vacuum-size=500M"))
                .status()
                .expect("Error limiting systemd logs to 500M");

            Command::new("flatpak")
                .args(Some("uninstall"))
                .args(Some("--unused"))
                .status()
                .expect("Error cleaning unused flatpaks");

            process::exit(0);
            
        },

        "--install-arch-lxde" => {

            utils::remove_arch();

            utils::utils_archlinux();

            Command::new("pacman")
                .args(Some("-Sy"))
                .args(Some("lxde"))
                .args(Some("lightdm"))
                .args(Some("lightdm-gtk-greeter"))
                .args(Some("adwaita-icon-theme"))
                .args(Some("xarchiver"))
                .args(Some("--noconfirm"))
                .status()
                .expect("Error installing minimal lxde on archlinux");
        
            Command::new("systemctl")
                .args(Some("enable"))
                .args(Some("lightdm"))
                .args(Some("-f"))
                .status()
                .expect("Error enabling lightdm on startup");

            utils::remove_files_archlinux();

            Command::new("reboot")
                .status()
                .expect("Error restarting system");
        },

        "--install-arch-lxqt" => {

            utils::remove_arch();

            utils::utils_archlinux();

            Command::new("pacman")
                .args(Some("-Sy"))
                .args(Some("lxqt"))
                .args(Some("lightdm"))
                .args(Some("lightdm-gtk-greeter"))
                .args(Some("adwaita-icon-theme"))
                .args(Some("xarchiver"))
                .args(Some("--noconfirm"))
                .status()
                .expect("Error installing minimal lxqt on archlinux");
        
            Command::new("systemctl")
                .args(Some("enable"))
                .args(Some("lightdm"))
                .args(Some("-f"))
                .status()
                .expect("Error enabling lightdm on startup");

            utils::remove_files_archlinux();

            Command::new("reboot")
                .status()
                .expect("Error restarting system");
    
        },
        
        "--install-arch-xfce" => {

            utils::remove_arch();

            utils::utils_archlinux();

            Command::new("pacman")
                .args(Some("-Sy"))
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
        
            Command::new("systemctl")
                .args(Some("enable"))
                .args(Some("lightdm"))
                .args(Some("-f"))
                .status()
                .expect("Error enabling lightdm on startup");

            utils::remove_files_archlinux();

            Command::new("reboot")
                .status()
                .expect("Error restarting system");

        },

        "--install-arch-gnome" => {

            utils::remove_arch();

            utils::utils_archlinux();

            Command::new("pacman")
                .args(Some("-Sy"))
                .args(Some("gdm"))
                .args(Some("xorg-server"))
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
                .args(Some("--noconfirm"))
                .status()
                .expect("Error installing gnome minimal on archlinux");
        
            Command::new("systemctl")
                .args(Some("enable"))
                .args(Some("gdm"))
                .args(Some("-f"))
                .status()
                .expect("Error enabling gdm on startup");

            utils::remove_files_archlinux();

            Command::new("gsettings")
                .args(Some("set"))
                .args(Some("org.gnome.desktop.interface"))
                .args(Some("enable-animations"))
                .args(Some("false"))
                .status()
                .expect("Error to disable animations on gnome");
    
            Command::new("reboot")
                .status()
                .expect("Error restarting system");

        },

        "--install-arch-cinnamon" => {

            utils::remove_arch();

            utils::utils_archlinux();

            Command::new("pacman")
                .args(Some("-Sy"))
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

            Command::new("systemctl")
                .args(Some("enable"))
                .args(Some("lightdm"))
                .args(Some("-f"))
                .status()
                .expect("Error enabling lightdm on startup");

            utils::remove_files_archlinux();

            Command::new("reboot")
                .status()
                .expect("Error restarting system");

        },

        "--install-arch-mate" => {

            utils::remove_arch();

            utils::utils_archlinux();

            Command::new("pacman")
                .args(Some("-Sy"))
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

            Command::new("systemctl")
                .args(Some("enable"))
                .args(Some("lightdm"))
                .args(Some("-f"))
                .status()
                .expect("Error enabling lightdm on startup");

            utils::remove_files_archlinux();
    
            Command::new("reboot")
                .status()
                .expect("Error restarting system");

        },

        "--install-arch-kdeplasma" => {
            
            utils::remove_arch();

            utils::utils_archlinux();

            Command::new("pacman")
                .args(Some("-Sy"))
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

            Command::new("systemctl")
                .args(Some("enable"))
                .args(Some("sddm"))
                .args(Some("-f"))
                .status()
                .expect("Error enabling lightdm on startup");
                
            utils::remove_files_archlinux();

            Command::new("reboot")
                .status()
                .expect("Error restarting system");

        },

        
        "--install-debian-lxde" => {

            utils::remove_debian();

            utils::utils_debian();

            Command::new("apt")
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
        
            Command::new("systemctl")
                .args(Some("enable"))
                .args(Some("lightdm"))
                .args(Some("-f"))
                .status()
                .expect("Error enabling lightdm on startup");

            utils::remove_files_debian();

            Command::new("reboot")
                .status()
                .expect("Error restarting system");

        },

        "--install-debian-lxqt" => {

            utils::remove_debian();

            utils::utils_debian();

            Command::new("apt")
                .args(Some("install"))
                .args(Some("lxqt-core"))
                .args(Some("lightdm"))
                .args(Some("lightdm-gtk-greeter"))
                .args(Some("pavucontrol"))
                .args(Some("-y"))
                .status()
                .expect("Error installing lxqt minimal on debian 11");

            Command::new("systemctl")
                .args(Some("enable"))
                .args(Some("lightdm"))
                .args(Some("-f"))
                .status()
                .expect("Error enabling lightdm on startup");

            utils::remove_files_debian();

            Command::new("reboot")
                .status()
                .expect("Error restarting system");

        },

        "--install-debian-xfce" => {

            utils::remove_debian();

            utils::utils_debian();

            Command::new("apt")
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

            Command::new("systemctl")
                .args(Some("enable"))
                .args(Some("lightdm"))
                .args(Some("-f"))
                .status()
                .expect("Error enabling lightdm on startup");

            utils::remove_files_debian();

            Command::new("reboot")
                .status()
                .expect("Error restarting system");

        },

        "--install-debian-gnome" => {

            utils::remove_debian();

            utils::utils_debian();

            Command::new("apt")
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

            Command::new("systemctl")
                .args(Some("enable"))
                .args(Some("gdm3"))
                .args(Some("-f"))
                .status()
                .expect("Error enabling gdm3 on startup");

            utils::remove_files_debian();
            
            Command::new("gsettings")
                .args(Some("set"))
                .args(Some("org.gnome.desktop.interface"))
                .args(Some("enable-animations"))
                .args(Some("false"))
                .status()
                .expect("Error to disable animations on gnome");

            Command::new("reboot")
                .status()
                .expect("Error restarting system");

        },

        "--install-debian-cinnamon" => {

            utils::remove_debian();

            utils::utils_debian();

            Command::new("apt")
                .args(Some("install"))
                .args(Some("cinnamon-core"))
                .args(Some("lightdm"))
                .args(Some("lightdm-gtk-greeter"))
                .args(Some("-y"))
                .status()
                .expect("Error installing cinnamon minimal on debian 11");

            Command::new("systemctl")
                .args(Some("enable"))
                .args(Some("lightdm"))
                .args(Some("-f"))
                .status()
                .expect("Error enabling lightdm on startup");

            utils::remove_files_debian();

            Command::new("reboot")
                .status()
                .expect("Error restarting system");

        },

        "--install-debian-mate" => {

            utils::remove_debian();

            utils::utils_debian();

            Command::new("apt")
                .args(Some("install"))
                .args(Some("mate-desktop-environment-core"))
                .args(Some("lightdm"))
                .args(Some("lightdm-gtk-greeter"))
                .args(Some("marco"))
                .args(Some("-y"))
                .status()
                .expect("Error installing minimal mate on debian 11");

            Command::new("systemctl")
                .args(Some("enable"))
                .args(Some("lightdm"))
                .args(Some("-f"))
                .status()
                .expect("Error enabling lightdm on startup");

            utils::remove_files_debian();

            Command::new("reboot")
                .status()
                .expect("Error restarting system");

        },

        "--install-debian-kdeplasma" => {

            utils::remove_debian();

            utils::utils_debian();

            Command::new("apt")
                .args(Some("install"))
                .args(Some("sddm"))
                .args(Some("kde-plasma-desktop"))
                .args(Some("plasma-nm"))
                .args(Some("plasma-workspace-wayland"))
                .args(Some("-y"))
                .status()
                .expect("Error installing minimal kde plasma on debian 11");

            Command::new("apt")
                .args(Some("install"))
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

            Command::new("systemctl")
                .args(Some("enable"))
                .args(Some("sddm"))
                .args(Some("-f"))
                .status()
                .expect("Error enabling sddm on startup");

            utils::remove_files_debian();

            Command::new("reboot")
                .status()
                .expect("Error restarting system");

        },


        "--install-fedora-lxde" => {
            
            lib::texto(texts::DNF,"/etc/dnf/dnf.conf","Dnf not installed");

            utils::remove_fedora();

            utils::utils_fedora();

            Command::new("dnf")
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
        
            Command::new("systemctl")
                .args(Some("enable"))
                .args(Some("lightdm"))
                .args(Some("-f"))
                .status()
                .expect("Error enabling lightdm on startup");
        
            Command::new("systemctl")
                .args(Some("set-default"))
                .args(Some("graphical.target"))
                .status()
                .expect("Error enabling graphical mode boot");
            
            Command::new("reboot")
                .status()
                .expect("Error restarting system");

        },

        "--install-fedora-lxqt" => {

            lib::texto(texts::DNF,"/etc/dnf/dnf.conf","Dnf not installed");

            utils::remove_fedora();

            utils::utils_fedora();

            Command::new("dnf")
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

            Command::new("systemctl")
                .args(Some("enable"))
                .args(Some("lightdm"))
                .args(Some("-f"))
                .status()
                .expect("Error enabling lightdm on startup");
        
            Command::new("systemctl")
                .args(Some("set-default"))
                .args(Some("graphical.target"))
                .status()
                .expect("Error enabling graphical mode boot");

            Command::new("reboot")
                .status()
                .expect("Error restarting system");

        },

        "--install-fedora-xfce" => {
            
            lib::texto(texts::DNF,"/etc/dnf/dnf.conf","Dnf not installed");

            utils::remove_fedora();

            utils::utils_fedora();

            Command::new("dnf")
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
        
            Command::new("systemctl")
                .args(Some("enable"))
                .args(Some("lightdm"))
                .args(Some("-f"))
                .status()
                .expect("Error enabling lightdm on startup");
        
            Command::new("systemctl")
                .args(Some("set-default"))
                .args(Some("graphical.target"))
                .status()
                .expect("Error enabling graphical mode boot");

            Command::new("reboot")
                .status()
                .expect("Error restarting system");
    
        },

        "--install-fedora-gnome" => {
            
            lib::texto(texts::DNF,"/etc/dnf/dnf.conf","Dnf not installed");

            utils::remove_fedora();

            utils::utils_fedora();

            Command::new("dnf")
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
        
            Command::new("systemctl")
                .args(Some("enable"))
                .args(Some("gdm"))
                .args(Some("-f"))
                .spawn()
                .expect("Error enabling gdm on startup");
        
            Command::new("systemctl")
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

            Command::new("reboot")
                .status()
                .expect("Error restarting system");

        },

        "--install-fedora-cinnamon" => {
            
            lib::texto(texts::DNF,"/etc/dnf/dnf.conf","Dnf not installed");

            utils::remove_fedora();

            utils::utils_fedora();

            Command::new("dnf")
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

            Command::new("systemctl")
                .args(Some("enable"))
                .args(Some("lightdm"))
                .args(Some("-f"))
                .status()
                .expect("Error enabling lightdm on startup");

            Command::new("systemctl")
                .args(Some("set-default"))
                .args(Some("graphical.target"))
                .status()
                .expect("Error enabling graphical mode boot");

            Command::new("reboot")
                .status()
                .expect("Error restarting system");

        },

        "--install-fedora-mate" => {
            
            lib::texto(texts::DNF,"/etc/dnf/dnf.conf","Dnf not installed");

            utils::remove_fedora();

            utils::utils_fedora();

            Command::new("dnf")
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
        
            Command::new("systemctl")
                .args(Some("enable"))
                .args(Some("lightdm"))
                .args(Some("-f"))
                .status()
                .expect("Error enabling lightdm on startup");
        
            Command::new("systemctl")
                .args(Some("set-default"))
                .args(Some("graphical.target"))
                .status()
                .expect("Error enabling graphical mode boot");

            Command::new("reboot")
                .status()
                .expect("Error restarting system");

        },
        
        "--install-fedora-kdeplasma" => {
            
            lib::texto(texts::DNF,"/etc/dnf/dnf.conf","Dnf not installed");

            utils::remove_fedora();

            utils::utils_fedora();

            Command::new("dnf")
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
        
            Command::new("systemctl")
                .args(Some("enable"))
                .args(Some("sddm"))
                .args(Some("-f"))
                .status()
                .expect("Error enabling sddm on startup");
        
            Command::new("systemctl")
                .args(Some("set-default"))
                .args(Some("graphical.target"))
                .status()
                .expect("Error enabling graphical mode boot");

            Command::new("reboot")
                .status()
                .expect("Error restarting system");

        },

        "--help" => {

            println!("{}",texts::HELP_EN);

        },

        _ => {

            println!("{}",texts::HELP_EN);

        },

    }

}
