use std::process::Command;
use std::env;
mod texts;
mod lib;
mod utils;

const DNF : &str = "[main]
gpgcheck=1
installonly_limit=3
clean_requirements_on_remove=True
best=False
skip_if_unavailable=True
fastestmirror=True
max_parallel_downloads=7
defaultyes=True
";

fn main() {

    let args: Vec<String> = env::args().collect();
    let opcao = &args[1].trim();

    match &opcao[..] {

        "--install-arch-lxde" => {

            utils::utils_archlinux();

            Command::new("pacman")
                .args(Some("-Sy"))
                .args(Some("lxde"))
                .args(Some("lightdm"))
                .args(Some("lightdm-gtk-greeter"))
                .args(Some("xarchiver"))
                .args(Some("--noconfirm"))
                .status()
                .expect("Erro ao instalar o lxde no archlinux");
        
            Command::new("systemctl")
                .args(Some("enable"))
                .args(Some("lightdm"))
                .args(Some("-f"))
                .status()
                .expect("Erro ao habilitar o lightdm na inicializacao");

            Command::new("reboot")
                .status()
                .expect("Erro ao reiniciar o sistema");
    
        },

        "--install-arch-lxqt" => {

            utils::utils_archlinux();

            Command::new("pacman")
                .args(Some("-Sy"))
                .args(Some("lxqt"))
                .args(Some("lightdm"))
                .args(Some("lightdm-gtk-greeter"))
                .args(Some("xarchiver"))
                .args(Some("--noconfirm"))
                .status()
                .expect("Erro ao instalar o lxqt no archlinux");
        
            Command::new("systemctl")
                .args(Some("enable"))
                .args(Some("lightdm"))
                .args(Some("-f"))
                .status()
                .expect("Erro ao habilitar o lightdm na inicializacao");

            Command::new("reboot")
                .status()
                .expect("Erro ao reiniciar o sistema");
    
        },
        
        "--install-arch-xfce" => {

            utils::utils_archlinux();

            Command::new("pacman")
                .args(Some("-Sy"))
                .args(Some("lightdm"))
                .args(Some("lightdm-gtk-greeter"))
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
                .args(Some("--noconfirm"))
                .status()
                .expect("Erro ao instalar o xfce no archlinux");
        
            Command::new("systemctl")
                .args(Some("enable"))
                .args(Some("lightdm"))
                .args(Some("-f"))
                .status()
                .expect("Erro ao habilitar o lightdm na inicializacao");

            Command::new("reboot")
                .status()
                .expect("Erro ao reiniciar o sistema");

        },

        "--install-arch-gnome" => {

            utils::utils_archlinux();

            Command::new("pacman")
                .args(Some("-Sy"))
                .args(Some("gdm"))
                .args(Some("weston"))
                .args(Some("gnome-session"))
                .args(Some("gnome-terminal"))
                .args(Some("nautilus-terminal"))
                .args(Some("nautilus"))
                .args(Some("file-roller"))
                .args(Some("gnome-control-center"))
                .args(Some("--noconfirm"))
                .status()
                .expect("Erro ao instalar o gnome no archlinux");
        
            Command::new("systemctl")
                .args(Some("enable"))
                .args(Some("gdm"))
                .args(Some("-f"))
                .status()
                .expect("Erro ao habilitar o gdm na inicializacao");

            Command::new("reboot")
                .status()
                .expect("Erro ao reiniciar o sistema");

        },

        "--install-arch-cinnamon" => {

            utils::utils_archlinux();

            Command::new("pacman")
                .args(Some("-Sy"))
                .args(Some("lightdm"))
                .args(Some("lightdm-gtk-greeter"))
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
                .args(Some("file-roller"))
                .args(Some("--noconfirm"))
                .status()
                .expect("Erro ao instalar o cinnamon no archlinux");

            Command::new("systemctl")
                .args(Some("enable"))
                .args(Some("lightdm"))
                .args(Some("-f"))
                .status()
                .expect("Erro ao habilitar o lightdm na inicializacao");

            Command::new("reboot")
                .status()
                .expect("Erro ao reiniciar o sistema");

        },

        "--install-arch-mate" => {

            Command::new("pacman")
                .args(Some("-Sy"))
                .args(Some("lightdm"))
                .args(Some("lightdm-gtk-greeter"))
                .args(Some("mate-control-center"))
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
                .expect("Erro ao instalar o mate desktop minimal no archlinux");

            Command::new("systemctl")
                .args(Some("enable"))
                .args(Some("lightdm"))
                .args(Some("-f"))
                .status()
                .expect("Erro ao habilitar o lightdm na inicializacao");

            Command::new("reboot")
                .status()
                .expect("Erro ao reiniciar o sistema");

        },

        "--install-arch-kdeplasma" => {

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
                .args(Some("spectacle"))
                .args(Some("dolphin"))
                .args(Some("discover"))
                .args(Some("--noconfirm"))
                .status()
                .expect("Erro ao instalar o cinnamon no archlinux");

            Command::new("systemctl")
                .args(Some("enable"))
                .args(Some("sddm"))
                .args(Some("-f"))
                .status()
                .expect("Erro ao habilitar o lightdm na inicializacao");

            Command::new("reboot")
                .status()
                .expect("Erro ao reiniciar o sistema");

        },

        
        "--install-debian-lxde" => {

            utils::utils_debian();

            Command::new(" apt")
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
                .expect("Erro ao instalar o lxde minimal no debian 11");
        
            Command::new("systemctl")
                .args(Some("enable"))
                .args(Some("lightdm"))
                .args(Some("-f"))
                .status()
                .expect("Erro ao habilitar o lightdm na inicializacao");

            Command::new("reboot")
                .status()
                .expect("Erro ao reiniciar o sistema");

        },

        "--install-debian-lxqt" => {

            utils::utils_debian();

            Command::new("apt")
                .args(Some("install"))
                .args(Some("lxqt-core"))
                .args(Some("lightdm"))
                .args(Some("lightdm-gtk-greeter"))
                .args(Some("pavucontrol"))
                .args(Some("-y"))
                .status()
                .expect("Erro ao instalar o lxqt minimal no debian 11");

            Command::new("systemctl")
                .args(Some("enable"))
                .args(Some("lightdm"))
                .args(Some("-f"))
                .status()
                .expect("Erro ao habilitar o lightdm na inicializacao");
            
            Command::new("reboot")
                .status()
                .expect("Erro ao reiniciar o sistema");

        },

        "--install-debian-xfce" => {

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
                .expect("Erro ao instalar o xfce4 minimal no debian 11");

            Command::new("systemctl")
                .args(Some("enable"))
                .args(Some("lightdm"))
                .args(Some("-f"))
                .status()
                .expect("Erro ao habilitar o lightdm na inicializacao");

            Command::new("reboot")
                .status()
                .expect("Erro ao reiniciar o sistema");

        },

        "--install-debian-gnome" => {

            utils::utils_debian();

            Command::new("apt")
                .args(Some("install"))
                .args(Some("gdm3"))
                .args(Some("gnome-session"))
                .args(Some("gnome-control-center"))
                .args(Some("gnome-terminal"))
                .args(Some("gnome-tweaks"))
                .args(Some("nautilus"))
                .args(Some("adwaita-icon-theme-full"))
                .args(Some("--no-install-recommends"))
                .args(Some("-y"))
                .status()
                .expect("Erro ao instalar o gnome minimal no debian 11");

            Command::new("systemctl")
                .args(Some("enable"))
                .args(Some("gdm3"))
                .args(Some("-f"))
                .status()
                .expect("Erro ao habilitar o gdm3 na inicializacao");

            Command::new("reboot")
                .status()
                .expect("Erro ao reiniciar o sistema");

        },

        "--install-debian-cinnamon" => {

            utils::utils_debian();

            Command::new("apt")
                .args(Some("install"))
                .args(Some("cinnamon-core"))
                .args(Some("lightdm"))
                .args(Some("lightdm-gtk-greeter"))
                .args(Some("-y"))
                .status()
                .expect("Erro ao instalar o cinnamon minimal no debian 11");

            Command::new("systemctl")
                .args(Some("enable"))
                .args(Some("lightdm"))
                .args(Some("-f"))
                .status()
                .expect("Erro ao habilitar o lightdm na inicializacao");
            
            Command::new("reboot")
                .status()
                .expect("Erro ao reiniciar o sistema");

        },

        "--install-debian-mate" => {

            utils::utils_debian();

            Command::new("apt")
                .args(Some("install"))
                .args(Some("mate-desktop-environment-core"))
                .args(Some("lightdm"))
                .args(Some("lightdm-gtk-greeter"))
                .args(Some("marco"))
                .args(Some("-y"))
                .status()
                .expect("Erro ao instalar o mate minimal no debian 11");

            Command::new("systemctl")
                .args(Some("enable"))
                .args(Some("lightdm"))
                .args(Some("-f"))
                .status()
                .expect("Erro ao habilitar o lightdm na inicializacao");
        
            Command::new("reboot")
                .status()
                .expect("Erro ao reiniciar o sistema");

        },

        "--install-debian-kdeplasma" => {

            utils::utils_debian();

            Command::new("apt")
                .args(Some("install"))
                .args(Some("sddm"))
                .args(Some("kde-plasma-desktop"))
                .args(Some("plasma-nm"))
                .args(Some("-y"))
                .status()
                .expect("Erro ao instalar o kde plasma minimal no debian 11");

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
                .expect("Erro ao instalar o kde plasma minimal no debian 11");

            Command::new("systemctl")
                .args(Some("enable"))
                .args(Some("sddm"))
                .args(Some("-f"))
                .status()
                .expect("Erro ao habilitar o lightdm na inicializacao");

            Command::new("reboot")
                .status()
                .expect("Erro ao reiniciar o sistema");

        },



        "--install-fedora-lxde" => {
            
            lib::texto(DNF,"/etc/dnf/dnf.conf","Dnf não instalado",);

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
                .expect("Erro ao instalar o lxde minimal no fedora 34");
        
            Command::new("systemctl")
                .args(Some("enable"))
                .args(Some("lightdm"))
                .args(Some("-f"))
                .status()
                .expect("Erro ao habilitar o lightdm na inicializacao");
        
            Command::new("systemctl")
                .args(Some("set-default"))
                .args(Some("graphical.target"))
                .status()
                .expect("Erro ao habilitar a inicializacao em modo grafico");
            
            Command::new("reboot")
                .status()
                .expect("Erro ao reiniciar o sistema");

        },

        "--install-fedora-lxqt" => {

            lib::texto(DNF,"/etc/dnf/dnf.conf","Dnf não instalado");

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
                .expect("Erro ao instalar o lxqt minimal no fedora 34");

            Command::new("systemctl")
                .args(Some("enable"))
                .args(Some("lightdm"))
                .args(Some("-f"))
                .status()
                .expect("Erro ao habilitar o lightdm na inicializacao");
        
            Command::new("systemctl")
                .args(Some("set-default"))
                .args(Some("graphical.target"))
                .status()
                .expect("Erro ao habilitar a inicializacao em modo grafico");

            Command::new("reboot")
                .status()
                .expect("Erro ao reiniciar o sistema");

        },

        "--install-fedora-xfce" => {
            
            lib::texto(DNF,"/etc/dnf/dnf.conf","Dnf não instalado");

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
                .expect("Erro ao instalar o xfce4 minimal no fedora 34");
        
            Command::new("systemctl")
                .args(Some("enable"))
                .args(Some("lightdm"))
                .args(Some("-f"))
                .status()
                .expect("Erro ao habilitar o lightdm na inicializacao");
        
            Command::new("systemctl")
                .args(Some("set-default"))
                .args(Some("graphical.target"))
                .status()
                .expect("Erro ao habilitar a inicializacao em modo grafico");

            Command::new("reboot")
                .status()
                .expect("Erro ao reiniciar o sistema");
    
        },

        "--install-fedora-gnome" => {
            
            lib::texto(DNF,"/etc/dnf/dnf.conf","Dnf não instalado");

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
                .args(Some("-y"))
                .status()
                .expect("Erro ao instalar o gnome no fedora 34");
        
            Command::new("systemctl")
                .args(Some("enable"))
                .args(Some("gdm"))
                .args(Some("-f"))
                .spawn()
                .expect("Erro ao habilitar o gdm na inicializacao");
        
            Command::new("systemctl")
                .args(Some("set-default"))
                .args(Some("graphical.target"))
                .status()
                .expect("Erro ao habilitar a inicializacao em modo grafico");

            Command::new("reboot")
                .status()
                .expect("Erro ao reiniciar o sistema");

        },

        "--install-fedora-cinnamon" => {
            
            lib::texto(DNF,"/etc/dnf/dnf.conf","Dnf não instalado");

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
                .expect("Erro ao instalar o cinnamon no fedora");

            Command::new("systemctl")
                .args(Some("enable"))
                .args(Some("lightdm"))
                .args(Some("-f"))
                .status()
                .expect("Erro ao habilitar o lightdm na inicializacao");

            Command::new("systemctl")
                .args(Some("set-default"))
                .args(Some("graphical.target"))
                .status()
                .expect("Erro ao habilitar a inicializacao em modo grafico");

            Command::new("reboot")
                .status()
                .expect("Erro ao reiniciar o sistema");

        },

        "--install-fedora-mate" => {
            
            lib::texto(DNF,"/etc/dnf/dnf.conf","Dnf não instalado");

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
                .expect("Erro ao instalar o mate no fedora");
        
            Command::new("systemctl")
                .args(Some("enable"))
                .args(Some("lightdm"))
                .args(Some("-f"))
                .status()
                .expect("Erro ao habilitar o lightdm na inicializacao");
        
            Command::new("systemctl")
                .args(Some("set-default"))
                .args(Some("graphical.target"))
                .status()
                .expect("Erro ao habilitar a inicializacao em modo grafico");

            Command::new("reboot")
                .status()
                .expect("Erro ao reiniciar o sistema");

        },
        
        "--install-fedora-kdeplasma" => {
            
            lib::texto(DNF,"/etc/dnf/dnf.conf","Dnf não instalado");

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
                .expect("Erro ao instalar o Kde Plasma no fedora");
        
            Command::new("systemctl")
                .args(Some("enable"))
                .args(Some("sddm"))
                .args(Some("-f"))
                .status()
                .expect("Erro ao habilitar o sddm na inicializacao");
        
            Command::new("systemctl")
                .args(Some("set-default"))
                .args(Some("graphical.target"))
                .status()
                .expect("Erro ao habilitar a inicializacao em modo grafico");

            Command::new("reboot")
                .status()
                .expect("Erro ao reiniciar o sistema");

        },

        "--help" => {
            println!("{}",texts::HELP);
        },

        _ => {
            println!("{}",texts::HELP);
        },

    }

}