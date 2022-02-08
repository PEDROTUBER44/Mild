use std::process::Command;
use std::io::Write;
use std::process;
use std::env;
use std::io;
mod texts;
mod utils;
mod lib;

fn main() {
    let args: Vec<String> = env::args().collect();
    let option = &args[1].trim();

    match &option[..] {

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

        "--install-arch-lxde" => {

            //Put here a system language checker with a structure match lang[..] { "pt_BR" => {...}, "en" => {...}, }
            //Also put here an option to remove configuration files from the home folder

            println!("The following packages will be removed from your system and after removal your system will be updated:");
            println!("");
            println!("lxde lightdm lightdm-gtk-greeter adwaita-icon-theme xarchiver lxqt xfce4-settings xfce4-pulseaudio-plugin exo garcon tumbler xfce4-panel xfce4-session xfce4-whiskermenu-plugin xfce4-terminal xfconf xfdesktop xfwm4 thunar file-roller gdm weston gnome-session gnome-terminal nautilus-terminal nautilus gnome-control-center gedit eog evince cinnamon cinnamon-session cinnamon-desktop gnome-terminal cinnamon-control-center cinnamon-menus cinnamon-screensaver cinnamon-settings-daemon cinnamon-translations cjs muffin nemo nemo-fileroller mate-control-center mate-desktop mate-power-manager mate-screensaver mate-common mate-session-manager mate-settings-daemon mate-terminal mate-panel marco caja sddm plasma-desktop plasma-nm konsole plasma-wayland-session kcm-fcitx kscreen ksysguard spectacle dolphin discover cutefish");
            println!("");
            println!("");
            println!("The following packages will be installed:");
            println!("");
            println!("lxde lightdm lightdm-gtk-greeter adwaita-icon-theme xarchiver");
            println!("");
            print!("Do you wish to continue? (Y/n): ");
            io::stdout().flush().unwrap();
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Error to read user input");

            match &input[..] {
                "y" => {
                    utils::remove_arch();
                    utils::utils_archlinux();
                    utils::install_arch_lxde();
                    utils::remove_files_archlinux();
                    Command::new("reboot").status().expect("Error restarting system");
                },

                "Y" => {
                    utils::remove_arch();
                    utils::utils_archlinux();
                    utils::install_arch_lxde();
                    utils::remove_files_archlinux();
                    Command::new("reboot").status().expect("Error restarting system");
                },

                "n" => {
                    println!("Aborting...");
                    process::exit(0x0100);
                },

                "N" => {
                    println!("Aborting...");
                    process::exit(0x0100);
                },

                "" => {
                    utils::remove_arch();
                    utils::utils_archlinux();
                    utils::install_arch_lxde();
                    utils::remove_files_archlinux();
                    Command::new("reboot").status().expect("Error restarting system");
                },

                _ => {
                    println!("Aborting...");
                    process::exit(0x0100);
                }
            }
        },

        "--install-arch-lxqt" => {

            //Put here a system language checker with a structure match lang[..] { "pt_BR" => {...}, "en" => {...}, }
            //Also put here an option to remove configuration files from the home folder

            println!("The following packages will be removed from your system and after removal your system will be updated:");
            println!("");
            println!("lxde lightdm lightdm-gtk-greeter adwaita-icon-theme xarchiver lxqt xfce4-settings xfce4-pulseaudio-plugin exo garcon tumbler xfce4-panel xfce4-session xfce4-whiskermenu-plugin xfce4-terminal xfconf xfdesktop xfwm4 thunar file-roller gdm weston gnome-session gnome-terminal nautilus-terminal nautilus gnome-control-center gedit eog evince cinnamon cinnamon-session cinnamon-desktop gnome-terminal cinnamon-control-center cinnamon-menus cinnamon-screensaver cinnamon-settings-daemon cinnamon-translations cjs muffin nemo nemo-fileroller mate-control-center mate-desktop mate-power-manager mate-screensaver mate-common mate-session-manager mate-settings-daemon mate-terminal mate-panel marco caja sddm plasma-desktop plasma-nm konsole plasma-wayland-session kcm-fcitx kscreen ksysguard spectacle dolphin discover cutefish");
            println!("");
            println!("");
            println!("The following packages will be installed:");
            println!("");
            println!("lxqt lightdm lightdm-gtk-greeter adwaita-icon-theme xarchiver");
            println!("");
            print!("Do you wish to continue? (Y/n): ");
            io::stdout().flush().unwrap();
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Error to read user input");

            match &input[..] {
                "y" => {
                    utils::remove_arch();
                    utils::utils_archlinux();
                    utils::install_arch_lxqt();
                    utils::remove_files_archlinux();
                    Command::new("reboot").status().expect("Error restarting system");
                },

                "Y" => {
                    utils::remove_arch();
                    utils::utils_archlinux();
                    utils::install_arch_lxqt();
                    utils::remove_files_archlinux();
                    Command::new("reboot").status().expect("Error restarting system");
                },

                "n" => {
                    println!("Aborting...");
                    process::exit(0x0100);
                },

                "N" => {
                    println!("Aborting...");
                    process::exit(0x0100);
                },

                "" => {
                    utils::remove_arch();
                    utils::utils_archlinux();
                    utils::install_arch_lxqt();
                    utils::remove_files_archlinux();
                    Command::new("reboot").status().expect("Error restarting system");
                },

                _ => {
                    println!("Aborting...");
                    process::exit(0x0100);
                }
            }
        },

        "--install-arch-xfce" => {

            //Put here a system language checker with a structure match lang[..] { "pt_BR" => {...}, "en" => {...}, }
            //Also put here an option to remove configuration files from the home folder

            println!("The following packages will be removed from your system and after removal your system will be updated:");
            println!("");
            println!("lxde lightdm lightdm-gtk-greeter adwaita-icon-theme xarchiver lxqt xfce4-settings xfce4-pulseaudio-plugin exo garcon tumbler xfce4-panel xfce4-session xfce4-whiskermenu-plugin xfce4-terminal xfconf xfdesktop xfwm4 thunar file-roller gdm weston gnome-session gnome-terminal nautilus-terminal nautilus gnome-control-center gedit eog evince cinnamon cinnamon-session cinnamon-desktop gnome-terminal cinnamon-control-center cinnamon-menus cinnamon-screensaver cinnamon-settings-daemon cinnamon-translations cjs muffin nemo nemo-fileroller mate-control-center mate-desktop mate-power-manager mate-screensaver mate-common mate-session-manager mate-settings-daemon mate-terminal mate-panel marco caja sddm plasma-desktop plasma-nm konsole plasma-wayland-session kcm-fcitx kscreen ksysguard spectacle dolphin discover cutefish");
            println!("");
            println!("");
            println!("The following packages will be installed:");
            println!("");
            println!("lightdm lightdm-gtk-greeter xfce4-settings xfce4-pulseaudio-plugin adwaita-icon-theme exo garcon tumbler xfce4-panel xfce4-session xfce4-whiskermenu-plugin xfce4-terminal xfconf xfdesktop xfwm4 thunar file-roller");
            println!("");
            print!("Do you wish to continue? (Y/n): ");
            io::stdout().flush().unwrap();
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Error to read user input");

            match &input[..] {
                "y" => {
                    utils::remove_arch();
                    utils::utils_archlinux();
                    utils::install_arch_xfce();
                    utils::remove_files_archlinux();
                    Command::new("reboot").status().expect("Error restarting system");
                },

                "Y" => {
                    utils::remove_arch();
                    utils::utils_archlinux();
                    utils::install_arch_xfce();
                    utils::remove_files_archlinux();
                    Command::new("reboot").status().expect("Error restarting system");
                },

                "n" => {
                    println!("Aborting...");
                    process::exit(0x0100);
                },

                "N" => {
                    println!("Aborting...");
                    process::exit(0x0100);
                },

                "" => {
                    utils::remove_arch();
                    utils::utils_archlinux();
                    utils::install_arch_xfce();
                    utils::remove_files_archlinux();
                    Command::new("reboot").status().expect("Error restarting system");
                },

                _ => {
                    println!("Aborting...");
                    process::exit(0x0100);
                }
            }
        },

        "--install-arch-gnome" => {

            //Put here a system language checker with a structure match lang[..] { "pt_BR" => {...}, "en" => {...}, }
            //Also put here an option to remove configuration files from the home folder

            println!("The following packages will be removed from your system and after removal your system will be updated:");
            println!("");
            println!("lxde lightdm lightdm-gtk-greeter adwaita-icon-theme xarchiver lxqt xfce4-settings xfce4-pulseaudio-plugin exo garcon tumbler xfce4-panel xfce4-session xfce4-whiskermenu-plugin xfce4-terminal xfconf xfdesktop xfwm4 thunar file-roller gdm weston gnome-session gnome-terminal nautilus-terminal nautilus gnome-control-center gedit eog evince cinnamon cinnamon-session cinnamon-desktop gnome-terminal cinnamon-control-center cinnamon-menus cinnamon-screensaver cinnamon-settings-daemon cinnamon-translations cjs muffin nemo nemo-fileroller mate-control-center mate-desktop mate-power-manager mate-screensaver mate-common mate-session-manager mate-settings-daemon mate-terminal mate-panel marco caja sddm plasma-desktop plasma-nm konsole plasma-wayland-session kcm-fcitx kscreen ksysguard spectacle dolphin discover cutefish");
            println!("");
            println!("");
            println!("The following packages will be installed:");
            println!("");
            println!("weston gnome-session gnome-terminal nautilus-terminal nautilus gnome-control-center gedit adwaita-icon-theme eog evince seahorse");
            println!("");
            print!("Do you wish to continue? (Y/n): ");
            io::stdout().flush().unwrap();
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Error to read user input");

            match &input[..] {
                "y" => {
                    utils::remove_arch();
                    utils::utils_archlinux();
                    utils::install_arch_gnome();
                    utils::remove_files_archlinux();
                    Command::new("reboot").status().expect("Error restarting system");
                },

                "Y" => {
                    utils::remove_arch();
                    utils::utils_archlinux();
                    utils::install_arch_gnome();
                    utils::remove_files_archlinux();
                    Command::new("reboot").status().expect("Error restarting system");
                },

                "n" => {
                    println!("Aborting...");
                    process::exit(0x0100);
                },

                "N" => {
                    println!("Aborting...");
                    process::exit(0x0100);
                },

                "" => {
                    utils::remove_arch();
                    utils::utils_archlinux();
                    utils::install_arch_gnome();
                    utils::remove_files_archlinux();
                    Command::new("reboot").status().expect("Error restarting system");
                },

                _ => {
                    println!("Aborting...");
                    process::exit(0x0100);
                }
            }
        },

        "--install-arch-cinnamon" => {

            //Put here a system language checker with a structure match lang[..] { "pt_BR" => {...}, "en" => {...}, }
            //Also put here an option to remove configuration files from the home folder

            println!("The following packages will be removed from your system and after removal your system will be updated:");
            println!("");
            println!("lxde lightdm lightdm-gtk-greeter adwaita-icon-theme xarchiver lxqt xfce4-settings xfce4-pulseaudio-plugin exo garcon tumbler xfce4-panel xfce4-session xfce4-whiskermenu-plugin xfce4-terminal xfconf xfdesktop xfwm4 thunar file-roller gdm weston gnome-session gnome-terminal nautilus-terminal nautilus gnome-control-center gedit eog evince cinnamon cinnamon-session cinnamon-desktop gnome-terminal cinnamon-control-center cinnamon-menus cinnamon-screensaver cinnamon-settings-daemon cinnamon-translations cjs muffin nemo nemo-fileroller mate-control-center mate-desktop mate-power-manager mate-screensaver mate-common mate-session-manager mate-settings-daemon mate-terminal mate-panel marco caja sddm plasma-desktop plasma-nm konsole plasma-wayland-session kcm-fcitx kscreen ksysguard spectacle dolphin discover cutefish");
            println!("");
            println!("");
            println!("The following packages will be installed:");
            println!("");
            println!("lightdm lightdm-gtk-greeter cinnamon cinnamon-session cinnamon-desktop gnome-terminal cinnamon-control-center cinnamon-menus cinnamon-screensaver cinnamon-settings-daemon cinnamon-translations adwaita-icon-theme cjs muffin nemo nemo-fileroller file-roller");
            println!("");
            print!("Do you wish to continue? (Y/n): ");
            io::stdout().flush().unwrap();
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Error to read user input");

            match &input[..] {
                "y" => {
                    utils::remove_arch();
                    utils::utils_archlinux();
                    utils::install_arch_cinnamon();
                    utils::remove_files_archlinux();
                    Command::new("reboot").status().expect("Error restarting system");
                },

                "Y" => {
                    utils::remove_arch();
                    utils::utils_archlinux();
                    utils::install_arch_cinnamon();
                    utils::remove_files_archlinux();
                    Command::new("reboot").status().expect("Error restarting system");
                },

                "n" => {
                    println!("Aborting...");
                    process::exit(0x0100);
                },

                "N" => {
                    println!("Aborting...");
                    process::exit(0x0100);
                },

                "" => {
                    utils::remove_arch();
                    utils::utils_archlinux();
                    utils::install_arch_cinnamon();
                    utils::remove_files_archlinux();
                    Command::new("reboot").status().expect("Error restarting system");
                },

                _ => {
                    println!("Aborting...");
                    process::exit(0x0100);
                }
            }
        },

        "--install-arch-mate" => {
                        
            //Put here a system language checker with a structure match lang[..] { "pt_BR" => {...}, "en" => {...}, }
            //Also put here an option to remove configuration files from the home folder

            println!("The following packages will be removed from your system and after removal your system will be updated:");
            println!("");
            println!("lxde lightdm lightdm-gtk-greeter adwaita-icon-theme xarchiver lxqt xfce4-settings xfce4-pulseaudio-plugin exo garcon tumbler xfce4-panel xfce4-session xfce4-whiskermenu-plugin xfce4-terminal xfconf xfdesktop xfwm4 thunar file-roller gdm weston gnome-session gnome-terminal nautilus-terminal nautilus gnome-control-center gedit eog evince cinnamon cinnamon-session cinnamon-desktop gnome-terminal cinnamon-control-center cinnamon-menus cinnamon-screensaver cinnamon-settings-daemon cinnamon-translations cjs muffin nemo nemo-fileroller mate-control-center mate-desktop mate-power-manager mate-screensaver mate-common mate-session-manager mate-settings-daemon mate-terminal mate-panel marco caja sddm plasma-desktop plasma-nm konsole plasma-wayland-session kcm-fcitx kscreen ksysguard spectacle dolphin discover cutefish");
            println!("");
            println!("");
            println!("The following packages will be installed:");
            println!("");
            println!("lightdm lightdm-gtk-greeter mate-control-center adwaita-icon-theme mate-desktop mate-power-manager mate-screensaver mate-common mate-session-manager mate-settings-daemon mate-terminal network-manager-applet mate-panel marco caja");
            println!("");
            print!("Do you wish to continue? (Y/n): ");
            io::stdout().flush().unwrap();
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Error to read user input");

            match &input[..] {
                "y" => {
                    utils::remove_arch();
                    utils::utils_archlinux();
                    utils::install_arch_mate();
                    utils::remove_files_archlinux();
                    Command::new("reboot").status().expect("Error restarting system");
                },

                "Y" => {
                    utils::remove_arch();
                    utils::utils_archlinux();
                    utils::install_arch_mate();
                    utils::remove_files_archlinux();
                    Command::new("reboot").status().expect("Error restarting system");
                },

                "n" => {
                    println!("Aborting...");
                    process::exit(0x0100);
                },

                "N" => {
                    println!("Aborting...");
                    process::exit(0x0100);
                },

                "" => {
                    utils::remove_arch();
                    utils::utils_archlinux();
                    utils::install_arch_mate();
                    utils::remove_files_archlinux();
                    Command::new("reboot").status().expect("Error restarting system");
                },

                _ => {
                    println!("Aborting...");
                    process::exit(0x0100);
                }
            }
        },

        "--install-arch-kdeplasma" => {
                                    
            //Put here a system language checker with a structure match lang[..] { "pt_BR" => {...}, "en" => {...}, }
            //Also put here an option to remove configuration files from the home folder

            println!("The following packages will be removed from your system and after removal your system will be updated:");
            println!("");
            println!("lxde lightdm lightdm-gtk-greeter adwaita-icon-theme xarchiver lxqt xfce4-settings xfce4-pulseaudio-plugin exo garcon tumbler xfce4-panel xfce4-session xfce4-whiskermenu-plugin xfce4-terminal xfconf xfdesktop xfwm4 thunar file-roller gdm weston gnome-session gnome-terminal nautilus-terminal nautilus gnome-control-center gedit eog evince cinnamon cinnamon-session cinnamon-desktop gnome-terminal cinnamon-control-center cinnamon-menus cinnamon-screensaver cinnamon-settings-daemon cinnamon-translations cjs muffin nemo nemo-fileroller mate-control-center mate-desktop mate-power-manager mate-screensaver mate-common mate-session-manager mate-settings-daemon mate-terminal mate-panel marco caja sddm plasma-desktop plasma-nm konsole plasma-wayland-session kcm-fcitx kscreen ksysguard spectacle dolphin discover cutefish");
            println!("");
            println!("");
            println!("The following packages will be installed:");
            println!("");
            println!("sddm plasma-desktop plasma-nm konsole plasma-wayland-session kcm-fcitx kscreen ksysguard adwaita-icon-theme spectacle dolphin discover");
            println!("");
            print!("Do you wish to continue? (Y/n): ");
            io::stdout().flush().unwrap();
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Error to read user input");

            match &input[..] {
                "y" => {
                    utils::remove_arch();
                    utils::utils_archlinux();
                    utils::install_arch_kdeplasma();
                    utils::remove_files_archlinux();
                    Command::new("reboot").status().expect("Error restarting system");
                },

                "Y" => {
                    utils::remove_arch();
                    utils::utils_archlinux();
                    utils::install_arch_kdeplasma();
                    utils::remove_files_archlinux();
                    Command::new("reboot").status().expect("Error restarting system");
                },

                "n" => {
                    println!("Aborting...");
                    process::exit(0x0100);
                },

                "N" => {
                    println!("Aborting...");
                    process::exit(0x0100);
                },

                "" => {
                    utils::remove_arch();
                    utils::utils_archlinux();
                    utils::install_arch_kdeplasma();
                    utils::remove_files_archlinux();
                    Command::new("reboot").status().expect("Error restarting system");
                },

                _ => {
                    println!("Aborting...");
                    process::exit(0x0100);
                }
            }
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

        "--install-debian-lxde" => {
                                    
            //Put here a system language checker with a structure match lang[..] { "pt_BR" => {...}, "en" => {...}, }
            //Also put here an option to remove configuration files from the home folder

            println!("The following packages will be removed from your system and after removal your system will be updated:");
            println!("");
            println!("plasma-workspace-wayland plasma-nm kde-plasma-desktop sddm konsole kscreen plasma-discover ksysguard okular kde-spectacle ark kwrite dolphin mate-desktop-environment-core marco cinnamon-core adwaita-icon-theme nautilus gnome-tweaks gnome-terminal gnome-control-center adwaita-qt qt5ct gdm3 gnome-session xfdesktop4 xfwm4 xfconf xfce4-terminal xfce4-settings xfce4-session xfce4-whiskermenu-plugin xfce4-pulseaudio-plugin xfce4-panel lightdm lxde-core lightdm-gtk-greeter lxterminal lxappearance pavucontrol lxsession-default-apps xscreensaver policykit-1 xarchiver pavucontrol lxqt-core thunar");
            println!("");
            println!("");
            println!("The following packages will be installed:");
            println!("");
            println!("lightdm lightdm-gtk-greeter lxde-core lxterminal lxappearance pavucontrol lxsession-default-apps xcreensaver policykit-1 xarchiver");
            println!("");
            print!("Do you wish to continue? (Y/n): ");
            io::stdout().flush().unwrap();
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Error to read user input");

            match &input[..] {
                "y" => {
                    utils::remove_debian();
                    utils::utils_debian();
                    utils::install_debian_lxde();
                    utils::remove_files_debian();
                    Command::new("reboot").status().expect("Error restarting system");
                },

                "Y" => {
                    utils::remove_debian();
                    utils::utils_debian();
                    utils::install_debian_lxde();
                    utils::remove_files_debian();
                    Command::new("reboot").status().expect("Error restarting system");
                },

                "n" => {
                    println!("Aborting...");
                    process::exit(0x0100);
                },

                "N" => {
                    println!("Aborting...");
                    process::exit(0x0100);
                },

                "" => {
                    utils::remove_debian();
                    utils::utils_debian();
                    utils::install_debian_lxde();
                    utils::remove_files_debian();
                    Command::new("reboot").status().expect("Error restarting system");
                },

                _ => {
                    println!("Aborting...");
                    process::exit(0x0100);
                }
            }
        },

        "--install-debian-lxqt" => {
                                                
            //Put here a system language checker with a structure match lang[..] { "pt_BR" => {...}, "en" => {...}, }
            //Also put here an option to remove configuration files from the home folder

            println!("The following packages will be removed from your system and after removal your system will be updated:");
            println!("");
            println!("plasma-workspace-wayland plasma-nm kde-plasma-desktop sddm konsole kscreen plasma-discover ksysguard okular kde-spectacle ark kwrite dolphin mate-desktop-environment-core marco cinnamon-core adwaita-icon-theme nautilus gnome-tweaks gnome-terminal gnome-control-center adwaita-qt qt5ct gdm3 gnome-session xfdesktop4 xfwm4 xfconf xfce4-terminal xfce4-settings xfce4-session xfce4-whiskermenu-plugin xfce4-pulseaudio-plugin xfce4-panel lightdm lxde-core lightdm-gtk-greeter lxterminal lxappearance pavucontrol lxsession-default-apps xscreensaver policykit-1 xarchiver pavucontrol lxqt-core thunar");
            println!("");
            println!("");
            println!("The following packages will be installed:");
            println!("");
            println!("lightdm lightdm-gtk-greeter lxqt-core pavucontrol");
            println!("");
            print!("Do you wish to continue? (Y/n): ");
            io::stdout().flush().unwrap();
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Error to read user input");

            match &input[..] {
                "y" => {
                    utils::remove_debian();
                    utils::utils_debian();
                    utils::install_debian_lxqt();
                    utils::remove_files_debian();
                    Command::new("reboot").status().expect("Error restarting system");
                },

                "Y" => {
                    utils::remove_debian();
                    utils::utils_debian();
                    utils::install_debian_lxqt();
                    utils::remove_files_debian();
                    Command::new("reboot").status().expect("Error restarting system");
                },

                "n" => {
                    println!("Aborting...");
                    process::exit(0x0100);
                },

                "N" => {
                    println!("Aborting...");
                    process::exit(0x0100);
                },

                "" => {
                    utils::remove_debian();
                    utils::utils_debian();
                    utils::install_debian_lxqt();
                    utils::remove_files_debian();
                    Command::new("reboot").status().expect("Error restarting system");
                },

                _ => {
                    println!("Aborting...");
                    process::exit(0x0100);
                }
            }
        },

        "--install-debian-xfce" => {
                                                
            //Put here a system language checker with a structure match lang[..] { "pt_BR" => {...}, "en" => {...}, }
            //Also put here an option to remove configuration files from the home folder

            println!("The following packages will be removed from your system and after removal your system will be updated:");
            println!("");
            println!("plasma-workspace-wayland plasma-nm kde-plasma-desktop sddm konsole kscreen plasma-discover ksysguard okular kde-spectacle ark kwrite dolphin mate-desktop-environment-core marco cinnamon-core adwaita-icon-theme nautilus gnome-tweaks gnome-terminal gnome-control-center adwaita-qt qt5ct gdm3 gnome-session xfdesktop4 xfwm4 xfconf xfce4-terminal xfce4-settings xfce4-session xfce4-whiskermenu-plugin xfce4-pulseaudio-plugin xfce4-panel lightdm lxde-core lightdm-gtk-greeter lxterminal lxappearance pavucontrol lxsession-default-apps xscreensaver policykit-1 xarchiver pavucontrol lxqt-core thunar");
            println!("");
            println!("");
            println!("The following packages will be installed:");
            println!("");
            println!("lightdm lightdm-gtk-greeter thunar xfce4-panel xfce4-pulseaudio-plugin xfce4-whiskermenu-plugin xfce4-session xfce4-settings xfce4-terminal pavucontrol xfdesktop4 xfwm4 adwaita-qt qt5ct xfconf");
            println!("");
            print!("Do you wish to continue? (Y/n): ");
            io::stdout().flush().unwrap();
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Error to read user input");

            match &input[..] {
                "y" => {
                    utils::remove_debian();
                    utils::utils_debian();
                    utils::install_debian_xfce();
                    utils::remove_files_debian();
                    Command::new("reboot").status().expect("Error restarting system");
                },

                "Y" => {
                    utils::remove_debian();
                    utils::utils_debian();
                    utils::install_debian_xfce();
                    utils::remove_files_debian();
                    Command::new("reboot").status().expect("Error restarting system");
                },

                "n" => {
                    println!("Aborting...");
                    process::exit(0x0100);
                },

                "N" => {
                    println!("Aborting...");
                    process::exit(0x0100);
                },

                "" => {
                    utils::remove_debian();
                    utils::utils_debian();
                    utils::install_debian_xfce();
                    utils::remove_files_debian();
                    Command::new("reboot").status().expect("Error restarting system");
                },

                _ => {
                    println!("Aborting...");
                    process::exit(0x0100);
                }
            }
        },

        "--install-debian-gnome" => {
                                                            
            //Put here a system language checker with a structure match lang[..] { "pt_BR" => {...}, "en" => {...}, }
            //Also put here an option to remove configuration files from the home folder

            println!("The following packages will be removed from your system and after removal your system will be updated:");
            println!("");
            println!("plasma-workspace-wayland plasma-nm kde-plasma-desktop sddm konsole kscreen plasma-discover ksysguard okular kde-spectacle ark kwrite dolphin mate-desktop-environment-core marco cinnamon-core adwaita-icon-theme nautilus gnome-tweaks gnome-terminal gnome-control-center adwaita-qt qt5ct gdm3 gnome-session xfdesktop4 xfwm4 xfconf xfce4-terminal xfce4-settings xfce4-session xfce4-whiskermenu-plugin xfce4-pulseaudio-plugin xfce4-panel lightdm lxde-core lightdm-gtk-greeter lxterminal lxappearance pavucontrol lxsession-default-apps xscreensaver policykit-1 xarchiver pavucontrol lxqt-core thunar");
            println!("");
            println!("");
            println!("The following packages will be installed:");
            println!("");
            println!("gdm3 gnome-session gnome-control-center gnome-terminal gnome-tweaks nautilus adwaita-icon-theme seahorse"); // Falta o eog, totem, gedit etc...
            println!("");
            print!("Do you wish to continue? (Y/n): ");
            io::stdout().flush().unwrap();
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Error to read user input");

            match &input[..] {
                "y" => {
                    utils::remove_debian();
                    utils::utils_debian();
                    utils::install_debian_gnome();
                    utils::remove_files_debian();
                    Command::new("reboot").status().expect("Error restarting system");
                },

                "Y" => {
                    utils::remove_debian();
                    utils::utils_debian();
                    utils::install_debian_gnome();
                    utils::remove_files_debian();
                    Command::new("reboot").status().expect("Error restarting system");
                },

                "n" => {
                    println!("Aborting...");
                    process::exit(0x0100);
                },

                "N" => {
                    println!("Aborting...");
                    process::exit(0x0100);
                },

                "" => {
                    utils::remove_debian();
                    utils::utils_debian();
                    utils::install_debian_gnome();
                    utils::remove_files_debian();
                    Command::new("reboot").status().expect("Error restarting system");
                },

                _ => {
                    println!("Aborting...");
                    process::exit(0x0100);
                }
            }
        },

        "--install-debian-cinnamon" => {
                                                            
            //Put here a system language checker with a structure match lang[..] { "pt_BR" => {...}, "en" => {...}, }
            //Also put here an option to remove configuration files from the home folder

            println!("The following packages will be removed from your system and after removal your system will be updated:");
            println!("");
            println!("plasma-workspace-wayland plasma-nm kde-plasma-desktop sddm konsole kscreen plasma-discover ksysguard okular kde-spectacle ark kwrite dolphin mate-desktop-environment-core marco cinnamon-core adwaita-icon-theme nautilus gnome-tweaks gnome-terminal gnome-control-center adwaita-qt qt5ct gdm3 gnome-session xfdesktop4 xfwm4 xfconf xfce4-terminal xfce4-settings xfce4-session xfce4-whiskermenu-plugin xfce4-pulseaudio-plugin xfce4-panel lightdm lxde-core lightdm-gtk-greeter lxterminal lxappearance pavucontrol lxsession-default-apps xscreensaver policykit-1 xarchiver pavucontrol lxqt-core thunar");
            println!("");
            println!("");
            println!("The following packages will be installed:");
            println!("");
            println!("cinnamon-core lightdm lightdm-gtk-greeter");
            println!("");
            print!("Do you wish to continue? (Y/n): ");
            io::stdout().flush().unwrap();
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Error to read user input");

            match &input[..] {
                "y" => {
                    utils::remove_debian();
                    utils::utils_debian();
                    utils::install_debian_cinnamon();
                    utils::remove_files_debian();
                    Command::new("reboot").status().expect("Error restarting system");
                },

                "Y" => {
                    utils::remove_debian();
                    utils::utils_debian();
                    utils::install_debian_cinnamon();
                    utils::remove_files_debian();
                    Command::new("reboot").status().expect("Error restarting system");
                },

                "n" => {
                    println!("Aborting...");
                    process::exit(0x0100);
                },

                "N" => {
                    println!("Aborting...");
                    process::exit(0x0100);
                },

                "" => {
                    utils::remove_debian();
                    utils::utils_debian();
                    utils::install_debian_cinnamon();
                    utils::remove_files_debian();
                    Command::new("reboot").status().expect("Error restarting system");
                },

                _ => {
                    println!("Aborting...");
                    process::exit(0x0100);
                }
            }
        },

        "--install-debian-mate" => {
                                                            
            //Put here a system language checker with a structure match lang[..] { "pt_BR" => {...}, "en" => {...}, }
            //Also put here an option to remove configuration files from the home folder

            println!("The following packages will be removed from your system and after removal your system will be updated:");
            println!("");
            println!("plasma-workspace-wayland plasma-nm kde-plasma-desktop sddm konsole kscreen plasma-discover ksysguard okular kde-spectacle ark kwrite dolphin mate-desktop-environment-core marco cinnamon-core adwaita-icon-theme nautilus gnome-tweaks gnome-terminal gnome-control-center adwaita-qt qt5ct gdm3 gnome-session xfdesktop4 xfwm4 xfconf xfce4-terminal xfce4-settings xfce4-session xfce4-whiskermenu-plugin xfce4-pulseaudio-plugin xfce4-panel lightdm lxde-core lightdm-gtk-greeter lxterminal lxappearance pavucontrol lxsession-default-apps xscreensaver policykit-1 xarchiver pavucontrol lxqt-core thunar");
            println!("");
            println!("");
            println!("The following packages will be installed:");
            println!("");
            println!("mate-desktop-environment-core lightdm lightdm-gtk-greeter marco");
            println!("");
            print!("Do you wish to continue? (Y/n): ");
            io::stdout().flush().unwrap();
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Error to read user input");

            match &input[..] {
                "y" => {
                    utils::remove_debian();
                    utils::utils_debian();
                    utils::install_debian_mate();
                    utils::remove_files_debian();
                    Command::new("reboot").status().expect("Error restarting system");
                },

                "Y" => {
                    utils::remove_debian();
                    utils::utils_debian();
                    utils::install_debian_mate();
                    utils::remove_files_debian();
                    Command::new("reboot").status().expect("Error restarting system");
                },

                "n" => {
                    println!("Aborting...");
                    process::exit(0x0100);
                },

                "N" => {
                    println!("Aborting...");
                    process::exit(0x0100);
                },

                "" => {
                    utils::remove_debian();
                    utils::utils_debian();
                    utils::install_debian_mate();
                    utils::remove_files_debian();
                    Command::new("reboot").status().expect("Error restarting system");
                },

                _ => {
                    println!("Aborting...");
                    process::exit(0x0100);
                }
            }
        },

        "--install-debian-kdeplasma" => {
                                                            
            //Put here a system language checker with a structure match lang[..] { "pt_BR" => {...}, "en" => {...}, }
            //Also put here an option to remove configuration files from the home folder

            println!("The following packages will be removed from your system and after removal your system will be updated:");
            println!("");
            println!("plasma-workspace-wayland plasma-nm kde-plasma-desktop sddm konsole kscreen plasma-discover ksysguard okular kde-spectacle ark kwrite dolphin mate-desktop-environment-core marco cinnamon-core adwaita-icon-theme nautilus gnome-tweaks gnome-terminal gnome-control-center adwaita-qt qt5ct gdm3 gnome-session xfdesktop4 xfwm4 xfconf xfce4-terminal xfce4-settings xfce4-session xfce4-whiskermenu-plugin xfce4-pulseaudio-plugin xfce4-panel lightdm lxde-core lightdm-gtk-greeter lxterminal lxappearance pavucontrol lxsession-default-apps xscreensaver policykit-1 xarchiver pavucontrol lxqt-core thunar");
            println!("");
            println!("");
            println!("The following packages will be installed:");
            println!("");
            println!("sddm kde-plasma-desktop plasma-nm plasma-workspace-wayland dolphin kwrite ark kde-spectacle okular ksysguard plasma-discover kscreen konsole");
            println!("");
            print!("Do you wish to continue? (Y/n): ");
            io::stdout().flush().unwrap();
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Error to read user input");

            match &input[..] {
                "y" => {
                    utils::remove_debian();
                    utils::utils_debian();
                    utils::install_debian_kdeplasma();
                    utils::remove_files_debian();
                    Command::new("reboot").status().expect("Error restarting system");
                },

                "Y" => {
                    utils::remove_debian();
                    utils::utils_debian();
                    utils::install_debian_kdeplasma();
                    utils::remove_files_debian();
                    Command::new("reboot").status().expect("Error restarting system");
                },

                "n" => {
                    println!("Aborting...");
                    process::exit(0x0100);
                },

                "N" => {
                    println!("Aborting...");
                    process::exit(0x0100);
                },

                "" => {
                    utils::remove_debian();
                    utils::utils_debian();
                    utils::install_debian_kdeplasma();
                    utils::remove_files_debian();
                    Command::new("reboot").status().expect("Error restarting system");
                },

                _ => {
                    println!("Aborting...");
                    process::exit(0x0100);
                }
            }
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

        "--install-fedora-lxde" => {
                                                                        
            //Put here a system language checker with a structure match lang[..] { "pt_BR" => {...}, "en" => {...}, }
            //Also put here an option to remove configuration files from the home folder
          
            println!("The following packages will be removed from your system and after removal your system will be updated:");
            println!("");
            println!("@lxde-desktop @plasma-desktop @gnome-desktop @cinnamon-desktop @lxqt-desktop @mate-desktop @xfce-desktop lxappearance lxde-common lxdm lxinput lxmenu-data lxpanel lxpolkit lxrandr xcompmgr xarchiver lxsession lxtask pcmanfm lxterminal network-manager-applet openbox obconf lightdm-gtk-greeter lightdm sddm plasma-desktop plasma-nm konsole kcm_colors kcm-fcitx kscreen ksysguard spectacle plasma-user-manager dolphin plasma-discover gdm gnome-shell nautilus gnome-terminal fedora-workstation-backgrounds file-roller gnome-terminal-nautilus cinnamon cinnamon-control-center cinnamon-desktop cinnamon-menus cinnamon-screensaver cinnamon-session nemo nemo-fileroller cinnamon-translations cjs muffin gnome-terminal breeze-cursor-theme breeze-gtk breeze-icon-theme firewall-config network-manager-applet notification-daemon obconf openbox pcmanfm-qt qterminal lxqt-about lxqt-archiver lxqt-config lxqt-notificationd lxqt-openssh-askpass lxqt-panel lxqt-policykit lxqt-powermanagement lxqt-qtplugin lxqt-session lxqt-themes lxqt-themes-fedora network-manager-applet xfwm4 xfce4-power-manager xfce4-session xfce4-settings xfce4-whiskermenu-plugin xfdesktop xfce4-terminal mate-control-center mate-desktop mate-power-manager mate-screensaver mate-screenshot mate-session-manager mate-settings-daemon mate-terminal network-manager-applet mate-panel marco caja");
            println!("");
            println!("");
            println!("The following packages will be installed:");
            println!("");
            println!("lxappearance lxde-common lxdm lxinput lxmenu-data lxpanel lxpolkit lxrandr xcompmgr xarchiver lxsession lxtask pcmanfm lxterminal network-manager-applet openbox obconf lightdm-gtk-greeter lightdm");
            println!("");
            print!("Do you wish to continue? (Y/n): ");
            io::stdout().flush().unwrap();
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Error to read user input");

            match &input[..] {
                "y" => {
                    lib::texto(texts::DNF,"/etc/dnf/dnf.conf","Dnf not installed");
                    utils::remove_fedora();
                    utils::utils_fedora();
                    utils::install_fedora_lxde();
                    Command::new("reboot").status().expect("Error restarting system");
                },

                "Y" => {
                    lib::texto(texts::DNF,"/etc/dnf/dnf.conf","Dnf not installed");
                    utils::remove_fedora();
                    utils::utils_fedora();
                    utils::install_fedora_lxde();
                    Command::new("reboot").status().expect("Error restarting system");
                },

                "n" => {
                    println!("Aborting...");
                    process::exit(0x0100);
                },

                "N" => {
                    println!("Aborting...");
                    process::exit(0x0100);
                },

                "" => {
                    lib::texto(texts::DNF,"/etc/dnf/dnf.conf","Dnf not installed");
                    utils::remove_fedora();
                    utils::utils_fedora();
                    utils::install_fedora_lxde();
                    Command::new("reboot").status().expect("Error restarting system");
                },

                _ => {
                    println!("Aborting...");
                    process::exit(0x0100);
                }
            }
        },

        "--install-fedora-lxqt" => {
                                                                                    
            //Put here a system language checker with a structure match lang[..] { "pt_BR" => {...}, "en" => {...}, }
            //Also put here an option to remove configuration files from the home folder
          
            println!("The following packages will be removed from your system and after removal your system will be updated:");
            println!("");
            println!("@lxde-desktop @plasma-desktop @gnome-desktop @cinnamon-desktop @lxqt-desktop @mate-desktop @xfce-desktop lxappearance lxde-common lxdm lxinput lxmenu-data lxpanel lxpolkit lxrandr xcompmgr xarchiver lxsession lxtask pcmanfm lxterminal network-manager-applet openbox obconf lightdm-gtk-greeter lightdm sddm plasma-desktop plasma-nm konsole kcm_colors kcm-fcitx kscreen ksysguard spectacle plasma-user-manager dolphin plasma-discover gdm gnome-shell nautilus gnome-terminal fedora-workstation-backgrounds file-roller gnome-terminal-nautilus cinnamon cinnamon-control-center cinnamon-desktop cinnamon-menus cinnamon-screensaver cinnamon-session nemo nemo-fileroller cinnamon-translations cjs muffin gnome-terminal breeze-cursor-theme breeze-gtk breeze-icon-theme firewall-config network-manager-applet notification-daemon obconf openbox pcmanfm-qt qterminal lxqt-about lxqt-archiver lxqt-config lxqt-notificationd lxqt-openssh-askpass lxqt-panel lxqt-policykit lxqt-powermanagement lxqt-qtplugin lxqt-session lxqt-themes lxqt-themes-fedora network-manager-applet xfwm4 xfce4-power-manager xfce4-session xfce4-settings xfce4-whiskermenu-plugin xfdesktop xfce4-terminal mate-control-center mate-desktop mate-power-manager mate-screensaver mate-screenshot mate-session-manager mate-settings-daemon mate-terminal network-manager-applet mate-panel marco caja");
            println!("");
            println!("");
            println!("The following packages will be installed:");
            println!("");
            println!("lightdm lightdm-gtk-greeter breeze-cursor-theme breeze-gtk breeze-icon-theme firewall-config network-manager-applet notification-daemon obconf openbox pcmanfm-qt qterminal lxqt-about lxqt-archiver lxqt-config lxqt-notificationd lxqt-openssh-askpass lxqt-panel lxqt-policykit lxqt-powermanagement lxqt-qtplugin lxqt-session lxqt-themes lxqt-themes-fedora");
            println!("");
            print!("Do you wish to continue? (Y/n): ");
            io::stdout().flush().unwrap();
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Error to read user input");

            match &input[..] {
                "y" => {
                    lib::texto(texts::DNF,"/etc/dnf/dnf.conf","Dnf not installed");
                    utils::remove_fedora();
                    utils::utils_fedora();
                    utils::install_fedora_lxqt();
                    Command::new("reboot").status().expect("Error restarting system");
                },

                "Y" => {
                    lib::texto(texts::DNF,"/etc/dnf/dnf.conf","Dnf not installed");
                    utils::remove_fedora();
                    utils::utils_fedora();
                    utils::install_fedora_lxqt();
                    Command::new("reboot").status().expect("Error restarting system");
                },

                "n" => {
                    println!("Aborting...");
                    process::exit(0x0100);
                },

                "N" => {
                    println!("Aborting...");
                    process::exit(0x0100);
                },

                "" => {
                    lib::texto(texts::DNF,"/etc/dnf/dnf.conf","Dnf not installed");
                    utils::remove_fedora();
                    utils::utils_fedora();
                    utils::install_fedora_lxqt();
                    Command::new("reboot").status().expect("Error restarting system");
                },

                _ => {
                    println!("Aborting...");
                    process::exit(0x0100);
                }
            }
        },

        "--install-fedora-xfce" => {
                                                                                    
            //Put here a system language checker with a structure match lang[..] { "pt_BR" => {...}, "en" => {...}, }
            //Also put here an option to remove configuration files from the home folder
          
            println!("The following packages will be removed from your system and after removal your system will be updated:");
            println!("");
            println!("@lxde-desktop @plasma-desktop @gnome-desktop @cinnamon-desktop @lxqt-desktop @mate-desktop @xfce-desktop lxappearance lxde-common lxdm lxinput lxmenu-data lxpanel lxpolkit lxrandr xcompmgr xarchiver lxsession lxtask pcmanfm lxterminal network-manager-applet openbox obconf lightdm-gtk-greeter lightdm sddm plasma-desktop plasma-nm konsole kcm_colors kcm-fcitx kscreen ksysguard spectacle plasma-user-manager dolphin plasma-discover gdm gnome-shell nautilus gnome-terminal fedora-workstation-backgrounds file-roller gnome-terminal-nautilus cinnamon cinnamon-control-center cinnamon-desktop cinnamon-menus cinnamon-screensaver cinnamon-session nemo nemo-fileroller cinnamon-translations cjs muffin gnome-terminal breeze-cursor-theme breeze-gtk breeze-icon-theme firewall-config network-manager-applet notification-daemon obconf openbox pcmanfm-qt qterminal lxqt-about lxqt-archiver lxqt-config lxqt-notificationd lxqt-openssh-askpass lxqt-panel lxqt-policykit lxqt-powermanagement lxqt-qtplugin lxqt-session lxqt-themes lxqt-themes-fedora network-manager-applet xfwm4 xfce4-power-manager xfce4-session xfce4-settings xfce4-whiskermenu-plugin xfdesktop xfce4-terminal mate-control-center mate-desktop mate-power-manager mate-screensaver mate-screenshot mate-session-manager mate-settings-daemon mate-terminal network-manager-applet mate-panel marco caja");
            println!("");
            println!("");
            println!("The following packages will be installed:");
            println!("");
            println!("network-manager-applet xfwm4 xfce4-power-manager xfce4-session xfce4-settings xfce4-whiskermenu-plugin xfdesktop lightdm lightdm-gtk-greeter xfce4-terminal");
            println!("");
            print!("Do you wish to continue? (Y/n): ");
            io::stdout().flush().unwrap();
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Error to read user input");

            match &input[..] {
                "y" => {
                    lib::texto(texts::DNF,"/etc/dnf/dnf.conf","Dnf not installed");
                    utils::remove_fedora();
                    utils::utils_fedora();
                    utils::install_fedora_xfce();
                    Command::new("reboot").status().expect("Error restarting system");
                },

                "Y" => {
                    lib::texto(texts::DNF,"/etc/dnf/dnf.conf","Dnf not installed");
                    utils::remove_fedora();
                    utils::utils_fedora();
                    utils::install_fedora_xfce();
                    Command::new("reboot").status().expect("Error restarting system");
                },

                "n" => {
                    println!("Aborting...");
                    process::exit(0x0100);
                },

                "N" => {
                    println!("Aborting...");
                    process::exit(0x0100);
                },

                "" => {
                    lib::texto(texts::DNF,"/etc/dnf/dnf.conf","Dnf not installed");
                    utils::remove_fedora();
                    utils::utils_fedora();
                    utils::install_fedora_xfce();
                    Command::new("reboot").status().expect("Error restarting system");
                },

                _ => {
                    println!("Aborting...");
                    process::exit(0x0100);
                }
            }
        },

        "--install-fedora-gnome" => {
                                                                                    
            //Put here a system language checker with a structure match lang[..] { "pt_BR" => {...}, "en" => {...}, }
            //Also put here an option to remove configuration files from the home folder
          
            println!("The following packages will be removed from your system and after removal your system will be updated:");
            println!("");
            println!("@lxde-desktop @plasma-desktop @gnome-desktop @cinnamon-desktop @lxqt-desktop @mate-desktop @xfce-desktop lxappearance lxde-common lxdm lxinput lxmenu-data lxpanel lxpolkit lxrandr xcompmgr xarchiver lxsession lxtask pcmanfm lxterminal network-manager-applet openbox obconf lightdm-gtk-greeter lightdm sddm plasma-desktop plasma-nm konsole kcm_colors kcm-fcitx kscreen ksysguard spectacle plasma-user-manager dolphin plasma-discover gdm gnome-shell nautilus gnome-terminal fedora-workstation-backgrounds file-roller gnome-terminal-nautilus cinnamon cinnamon-control-center cinnamon-desktop cinnamon-menus cinnamon-screensaver cinnamon-session nemo nemo-fileroller cinnamon-translations cjs muffin gnome-terminal breeze-cursor-theme breeze-gtk breeze-icon-theme firewall-config network-manager-applet notification-daemon obconf openbox pcmanfm-qt qterminal lxqt-about lxqt-archiver lxqt-config lxqt-notificationd lxqt-openssh-askpass lxqt-panel lxqt-policykit lxqt-powermanagement lxqt-qtplugin lxqt-session lxqt-themes lxqt-themes-fedora network-manager-applet xfwm4 xfce4-power-manager xfce4-session xfce4-settings xfce4-whiskermenu-plugin xfdesktop xfce4-terminal mate-control-center mate-desktop mate-power-manager mate-screensaver mate-screenshot mate-session-manager mate-settings-daemon mate-terminal network-manager-applet mate-panel marco caja");
            println!("");
            println!("");
            println!("The following packages will be installed:");
            println!("");
            println!("gdm gnome-shell nautilus gnome-terminal fedora-workstation-backgrounds file-roller gnome-terminal-nautilus seahorse");
            println!("");
            print!("Do you wish to continue? (Y/n): ");
            io::stdout().flush().unwrap();
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Error to read user input");

            match &input[..] {
                "y" => {
                    lib::texto(texts::DNF,"/etc/dnf/dnf.conf","Dnf not installed");
                    utils::remove_fedora();
                    utils::utils_fedora();
                    utils::install_fedora_gnome();
                    Command::new("reboot").status().expect("Error restarting system");
                },

                "Y" => {
                    lib::texto(texts::DNF,"/etc/dnf/dnf.conf","Dnf not installed");
                    utils::remove_fedora();
                    utils::utils_fedora();
                    utils::install_fedora_gnome();
                    Command::new("reboot").status().expect("Error restarting system");
                },

                "n" => {
                    println!("Aborting...");
                    process::exit(0x0100);
                },

                "N" => {
                    println!("Aborting...");
                    process::exit(0x0100);
                },

                "" => {
                    lib::texto(texts::DNF,"/etc/dnf/dnf.conf","Dnf not installed");
                    utils::remove_fedora();
                    utils::utils_fedora();
                    utils::install_fedora_gnome();
                    Command::new("reboot").status().expect("Error restarting system");
                },

                _ => {
                    println!("Aborting...");
                    process::exit(0x0100);
                }
            }
        },

        "--install-fedora-cinnamon" => {
                                                                                    
            //Put here a system language checker with a structure match lang[..] { "pt_BR" => {...}, "en" => {...}, }
            //Also put here an option to remove configuration files from the home folder
          
            println!("The following packages will be removed from your system and after removal your system will be updated:");
            println!("");
            println!("@lxde-desktop @plasma-desktop @gnome-desktop @cinnamon-desktop @lxqt-desktop @mate-desktop @xfce-desktop lxappearance lxde-common lxdm lxinput lxmenu-data lxpanel lxpolkit lxrandr xcompmgr xarchiver lxsession lxtask pcmanfm lxterminal network-manager-applet openbox obconf lightdm-gtk-greeter lightdm sddm plasma-desktop plasma-nm konsole kcm_colors kcm-fcitx kscreen ksysguard spectacle plasma-user-manager dolphin plasma-discover gdm gnome-shell nautilus gnome-terminal fedora-workstation-backgrounds file-roller gnome-terminal-nautilus cinnamon cinnamon-control-center cinnamon-desktop cinnamon-menus cinnamon-screensaver cinnamon-session nemo nemo-fileroller cinnamon-translations cjs muffin gnome-terminal breeze-cursor-theme breeze-gtk breeze-icon-theme firewall-config network-manager-applet notification-daemon obconf openbox pcmanfm-qt qterminal lxqt-about lxqt-archiver lxqt-config lxqt-notificationd lxqt-openssh-askpass lxqt-panel lxqt-policykit lxqt-powermanagement lxqt-qtplugin lxqt-session lxqt-themes lxqt-themes-fedora network-manager-applet xfwm4 xfce4-power-manager xfce4-session xfce4-settings xfce4-whiskermenu-plugin xfdesktop xfce4-terminal mate-control-center mate-desktop mate-power-manager mate-screensaver mate-screenshot mate-session-manager mate-settings-daemon mate-terminal network-manager-applet mate-panel marco caja");
            println!("");
            println!("");
            println!("The following packages will be installed:");
            println!("");
            println!("cinnamon cinnamon-control-center cinnamon-desktop cinnamon-menus cinnamon-screensaver cinnamon-session nemo nemo-fileroller cinnamon-translations cjs muffin gnome-terminal lightdm lightdm-gtk-greeter");
            println!("");
            print!("Do you wish to continue? (Y/n): ");
            io::stdout().flush().unwrap();
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Error to read user input");

            match &input[..] {
                "y" => {
                    lib::texto(texts::DNF,"/etc/dnf/dnf.conf","Dnf not installed");
                    utils::remove_fedora();
                    utils::utils_fedora();
                    utils::install_fedora_cinnamon();
                    Command::new("reboot").status().expect("Error restarting system");
                },

                "Y" => {
                    lib::texto(texts::DNF,"/etc/dnf/dnf.conf","Dnf not installed");
                    utils::remove_fedora();
                    utils::utils_fedora();
                    utils::install_fedora_cinnamon();
                    Command::new("reboot").status().expect("Error restarting system");
                },

                "n" => {
                    println!("Aborting...");
                    process::exit(0x0100);
                },

                "N" => {
                    println!("Aborting...");
                    process::exit(0x0100);
                },

                "" => {
                    lib::texto(texts::DNF,"/etc/dnf/dnf.conf","Dnf not installed");
                    utils::remove_fedora();
                    utils::utils_fedora();
                    utils::install_fedora_cinnamon();
                    Command::new("reboot").status().expect("Error restarting system");
                },

                _ => {
                    println!("Aborting...");
                    process::exit(0x0100);
                }
            }
        },

        "--install-fedora-mate" => {
                                                                                                
            //Put here a system language checker with a structure match lang[..] { "pt_BR" => {...}, "en" => {...}, }
            //Also put here an option to remove configuration files from the home folder
          
            println!("The following packages will be removed from your system and after removal your system will be updated:");
            println!("");
            println!("@lxde-desktop @plasma-desktop @gnome-desktop @cinnamon-desktop @lxqt-desktop @mate-desktop @xfce-desktop lxappearance lxde-common lxdm lxinput lxmenu-data lxpanel lxpolkit lxrandr xcompmgr xarchiver lxsession lxtask pcmanfm lxterminal network-manager-applet openbox obconf lightdm-gtk-greeter lightdm sddm plasma-desktop plasma-nm konsole kcm_colors kcm-fcitx kscreen ksysguard spectacle plasma-user-manager dolphin plasma-discover gdm gnome-shell nautilus gnome-terminal fedora-workstation-backgrounds file-roller gnome-terminal-nautilus cinnamon cinnamon-control-center cinnamon-desktop cinnamon-menus cinnamon-screensaver cinnamon-session nemo nemo-fileroller cinnamon-translations cjs muffin gnome-terminal breeze-cursor-theme breeze-gtk breeze-icon-theme firewall-config network-manager-applet notification-daemon obconf openbox pcmanfm-qt qterminal lxqt-about lxqt-archiver lxqt-config lxqt-notificationd lxqt-openssh-askpass lxqt-panel lxqt-policykit lxqt-powermanagement lxqt-qtplugin lxqt-session lxqt-themes lxqt-themes-fedora network-manager-applet xfwm4 xfce4-power-manager xfce4-session xfce4-settings xfce4-whiskermenu-plugin xfdesktop xfce4-terminal mate-control-center mate-desktop mate-power-manager mate-screensaver mate-screenshot mate-session-manager mate-settings-daemon mate-terminal network-manager-applet mate-panel marco caja");
            println!("");
            println!("");
            println!("The following packages will be installed:");
            println!("");
            println!("lightdm lightdm-gtk-greeter mate-control-center mate-desktop mate-power-manager mate-screensaver mate-screenshot mate-session-manager mate-settings-daemon mate-terminal network-manager-applet mate-panel marco caja");
            println!("");
            print!("Do you wish to continue? (Y/n): ");
            io::stdout().flush().unwrap();
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Error to read user input");

            match &input[..] {
                "y" => {
                    lib::texto(texts::DNF,"/etc/dnf/dnf.conf","Dnf not installed");
                    utils::remove_fedora();
                    utils::utils_fedora();
                    utils::install_fedora_mate();
                    Command::new("reboot").status().expect("Error restarting system");
                },

                "Y" => {
                    lib::texto(texts::DNF,"/etc/dnf/dnf.conf","Dnf not installed");
                    utils::remove_fedora();
                    utils::utils_fedora();
                    utils::install_fedora_mate();
                    Command::new("reboot").status().expect("Error restarting system");
                },

                "n" => {
                    println!("Aborting...");
                    process::exit(0x0100);
                },

                "N" => {
                    println!("Aborting...");
                    process::exit(0x0100);
                },

                "" => {
                    lib::texto(texts::DNF,"/etc/dnf/dnf.conf","Dnf not installed");
                    utils::remove_fedora();
                    utils::utils_fedora();
                    utils::install_fedora_mate();
                    Command::new("reboot").status().expect("Error restarting system");
                },

                _ => {
                    println!("Aborting...");
                    process::exit(0x0100);
                }
            }
        },

        "--install-fedora-kdeplasma" => {
                                                                                                            
            //Put here a system language checker with a structure match lang[..] { "pt_BR" => {...}, "en" => {...}, }
            //Also put here an option to remove configuration files from the home folder
          
            println!("The following packages will be removed from your system and after removal your system will be updated:");
            println!("");
            println!("@lxde-desktop @plasma-desktop @gnome-desktop @cinnamon-desktop @lxqt-desktop @mate-desktop @xfce-desktop lxappearance lxde-common lxdm lxinput lxmenu-data lxpanel lxpolkit lxrandr xcompmgr xarchiver lxsession lxtask pcmanfm lxterminal network-manager-applet openbox obconf lightdm-gtk-greeter lightdm sddm plasma-desktop plasma-nm konsole kcm_colors kcm-fcitx kscreen ksysguard spectacle plasma-user-manager dolphin plasma-discover gdm gnome-shell nautilus gnome-terminal fedora-workstation-backgrounds file-roller gnome-terminal-nautilus cinnamon cinnamon-control-center cinnamon-desktop cinnamon-menus cinnamon-screensaver cinnamon-session nemo nemo-fileroller cinnamon-translations cjs muffin gnome-terminal breeze-cursor-theme breeze-gtk breeze-icon-theme firewall-config network-manager-applet notification-daemon obconf openbox pcmanfm-qt qterminal lxqt-about lxqt-archiver lxqt-config lxqt-notificationd lxqt-openssh-askpass lxqt-panel lxqt-policykit lxqt-powermanagement lxqt-qtplugin lxqt-session lxqt-themes lxqt-themes-fedora network-manager-applet xfwm4 xfce4-power-manager xfce4-session xfce4-settings xfce4-whiskermenu-plugin xfdesktop xfce4-terminal mate-control-center mate-desktop mate-power-manager mate-screensaver mate-screenshot mate-session-manager mate-settings-daemon mate-terminal network-manager-applet mate-panel marco caja");
            println!("");
            println!("");
            println!("The following packages will be installed:");
            println!("");
            println!("sddm plasma-desktop plasma-nm konsole kcm_colors kcm-fcitx kscreen ksysguard spectacle plasma-user-manager dolphin plasma-discover");
            println!("");
            print!("Do you wish to continue? (Y/n): ");
            io::stdout().flush().unwrap();
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Error to read user input");

            match &input[..] {
                "y" => {
                    lib::texto(texts::DNF,"/etc/dnf/dnf.conf","Dnf not installed");
                    utils::remove_fedora();
                    utils::utils_fedora();
                    utils::install_fedora_kdeplasma();
                    Command::new("reboot").status().expect("Error restarting system");
                },

                "Y" => {
                    lib::texto(texts::DNF,"/etc/dnf/dnf.conf","Dnf not installed");
                    utils::remove_fedora();
                    utils::utils_fedora();
                    utils::install_fedora_kdeplasma();
                    Command::new("reboot").status().expect("Error restarting system");
                },

                "n" => {
                    println!("Aborting...");
                    process::exit(0x0100);
                },

                "N" => {
                    println!("Aborting...");
                    process::exit(0x0100);
                },

                "" => {
                    lib::texto(texts::DNF,"/etc/dnf/dnf.conf","Dnf not installed");
                    utils::remove_fedora();
                    utils::utils_fedora();
                    utils::install_fedora_kdeplasma();
                    Command::new("reboot").status().expect("Error restarting system");
                },

                _ => {
                    println!("Aborting...");
                    process::exit(0x0100);
                }
            }
        },

        "--help" => {

            println!("{}",texts::HELP_EN);

        },

        _ => {

            println!("{}",texts::HELP_EN);

        }
    }
}