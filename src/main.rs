use std::{
    process::Command, // Import Command to execute system commands
    process::exit, // Import Exit Command to exit to program
    io::Write, // Import Write Command to write files
    env, // Import Env to read arguments from user 
    io // Import Standard IO - Input Output
};
mod texts;
mod utils;
mod lib;

fn main() {
    
    let args: Vec<String> = env::args().collect();
    let option = &args[1].trim();

    let all_packages_to_remove_archlinux = "lxde lightdm lightdm-gtk-greeter adwaita-icon-theme xarchiver lxqt xfce4-settings xfce4-pulseaudio-plugin exo garcon tumbler xfce4-panel xfce4-session xfce4-whiskermenu-plugin xfce4-terminal xfconf xfdesktop xfwm4 thunar file-roller gdm weston gnome-session gnome-terminal nautilus-terminal nautilus gnome-control-center gnome-screenshot gedit eog evince cinnamon cinnamon-session cinnamon-desktop gnome-terminal cinnamon-control-center cinnamon-menus cinnamon-screensaver cinnamon-settings-daemon cinnamon-translations cjs muffin nemo nemo-fileroller mate-control-center mate-desktop mate-power-manager mate-screensaver mate-common mate-session-manager mate-settings-daemon mate-terminal mate-panel marco caja sddm plasma-desktop plasma-nm konsole plasma-wayland-session kcm-fcitx kscreen ksysguard spectacle dolphin discover cutefish";
    let all_packages_to_remove_debian = "plasma-workspace-wayland plasma-nm kde-plasma-desktop sddm konsole kscreen plasma-discover ksysguard okular kde-spectacle ark kwrite dolphin mate-desktop-environment-core marco cinnamon-core adwaita-icon-theme nautilus gnome-tweaks gnome-terminal gnome-control-center adwaita-qt qt5ct gdm3 gnome-session xfdesktop4 xfwm4 xfconf xfce4-terminal xfce4-settings xfce4-session xfce4-whiskermenu-plugin xfce4-pulseaudio-plugin xfce4-panel lightdm lxde-core lightdm-gtk-greeter lxterminal lxappearance pavucontrol lxsession-default-apps xscreensaver policykit-1 xarchiver pavucontrol lxqt-core thunar";
    let all_packages_to_remove_fedora = "@lxde-desktop @plasma-desktop @gnome-desktop @cinnamon-desktop @lxqt-desktop @mate-desktop @xfce-desktop lxappearance lxde-common lxdm lxinput lxmenu-data lxpanel lxpolkit lxrandr xcompmgr xarchiver lxsession lxtask pcmanfm lxterminal network-manager-applet openbox obconf lightdm-gtk-greeter lightdm sddm plasma-desktop plasma-nm konsole kcm_colors kcm-fcitx kscreen ksysguard spectacle plasma-user-manager dolphin plasma-discover gdm gnome-shell nautilus gnome-terminal fedora-workstation-backgrounds file-roller gnome-terminal-nautilus cinnamon cinnamon-control-center cinnamon-desktop cinnamon-menus cinnamon-screensaver cinnamon-session nemo nemo-fileroller cinnamon-translations cjs muffin gnome-terminal breeze-cursor-theme breeze-gtk breeze-icon-theme firewall-config network-manager-applet notification-daemon obconf openbox pcmanfm-qt qterminal lxqt-about lxqt-archiver lxqt-config lxqt-notificationd lxqt-openssh-askpass lxqt-panel lxqt-policykit lxqt-powermanagement lxqt-qtplugin lxqt-session lxqt-themes lxqt-themes-fedora network-manager-applet xfwm4 xfce4-power-manager xfce4-session xfce4-settings xfce4-whiskermenu-plugin xfdesktop xfce4-terminal mate-control-center mate-desktop mate-power-manager mate-screensaver mate-screenshot mate-session-manager mate-settings-daemon mate-terminal network-manager-applet mate-panel marco caja gstreamer1-plugins-base gstreamer1-plugins-good gstreamer1-plugins-ugly gstreamer1-plugins-bad-free gstreamer1-plugins-bad-free gstreamer1-plugins-bad-freeworld gstreamer1-plugins-bad-free-extras ffmpeg";

    match &option[..] {

        "--install-arch-lxde" => {

            println!("The following packages will be removed from your system and after removal your system will be updated:");
            println!("");
            println!("{}",all_packages_to_remove_archlinux);
            println!("");
            println!("");
            println!("The following packages will be installed:");
            println!("");
            println!("lxde lightdm lightdm-gtk-greeter adwaita-icon-theme xarchiver");
            println!("");
            println!("");
            print!("Do you wish to continue? (Y/n): ");

            io::stdout().flush().unwrap();
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Error to read user input");

            fn exec_installation() {

                utils::remove_arch();
                utils::utils_archlinux();
                utils::install_arch_lxde();
                utils::remove_files_archlinux();
                Command::new("sudo").args(Some("reboot")).status().expect("Error to restarting system");

            }

            match &input[..] {

                "y" => exec_installation(),
                "Y" => exec_installation(),
                "" => exec_installation(),
                
                "n" => {

                    println!("Aborted installation");
                    exit(0);

                },
                "N" => {

                    println!("Aborted installation");
                    exit(0);

                },
                _ => {

                    println!("Aborted installation");
                    exit(0);
                    
                }

            }

        },

        "--install-arch-lxqt" => {

            println!("The following packages will be removed from your system and after removal your system will be updated:");
            println!("");
            println!("{}",all_packages_to_remove_archlinux);
            println!("");
            println!("");
            println!("The following packages will be installed:");
            println!("");
            println!("lxqt lightdm lightdm-gtk-greeter adwaita-icon-theme xarchiver");
            println!("");
            println!("");
            print!("Do you wish to continue? (Y/n): ");

            io::stdout().flush().unwrap();
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Error to read user input");

            fn exec_installation() {

                utils::remove_arch();
                utils::utils_archlinux();
                utils::install_arch_lxqt();
                utils::remove_files_archlinux();
                Command::new("sudo").args(Some("reboot")).status().expect("Error to restarting system");

            }

            match &input[..] {

                "y" => exec_installation(),
                "Y" => exec_installation(),
                "" => exec_installation(),
                
                "n" => {

                    println!("Aborted installation");
                    exit(0);

                },
                "N" => {

                    println!("Aborted installation");
                    exit(0);

                },
                _ => {

                    println!("Aborted installation");
                    exit(0);
                    
                }

            }

        },

        "--install-arch-xfce" => {

            println!("The following packages will be removed from your system and after removal your system will be updated:");
            println!("");
            println!("{}",all_packages_to_remove_archlinux);
            println!("");
            println!("");
            println!("The following packages will be installed:");
            println!("");
            println!("lightdm lightdm-gtk-greeter xfce4-settings xfce4-pulseaudio-plugin adwaita-icon-theme exo garcon tumbler xfce4-panel xfce4-session xfce4-whiskermenu-plugin xfce4-terminal xfconf xfdesktop xfwm4 thunar file-roller");
            println!("");
            println!("");
            print!("Do you wish to continue? (Y/n): ");

            io::stdout().flush().unwrap();
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Error to read user input");

            fn exec_installation() {

                utils::remove_arch();
                utils::utils_archlinux();
                utils::install_arch_xfce();
                utils::remove_files_archlinux();
                Command::new("sudo").args(Some("reboot")).status().expect("Error to restarting system");

            }

            match &input[..] {

                "y" => exec_installation(),
                "Y" => exec_installation(),
                "" => exec_installation(),
                
                "n" => {

                    println!("Aborted installation");
                    exit(0);

                },
                "N" => {

                    println!("Aborted installation");
                    exit(0);

                },
                _ => {

                    println!("Aborted installation");
                    exit(0);
                    
                }

            }

        },

        "--install-arch-gnome" => {

            println!("The following packages will be removed from your system and after removal your system will be updated:");
            println!("");
            println!("{}",all_packages_to_remove_archlinux);
            println!("");
            println!("");
            println!("The following packages will be installed:");
            println!("");
            println!("weston gnome-session gnome-terminal nautilus-terminal nautilus gnome-control-center gedit adwaita-icon-theme eog evince seahorse gnome-screenshot");
            println!("");
            println!("");
            print!("Do you wish to continue? (Y/n): ");

            io::stdout().flush().unwrap();
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Error to read user input");

            fn exec_installation() {

                utils::remove_arch();
                utils::utils_archlinux();
                utils::install_arch_gnome();
                utils::remove_files_archlinux();
                Command::new("sudo").args(Some("reboot")).status().expect("Error to restarting system");

            }

            match &input[..] {

                "y" => exec_installation(),
                "Y" => exec_installation(),
                "" => exec_installation(),
                
                "n" => {

                    println!("Aborted installation");
                    exit(0);

                },
                "N" => {

                    println!("Aborted installation");
                    exit(0);

                },
                _ => {

                    println!("Aborted installation");
                    exit(0);
                    
                }

            }

        },

        "--install-arch-cinnamon" => {

            println!("The following packages will be removed from your system and after removal your system will be updated:");
            println!("");
            println!("{}",all_packages_to_remove_archlinux);
            println!("");
            println!("");
            println!("The following packages will be installed:");
            println!("");
            println!("lightdm lightdm-gtk-greeter cinnamon cinnamon-session cinnamon-desktop gnome-terminal cinnamon-control-center cinnamon-menus cinnamon-screensaver cinnamon-settings-daemon cinnamon-translations adwaita-icon-theme cjs muffin nemo nemo-fileroller file-roller");
            println!("");
            println!("");
            print!("Do you wish to continue? (Y/n): ");

            io::stdout().flush().unwrap();
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Error to read user input");

            fn exec_installation() {

                utils::remove_arch();
                utils::utils_archlinux();
                utils::install_arch_cinnamon();
                utils::remove_files_archlinux();
                Command::new("sudo").args(Some("reboot")).status().expect("Error to restarting system");

            }

            match &input[..] {

                "y" => exec_installation(),
                "Y" => exec_installation(),
                "" => exec_installation(),
                
                "n" => {

                    println!("Aborted installation");
                    exit(0);

                },
                "N" => {

                    println!("Aborted installation");
                    exit(0);

                },
                _ => {

                    println!("Aborted installation");
                    exit(0);
                    
                }

            }

        },

        "--install-arch-mate" => {

            println!("The following packages will be removed from your system and after removal your system will be updated:");
            println!("");
            println!("{}",all_packages_to_remove_archlinux);
            println!("");
            println!("");
            println!("The following packages will be installed:");
            println!("");
            println!("lightdm lightdm-gtk-greeter mate-control-center adwaita-icon-theme mate-desktop mate-power-manager mate-screensaver mate-common mate-session-manager mate-settings-daemon mate-terminal network-manager-applet mate-panel marco caja");
            println!("");
            println!("");
            print!("Do you wish to continue? (Y/n): ");

            io::stdout().flush().unwrap();
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Error to read user input");

            fn exec_installation() {

                utils::remove_arch();
                utils::utils_archlinux();
                utils::install_arch_mate();
                utils::remove_files_archlinux();
                Command::new("sudo").args(Some("reboot")).status().expect("Error to restarting system");

            }

            match &input[..] {

                "y" => exec_installation(),
                "Y" => exec_installation(),
                "" => exec_installation(),
                
                "n" => {

                    println!("Aborted installation");
                    exit(0);

                },
                "N" => {

                    println!("Aborted installation");
                    exit(0);

                },
                _ => {

                    println!("Aborted installation");
                    exit(0);
                    
                }

            }

        },

        "--install-arch-kdeplasma" => {

            println!("The following packages will be removed from your system and after removal your system will be updated:");
            println!("");
            println!("{}",all_packages_to_remove_archlinux);
            println!("");
            println!("");
            println!("The following packages will be installed:");
            println!("");
            println!("sddm plasma-desktop plasma-nm konsole plasma-wayland-session kcm-fcitx kscreen ksysguard adwaita-icon-theme spectacle dolphin discover");
            println!("");
            println!("");
            print!("Do you wish to continue? (Y/n): ");

            io::stdout().flush().unwrap();
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Error to read user input");

            fn exec_installation() {

                utils::remove_arch();
                utils::utils_archlinux();
                utils::install_arch_kdeplasma();
                utils::remove_files_archlinux();
                Command::new("sudo").args(Some("reboot")).status().expect("Error to restarting system");

            }

            match &input[..] {

                "y" => exec_installation(),
                "Y" => exec_installation(),
                "" => exec_installation(),
                
                "n" => {

                    println!("Aborted installation");
                    exit(0);

                },
                "N" => {

                    println!("Aborted installation");
                    exit(0);

                },
                _ => {

                    println!("Aborted installation");
                    exit(0);
                    
                }

            }

        },

        "--install-arch-bspwm" => {

            println!("The following packages will be removed from your system and after removal your system will be updated:");
            println!("");
            println!("{}",all_packages_to_remove_archlinux);
            println!("");
            println!("");
            println!("The following packages will be installed:");
            println!("");
            println!(""); // Place the packages to be installed
            println!("");
            println!("");
            print!("Do you wish to continue? (Y/n): ");

            io::stdout().flush().unwrap();
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Error to read user input");

            fn exec_installation() {

                utils::remove_arch();
                utils::utils_archlinux();
                utils::install_arch_bspwm();
                utils::remove_files_archlinux();
                Command::new("sudo").args(Some("reboot")).status().expect("Error to restarting system");

            }

            match &input[..] {

                "y" => exec_installation(),
                "Y" => exec_installation(),
                "" => exec_installation(),
                
                "n" => {

                    println!("Aborted installation");
                    exit(0);

                },
                "N" => {

                    println!("Aborted installation");
                    exit(0);

                },
                _ => {

                    println!("Aborted installation");
                    exit(0);
                    
                }

            }

        },

        "--install-arch-cutefish" => {

            println!("The following packages will be removed from your system and after removal your system will be updated:");
            println!("");
            println!("{}",all_packages_to_remove_archlinux);
            println!("");
            println!("");
            println!("The following packages will be installed:");
            println!("");
            println!(""); // Place the packages to be installed
            println!("");
            println!("");
            print!("Do you wish to continue? (Y/n): ");

            io::stdout().flush().unwrap();
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Error to read user input");

            fn exec_installation() {

                utils::remove_arch();
                utils::utils_archlinux();
                utils::install_arch_cutefish();
                utils::remove_files_archlinux();
                Command::new("sudo").args(Some("reboot")).status().expect("Error to restarting system");

            }

            match &input[..] {

                "y" => exec_installation(),
                "Y" => exec_installation(),
                "" => exec_installation(),
                
                "n" => {

                    println!("Aborted installation");
                    exit(0);

                },
                "N" => {

                    println!("Aborted installation");
                    exit(0);

                },
                _ => {

                    println!("Aborted installation");
                    exit(0);
                    
                }

            }

        },

        "--clean-arch" => utils::clean_arch(),


        "--install-debian-lxde" => {

            println!("The following packages will be removed from your system and after removal your system will be updated:");
            println!("");
            println!("{}",all_packages_to_remove_debian);
            println!("");
            println!("");
            println!("The following packages will be installed:");
            println!("");
            println!("lightdm lightdm-gtk-greeter lxde-core lxterminal lxappearance pavucontrol lxsession-default-apps xcreensaver policykit-1 xarchiver");
            println!("");
            println!("");
            print!("Do you wish to continue? (Y/n): ");

            io::stdout().flush().unwrap();
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Error to read user input");

            fn exec_installation() {

                utils::remove_debian();
                utils::utils_debian();
                utils::install_debian_lxde();
                utils::remove_files_debian();
                Command::new("sudo").args(Some("reboot")).status().expect("Error to restarting system");

            }

            match &input[..] {

                "y" => exec_installation(),
                "Y" => exec_installation(),
                "" => exec_installation(),
                
                "n" => {

                    println!("Aborted installation");
                    exit(0);

                },
                "N" => {

                    println!("Aborted installation");
                    exit(0);

                },
                _ => {

                    println!("Aborted installation");
                    exit(0);
                    
                }

            }

        },

        "--install-debian-lxqt" => {

            println!("The following packages will be removed from your system and after removal your system will be updated:");
            println!("");
            println!("{}",all_packages_to_remove_debian);
            println!("");
            println!("");
            println!("The following packages will be installed:");
            println!("");
            println!("lightdm lightdm-gtk-greeter lxqt-core pavucontrol");
            println!("");
            println!("");
            print!("Do you wish to continue? (Y/n): ");

            io::stdout().flush().unwrap();
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Error to read user input");

            fn exec_installation() {

                utils::remove_debian();
                utils::utils_debian();
                utils::install_debian_lxqt();
                utils::remove_files_debian();
                Command::new("sudo").args(Some("reboot")).status().expect("Error to restarting system");

            }

            match &input[..] {

                "y" => exec_installation(),
                "Y" => exec_installation(),
                "" => exec_installation(),
                
                "n" => {

                    println!("Aborted installation");
                    exit(0);

                },
                "N" => {

                    println!("Aborted installation");
                    exit(0);

                },
                _ => {

                    println!("Aborted installation");
                    exit(0);
                    
                }

            }

        },

        "--install-debian-xfce" => {

            println!("The following packages will be removed from your system and after removal your system will be updated:");
            println!("");
            println!("{}",all_packages_to_remove_debian);
            println!("");
            println!("");
            println!("The following packages will be installed:");
            println!("");
            println!("lightdm lightdm-gtk-greeter thunar xfce4-panel xfce4-pulseaudio-plugin xfce4-whiskermenu-plugin xfce4-session xfce4-settings xfce4-terminal pavucontrol xfdesktop4 xfwm4 adwaita-qt qt5ct xfconf");
            println!("");
            println!("");
            print!("Do you wish to continue? (Y/n): ");

            io::stdout().flush().unwrap();
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Error to read user input");

            fn exec_installation() {

                utils::remove_debian();
                utils::utils_debian();
                utils::install_debian_xfce();
                utils::remove_files_debian();
                Command::new("sudo").args(Some("reboot")).status().expect("Error to restarting system");

            }

            match &input[..] {

                "y" => exec_installation(),
                "Y" => exec_installation(),
                "" => exec_installation(),
                
                "n" => {

                    println!("Aborted installation");
                    exit(0);

                },
                "N" => {

                    println!("Aborted installation");
                    exit(0);

                },
                _ => {

                    println!("Aborted installation");
                    exit(0);
                    
                }

            }

        },

        "--install-debian-gnome" => {

            println!("The following packages will be removed from your system and after removal your system will be updated:");
            println!("");
            println!("{}",all_packages_to_remove_debian);
            println!("");
            println!("");
            println!("The following packages will be installed:");
            println!("");
            println!("gdm3 gnome-session gnome-control-center gnome-terminal gnome-tweaks nautilus adwaita-icon-theme seahorse");
            println!("");
            println!("");
            print!("Do you wish to continue? (Y/n): ");

            io::stdout().flush().unwrap();
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Error to read user input");

            fn exec_installation() {

                utils::remove_debian();
                utils::utils_debian();
                utils::install_debian_gnome();
                utils::remove_files_debian();
                Command::new("sudo").args(Some("reboot")).status().expect("Error to restarting system");

            }

            match &input[..] {

                "y" => exec_installation(),
                "Y" => exec_installation(),
                "" => exec_installation(),
                
                "n" => {

                    println!("Aborted installation");
                    exit(0);

                },
                "N" => {

                    println!("Aborted installation");
                    exit(0);

                },
                _ => {

                    println!("Aborted installation");
                    exit(0);
                    
                }

            }

        },

        "--install-debian-cinnamon" => {

            println!("The following packages will be removed from your system and after removal your system will be updated:");
            println!("");
            println!("{}",all_packages_to_remove_debian);
            println!("");
            println!("");
            println!("The following packages will be installed:");
            println!("");
            println!("cinnamon-core lightdm lightdm-gtk-greeter");
            println!("");
            println!("");
            print!("Do you wish to continue? (Y/n): ");

            io::stdout().flush().unwrap();
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Error to read user input");

            fn exec_installation() {

                utils::remove_debian();
                utils::utils_debian();
                utils::install_debian_cinnamon();
                utils::remove_files_debian();
                Command::new("sudo").args(Some("reboot")).status().expect("Error to restarting system");

            }

            match &input[..] {

                "y" => exec_installation(),
                "Y" => exec_installation(),
                "" => exec_installation(),
                
                "n" => {

                    println!("Aborted installation");
                    exit(0);

                },
                "N" => {

                    println!("Aborted installation");
                    exit(0);

                },
                _ => {

                    println!("Aborted installation");
                    exit(0);
                    
                }

            }

        },

        "--install-debian-mate" => {

            println!("The following packages will be removed from your system and after removal your system will be updated:");
            println!("");
            println!("{}",all_packages_to_remove_debian);
            println!("");
            println!("");
            println!("The following packages will be installed:");
            println!("");
            println!("mate-desktop-environment-core lightdm lightdm-gtk-greeter marco");
            println!("");
            println!("");
            print!("Do you wish to continue? (Y/n): ");

            io::stdout().flush().unwrap();
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Error to read user input");

            fn exec_installation() {

                utils::remove_debian();
                utils::utils_debian();
                utils::install_debian_mate();
                utils::remove_files_debian();
                Command::new("sudo").args(Some("reboot")).status().expect("Error to restarting system");

            }

            match &input[..] {

                "y" => exec_installation(),
                "Y" => exec_installation(),
                "" => exec_installation(),
                
                "n" => {

                    println!("Aborted installation");
                    exit(0);

                },
                "N" => {

                    println!("Aborted installation");
                    exit(0);

                },
                _ => {

                    println!("Aborted installation");
                    exit(0);
                    
                }

            }

        },

        "--install-debian-kdeplasma" => {

            println!("The following packages will be removed from your system and after removal your system will be updated:");
            println!("");
            println!("{}",all_packages_to_remove_debian);
            println!("");
            println!("");
            println!("The following packages will be installed:");
            println!("");
            println!("sddm kde-plasma-desktop plasma-nm plasma-workspace-wayland dolphin kwrite ark kde-spectacle okular ksysguard plasma-discover kscreen konsole");
            println!("");
            println!("");
            print!("Do you wish to continue? (Y/n): ");

            io::stdout().flush().unwrap();
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Error to read user input");

            fn exec_installation() {

                utils::remove_debian();
                utils::utils_debian();
                utils::install_debian_kdeplasma();
                utils::remove_files_debian();
                Command::new("sudo").args(Some("reboot")).status().expect("Error to restarting system");

            }

            match &input[..] {

                "y" => exec_installation(),
                "Y" => exec_installation(),
                "" => exec_installation(),
                
                "n" => {

                    println!("Aborted installation");
                    exit(0);

                },
                "N" => {

                    println!("Aborted installation");
                    exit(0);

                },
                _ => {

                    println!("Aborted installation");
                    exit(0);
                    
                }

            }

        },

        "--install-debian-bspwm" => {

            println!("The following packages will be removed from your system and after removal your system will be updated:");
            println!("");
            println!("{}",all_packages_to_remove_debian);
            println!("");
            println!("");
            println!("The following packages will be installed:");
            println!("");
            println!(""); // Place the packages to be installed
            println!("");
            println!("");
            print!("Do you wish to continue? (Y/n): ");

            io::stdout().flush().unwrap();
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Error to read user input");

            fn exec_installation() {

                utils::remove_debian();
                utils::utils_debian();
                utils::install_debian_bspwm();
                utils::remove_files_debian();
                Command::new("sudo").args(Some("reboot")).status().expect("Error to restarting system");

            }

            match &input[..] {

                "y" => exec_installation(),
                "Y" => exec_installation(),
                "" => exec_installation(),
                
                "n" => {

                    println!("Aborted installation");
                    exit(0);

                },
                "N" => {

                    println!("Aborted installation");
                    exit(0);

                },
                _ => {

                    println!("Aborted installation");
                    exit(0);
                    
                }

            }

        },

        "--install-debian-cutefish" => {

            println!("The following packages will be removed from your system and after removal your system will be updated:");
            println!("");
            println!("{}",all_packages_to_remove_debian);
            println!("");
            println!("");
            println!("The following packages will be installed:");
            println!("");
            println!(""); // Place the packages to be installed
            println!("");
            println!("");
            print!("Do you wish to continue? (Y/n): ");

            io::stdout().flush().unwrap();
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Error to read user input");

            fn exec_installation() {

                utils::remove_debian();
                utils::utils_debian();
                utils::install_debian_cutefish();
                utils::remove_files_debian();
                Command::new("sudo").args(Some("reboot")).status().expect("Error to restarting system");

            }

            match &input[..] {

                "y" => exec_installation(),
                "Y" => exec_installation(),
                "" => exec_installation(),
                
                "n" => {

                    println!("Aborted installation");
                    exit(0);

                },
                "N" => {

                    println!("Aborted installation");
                    exit(0);

                },
                _ => {

                    println!("Aborted installation");
                    exit(0);
                    
                }

            }

        },

        "--clean-debian" => utils::clean_debian(),


        "--install-fedora-lxde" => {

            println!("The following packages will be removed from your system and after removal your system will be updated:");
            println!("");
            println!("{}",all_packages_to_remove_fedora);
            println!("");
            println!("");
            println!("The following packages will be installed:");
            println!("");
            println!("lxappearance lxde-common lxdm lxinput lxmenu-data lxpanel lxpolkit lxrandr xcompmgr xarchiver lxsession lxtask pcmanfm lxterminal network-manager-applet openbox obconf lightdm-gtk-greeter lightdm");
            println!("");
            println!("");
            print!("Do you wish to continue? (Y/n): ");

            io::stdout().flush().unwrap();
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Error to read user input");

            fn exec_installation() {

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

                utils::remove_fedora();
                utils::utils_fedora();
                utils::install_fedora_lxde();
                Command::new("sudo").args(Some("reboot")).status().expect("Error to restarting system");

            }

            match &input[..] {

                "y" => exec_installation(),
                "Y" => exec_installation(),
                "" => exec_installation(),
                
                "n" => {

                    println!("Aborted installation");
                    exit(0);

                },
                "N" => {

                    println!("Aborted installation");
                    exit(0);

                },
                _ => {

                    println!("Aborted installation");
                    exit(0);
                    
                }

            }

        },

        "--install-fedora-lxqt" => {

            println!("The following packages will be removed from your system and after removal your system will be updated:");
            println!("");
            println!("{}",all_packages_to_remove_fedora);
            println!("");
            println!("");
            println!("The following packages will be installed:");
            println!("");
            println!("lightdm lightdm-gtk-greeter breeze-cursor-theme breeze-gtk breeze-icon-theme firewall-config network-manager-applet notification-daemon obconf openbox pcmanfm-qt qterminal lxqt-about lxqt-archiver lxqt-config lxqt-notificationd lxqt-openssh-askpass lxqt-panel lxqt-policykit lxqt-powermanagement lxqt-qtplugin lxqt-session lxqt-themes lxqt-themes-fedora");
            println!("");
            println!("");
            print!("Do you wish to continue? (Y/n): ");

            io::stdout().flush().unwrap();
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Error to read user input");

            fn exec_installation() {

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

                utils::remove_fedora();
                utils::utils_fedora();
                utils::install_fedora_lxqt();
                Command::new("sudo").args(Some("reboot")).status().expect("Error to restarting system");

            }

            match &input[..] {

                "y" => exec_installation(),
                "Y" => exec_installation(),
                "" => exec_installation(),
                
                "n" => {

                    println!("Aborted installation");
                    exit(0);

                },
                "N" => {

                    println!("Aborted installation");
                    exit(0);

                },
                _ => {

                    println!("Aborted installation");
                    exit(0);
                    
                }

            }

        },

        "--install-fedora-xfce" => {

            println!("The following packages will be removed from your system and after removal your system will be updated:");
            println!("");
            println!("{}",all_packages_to_remove_fedora);
            println!("");
            println!("");
            println!("The following packages will be installed:");
            println!("");
            println!("network-manager-applet xfwm4 xfce4-power-manager xfce4-session xfce4-settings xfce4-whiskermenu-plugin xfdesktop lightdm lightdm-gtk-greeter xfce4-terminal");
            println!("");
            println!("");
            print!("Do you wish to continue? (Y/n): ");

            io::stdout().flush().unwrap();
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Error to read user input");

            fn exec_installation() {

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

                utils::remove_fedora();
                utils::utils_fedora();
                utils::install_fedora_xfce();
                Command::new("sudo").args(Some("reboot")).status().expect("Error to restarting system");

            }

            match &input[..] {

                "y" => exec_installation(),
                "Y" => exec_installation(),
                "" => exec_installation(),
                
                "n" => {

                    println!("Aborted installation");
                    exit(0);

                },
                "N" => {

                    println!("Aborted installation");
                    exit(0);

                },
                _ => {

                    println!("Aborted installation");
                    exit(0);
                    
                }

            }

        },

        "--install-fedora-gnome" => {

            println!("The following packages will be removed from your system and after removal your system will be updated:");
            println!("");
            println!("{}",all_packages_to_remove_fedora);
            println!("");
            println!("");
            println!("The following packages will be installed:");
            println!("");
            println!("gdm gnome-shell nautilus gnome-terminal fedora-workstation-backgrounds file-roller gnome-terminal-nautilus seahorse");
            println!("");
            println!("");
            print!("Do you wish to continue? (Y/n): ");

            io::stdout().flush().unwrap();
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Error to read user input");

            fn exec_installation() {

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

                utils::remove_fedora();
                utils::utils_fedora();
                utils::install_fedora_gnome();
                Command::new("sudo").args(Some("reboot")).status().expect("Error to restarting system");

            }

            match &input[..] {

                "y" => exec_installation(),
                "Y" => exec_installation(),
                "" => exec_installation(),
                
                "n" => {

                    println!("Aborted installation");
                    exit(0);

                },
                "N" => {

                    println!("Aborted installation");
                    exit(0);

                },
                _ => {

                    println!("Aborted installation");
                    exit(0);
                    
                }

            }

        },

        "--install-fedora-cinnamon" => {

            println!("The following packages will be removed from your system and after removal your system will be updated:");
            println!("");
            println!("{}",all_packages_to_remove_fedora);
            println!("");
            println!("");
            println!("The following packages will be installed:");
            println!("");
            println!("cinnamon cinnamon-control-center cinnamon-desktop cinnamon-menus cinnamon-screensaver cinnamon-session nemo nemo-fileroller cinnamon-translations cjs muffin gnome-terminal lightdm lightdm-gtk-greeter");
            println!("");
            println!("");
            print!("Do you wish to continue? (Y/n): ");

            io::stdout().flush().unwrap();
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Error to read user input");

            fn exec_installation() {

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

                utils::remove_fedora();
                utils::utils_fedora();
                utils::install_fedora_cinnamon();
                Command::new("sudo").args(Some("reboot")).status().expect("Error to restarting system");

            }

            match &input[..] {

                "y" => exec_installation(),
                "Y" => exec_installation(),
                "" => exec_installation(),
                
                "n" => {

                    println!("Aborted installation");
                    exit(0);

                },
                "N" => {

                    println!("Aborted installation");
                    exit(0);

                },
                _ => {

                    println!("Aborted installation");
                    exit(0);
                    
                }

            }

        },

        "--install-fedora-mate" => {

            println!("The following packages will be removed from your system and after removal your system will be updated:");
            println!("");
            println!("{}",all_packages_to_remove_fedora);
            println!("");
            println!("");
            println!("The following packages will be installed:");
            println!("");
            println!("lightdm lightdm-gtk-greeter mate-control-center mate-desktop mate-power-manager mate-screensaver mate-screenshot mate-session-manager mate-settings-daemon mate-terminal network-manager-applet mate-panel marco caja");
            println!("");
            println!("");
            print!("Do you wish to continue? (Y/n): ");

            io::stdout().flush().unwrap();
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Error to read user input");

            fn exec_installation() {

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

                utils::remove_fedora();
                utils::utils_fedora();
                utils::install_fedora_mate();
                Command::new("sudo").args(Some("reboot")).status().expect("Error to restarting system");

            }

            match &input[..] {

                "y" => exec_installation(),
                "Y" => exec_installation(),
                "" => exec_installation(),
                
                "n" => {

                    println!("Aborted installation");
                    exit(0);

                },
                "N" => {

                    println!("Aborted installation");
                    exit(0);

                },
                _ => {

                    println!("Aborted installation");
                    exit(0);
                    
                }

            }

        },

        "--install-fedora-kdeplasma" => {

            println!("The following packages will be removed from your system and after removal your system will be updated:");
            println!("");
            println!("{}",all_packages_to_remove_fedora);
            println!("");
            println!("");
            println!("The following packages will be installed:");
            println!("");
            println!("sddm plasma-desktop plasma-nm konsole kcm_colors kcm-fcitx kscreen ksysguard spectacle plasma-user-manager dolphin plasma-discover");
            println!("");
            println!("");
            print!("Do you wish to continue? (Y/n): ");

            io::stdout().flush().unwrap();
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Error to read user input");

            fn exec_installation() {

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

                utils::remove_fedora();
                utils::utils_fedora();
                utils::install_fedora_kdeplasma();
                Command::new("sudo").args(Some("reboot")).status().expect("Error to restarting system");

            }

            match &input[..] {

                "y" => exec_installation(),
                "Y" => exec_installation(),
                "" => exec_installation(),
                
                "n" => {

                    println!("Aborted installation");
                    exit(0);

                },
                "N" => {

                    println!("Aborted installation");
                    exit(0);

                },
                _ => {

                    println!("Aborted installation");
                    exit(0);
                    
                }

            }

        },

        "--install-fedora-bspwm" => {

            println!("The following packages will be removed from your system and after removal your system will be updated:");
            println!("");
            println!("{}",all_packages_to_remove_fedora);
            println!("");
            println!("");
            println!("The following packages will be installed:");
            println!("");
            println!(""); // Place the packages to be installed
            println!("");
            println!("");
            print!("Do you wish to continue? (Y/n): ");

            io::stdout().flush().unwrap();
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Error to read user input");

            fn exec_installation() {

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

                utils::remove_fedora();
                utils::utils_fedora();
                utils::install_fedora_bspwm();
                Command::new("sudo").args(Some("reboot")).status().expect("Error to restarting system");

            }

            match &input[..] {

                "y" => exec_installation(),
                "Y" => exec_installation(),
                "" => exec_installation(),
                
                "n" => {

                    println!("Aborted installation");
                    exit(0);

                },
                "N" => {

                    println!("Aborted installation");
                    exit(0);

                },
                _ => {

                    println!("Aborted installation");
                    exit(0);
                    
                }

            }

        },

        "--install-fedora-cutefish" => {

            println!("The following packages will be removed from your system and after removal your system will be updated:");
            println!("");
            println!("{}",all_packages_to_remove_fedora);
            println!("");
            println!("");
            println!("The following packages will be installed:");
            println!("");
            println!(""); // Place the packages to be installed
            println!("");
            println!("");
            print!("Do you wish to continue? (Y/n): ");

            io::stdout().flush().unwrap();
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Error to read user input");

            fn exec_installation() {

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

                utils::remove_fedora();
                utils::utils_fedora();
                utils::install_fedora_cutefish();
                Command::new("sudo").args(Some("reboot")).status().expect("Error to restarting system");

            }

            match &input[..] {

                "y" => exec_installation(),
                "Y" => exec_installation(),
                "" => exec_installation(),
                
                "n" => {

                    println!("Aborted installation");
                    exit(0);

                },
                "N" => {

                    println!("Aborted installation");
                    exit(0);

                },
                _ => {

                    println!("Aborted installation");
                    exit(0);
                    
                }

            }

        },

        "--clean-fedora" => utils::clean_fedora(),

        "--help" => println!("{}",texts::HELP_EN_US),

        _ => println!("{}",texts::HELP_EN_US)

    }

}