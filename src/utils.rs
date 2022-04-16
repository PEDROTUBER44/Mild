use std::{
    process::{
        Command, // Importing the standard command library for running operating system commands
        exit // Importing the standard exit library to exit the program
    },
    io::{
        Write // Importing the default write library to write to files and more
    },
    io // Importing the standard io (Input & Output) library to capture user input
};
use colored::Colorize; // Library to customize the terminal font

pub fn show_the_changes_that_will_be_made_to_user(all_packages_to_remove: &str, all_packages_to_install: &str) {

    println!("These packages will be removed from your system which will then be updated:");
    println!("");
    println!("{}",all_packages_to_remove.red().bold());
    println!("");
    println!("");
    println!("These packages will be installed:");
    println!("");
    println!("{}",all_packages_to_install.green().bold());
    println!("");
    println!("");
    print!("Do you want to continue? ({}/{}): ","Y".green().bold(), "n".red().bold());
    
    io::stdout().flush().unwrap();

}

pub fn remove_extra_packages(system: &str) {

    match system {

        "archlinux" => {

            let packages_to_remove_all_desktop_enviroments_from_system = "pacman -Rsn lxde lightdm lightdm-gtk-greeter adwaita-icon-theme xarchiver lxqt xfce4-settings xfce4-pulseaudio-plugin exo garcon tumbler xfce4-panel xfce4-session xfce4-whiskermenu-plugin xfce4-terminal xfconf xfdesktop xfwm4 thunar file-roller gdm weston gnome-session gnome-terminal nautilus-terminal nautilus gnome-control-center gedit eog evince cinnamon cinnamon-session cinnamon-desktop gnome-terminal cinnamon-control-center cinnamon-menus cinnamon-screensaver cinnamon-settings-daemon cinnamon-translations cjs muffin nemo nemo-fileroller mate-control-center mate-desktop mate-power-manager mate-screensaver mate-common mate-session-manager mate-settings-daemon mate-terminal mate-panel marco caja sddm plasma-desktop plasma-nm konsole plasma-wayland-session kcm-fcitx kscreen ksysguard spectacle dolphin discover cutefish --noconfirm";
            let disable_gdm_from_systemd = "systemctl disable gdm -f";
            let disable_lightdm_from_systemd = "systemctl disable lightdm -f";
            let disable_sddm_from_systemd = "systemctl disable sddm -f";

            Command::new("sudo").args(packages_to_remove_all_desktop_enviroments_from_system.split_ascii_whitespace()).status().expect("Error removing all graphical environments");
            Command::new("sudo").args(disable_gdm_from_systemd.split_ascii_whitespace()).status().expect("Error disabling gdm on startup");
            Command::new("sudo").args(disable_lightdm_from_systemd.split_ascii_whitespace()).status().expect("Error disabling lightdm on startup");
            Command::new("sudo").args(disable_sddm_from_systemd.split_ascii_whitespace()).status().expect("Error disabling sddm on startup");

        },

        "debian" => {

            let packages_to_remove_all_desktop_enviroments_from_system = "apt remove lightdm lightdm-gtk-greeter lxde-core lxterminal lxappearance pavucontrol lxsession-default-apps xscreensaver policykit-1 xarchiver lxqt-core pavucontrol thunar xfce4-panel xfce4-pulseaudio-plugin xfce4-whiskermenu-plugin xfce4-session xfce4-settings xfce4-terminal xfconf xfdesktop4 xfwm4 adwaita-qt qt5ct gdm3 gnome-session gnome-control-center gnome-terminal gnome-tweaks nautilus adwaita-icon-theme cinnamon-core marco mate-desktop-environment-core dolphin kwrite ark kde-spectacle okular ksysguard plasma-discover kscreen konsole sddm kde-plasma-desktop plasma-nm plasma-workspace-wayland -y";
            let disable_gdm_from_systemd = "systemctl disable gdm -f";
            let disable_lightdm_from_systemd = "systemctl disable lightdm -f";
            let disable_sddm_from_systemd = "systemctl disable sddm -f";

            Command::new("sudo").args(packages_to_remove_all_desktop_enviroments_from_system.split_ascii_whitespace()).status().expect("Error removing all graphical environments");
            Command::new("sudo").args(disable_gdm_from_systemd.split_ascii_whitespace()).status().expect("Error disabling gdm on startup");
            Command::new("sudo").args(disable_lightdm_from_systemd.split_ascii_whitespace()).status().expect("Error disabling lightdm on startup");
            Command::new("sudo").args(disable_sddm_from_systemd.split_ascii_whitespace()).status().expect("Error disabling sddm on startup");
        
        },

        "fedora" => {

            let packages_to_remove_all_desktop_enviroments_from_system = "dnf remove @lxde-desktop @plasma-desktop @gnome-desktop @cinnamon-desktop @lxqt-desktop @mate-desktop @xfce-desktop lxappearance lxde-common lxdm lxinput lxmenu-data lxpanel lxpolkit lxrandr xcompmgr xarchiver lxsession lxtask pcmanfm lxterminal network-manager-applet openbox obconf lightdm-gtk-greeter lightdm sddm plasma-desktop plasma-nm konsole kcm_colors kcm-fcitx kscreen ksysguard spectacle plasma-user-manager dolphin plasma-discover gdm gnome-shell nautilus gnome-terminal fedora-workstation-backgrounds file-roller gnome-terminal-nautilus cinnamon cinnamon-control-center cinnamon-desktop cinnamon-menus cinnamon-screensaver cinnamon-session nemo nemo-fileroller cinnamon-translations cjs muffin gnome-terminal breeze-cursor-theme breeze-gtk breeze-icon-theme firewall-config network-manager-applet notification-daemon obconf openbox pcmanfm-qt qterminal lxqt-about lxqt-archiver lxqt-config lxqt-notificationd lxqt-openssh-askpass lxqt-panel lxqt-policykit lxqt-powermanagement lxqt-qtplugin lxqt-session lxqt-themes lxqt-themes-fedora network-manager-applet xfwm4 xfce4-power-manager xfce4-session xfce4-settings xfce4-whiskermenu-plugin xfdesktop xfce4-terminal mate-control-center mate-desktop mate-power-manager mate-screensaver mate-screenshot mate-session-manager mate-settings-daemon mate-terminal network-manager-applet mate-panel marco caja gstreamer1-plugins-base gstreamer1-plugins-good gstreamer1-plugins-ugly gstreamer1-plugins-bad-free gstreamer1-plugins-bad-free gstreamer1-plugins-bad-freeworld gstreamer1-plugins-bad-free-extras ffmpeg -y";
            let disable_gdm_from_systemd = "systemctl disable gdm -f";
            let disable_lightdm_from_systemd = "systemctl disable lightdm -f";
            let disable_sddm_from_systemd = "systemctl disable sddm -f";

            Command::new("sudo").args(packages_to_remove_all_desktop_enviroments_from_system.split_ascii_whitespace()).status().expect("Error removing graphical environments and their dependencies");
            Command::new("sudo").args(disable_gdm_from_systemd.split_ascii_whitespace()).status().expect("Error disabling gdm on startup");
            Command::new("sudo").args(disable_lightdm_from_systemd.split_ascii_whitespace()).status().expect("Error disabling lightdm on startup");
            Command::new("sudo").args(disable_sddm_from_systemd.split_ascii_whitespace()).status().expect("Error disabling sddm on startup");

        },

        _ => {

            println!("Internal error, system not found");

        }

    }

}

pub fn install_utils(system: &str) {

    match system {

        "archlinux" => {

            let utils_to_install = "pacman -Syu xorg networkmanager gvfs-mtp gvfs-goa gvfs-google exfat-utils p7zip firefox zip unzip unrar system-config-printer adwaita-icon-theme xf86-video-intel libgl mesa nvidia nvidia-libgl xf86-video-amdgpu --noconfirm";
            let enable_networkmanager_on_systemd = "systemctl enable NetworkManager -f";

            Command::new("sudo").args(utils_to_install.split_ascii_whitespace()).status().expect("Error installing archlinux utilities");
            Command::new("sudo").args(enable_networkmanager_on_systemd.split_ascii_whitespace()).status().expect("Error starting NetworkManager at startup");
            
        },

        "debian" => {

            let utils_to_install = "apt install sudo zip unzip unrar-free network-manager xorg firefox-esr gvfs pulseaudio exfat-utils p7zip-full system-config-printer adwaita-icon-theme -y";
            let enable_networkmanager_on_systemd = "systemctl enable NetworkManager -f";

            Command::new("sudo").args(utils_to_install.split_ascii_whitespace()).status().expect("Error installing debian utilities 11");
            Command::new("sudo").args(enable_networkmanager_on_systemd.split_ascii_whitespace()).status().expect("Error starting NetworkManager at startup");

        },

        "fedora" => {

            let update_system = "dnf update -y";
            let utils_to_install = "dnf install unrar p7zip zip unzip NetworkManager fedora-workstation-backgrounds firefox exfat-utils gvfs-mtp gvfs-goa system-config-printer gstreamer1-plugins-base gstreamer1-plugins-good gstreamer1-plugins-ugly gstreamer1-plugins-bad-free gstreamer1-plugins-bad-free gstreamer1-plugins-bad-freeworld gstreamer1-plugins-bad-free-extras ffmpeg https://download1.rpmfusion.org/nonfree/fedora/rpmfusion-nonfree-release-35.noarch.rpm https://download1.rpmfusion.org/free/fedora/rpmfusion-free-release-35.noarch.rpm -y";
            let enable_networkmanager_on_systemd = "systemctl enable NetworkManager -f";
        
            Command::new("sudo").args(update_system.split_ascii_whitespace()).status().expect("Error updating fedora 35");
            Command::new("sudo").args(utils_to_install.split_ascii_whitespace()).status().expect("Error installing fedora 35 utilities");
            Command::new("sudo").args(enable_networkmanager_on_systemd.split_ascii_whitespace()).status().expect("Error starting NetworkManager at startup");

        },

        _ => {

            println!("Internal error, system not found");

        }

    }

}

pub fn remove_extra_files(system: &str) {

    match system {

        "archlinux" => {

            let rename_avahi_discover = "mv /usr/share/applications/avahi-discover.desktop /usr/share/applications/avahi-discover.backup";
            let rename_bssh = "mv /usr/share/applications/bssh.desktop /usr/share/applications/bssh.backup";
            let rename_bvnc = "mv /usr/share/applications/bvnc.desktop /usr/share/applications/bvnc.backup";
            let rename_nm_connection_editor = "mv /usr/share/applications/nm-connection-editor.desktop /usr/share/applications/nm-connection-editor.backup";
            let rename_qv4l2 = "mv /usr/share/applications/qv4l2.desktop /usr/share/applications/qv4l2.backup";
            let rename_qvidcap = "mv /usr/share/applications/qvidcap.desktop /usr/share/applications/qvidcap.backup";

            Command::new("sudo").args(rename_avahi_discover.split_ascii_whitespace()).status().expect("Error to rename file: avahi-discover.desktop");
            Command::new("sudo").args(rename_bssh.split_ascii_whitespace()).status().expect("Error to rename file: bssh.desktop");
            Command::new("sudo").args(rename_bvnc.split_ascii_whitespace()).status().expect("Error to rename file: bvnc.desktop");
            Command::new("sudo").args(rename_nm_connection_editor.split_ascii_whitespace()).status().expect("Error to rename file: nm-connection-editor.desktop");
            Command::new("sudo").args(rename_qv4l2.split_ascii_whitespace()).status().expect("Error to rename file: qv4l2.desktop");
            Command::new("sudo").args(rename_qvidcap.split_ascii_whitespace()).status().expect("Error to rename file: qvidcap.desktop");

        },

        "debian" => {

            let rename_vim = "mv /usr/share/applications/vim.desktop /usr/share/applications/vim.backup";

            Command::new("sudo").args(rename_vim.split_ascii_whitespace()).status().expect("Error to rename file: vim.desktop");

        },

        "fedora" => {

            println!("No extra files to remove");

        },

        _ => {

            println!("Internal error, system not found");

        }

    }

}

pub fn clean_system(system: &str) {

    match system {

        "archlinux" => {

            let command_to_clean_list_of_orphaned_packages = "pacman -Rsn $(pacman -Qdtq) --noconfirm";
            let command_to_clean_pacman_cache = "pacman -Scc --noconfirm";
            let command_to_remove_unused_flatpaks = "flatpak uninstall --unused";
            let command_to_remove_folder_coredump = "rm -rf /var/lib/systemd/coredump/";
            let command_to_limiting_systemd_logs_to_2_days = "journalctl --vacuum-time=2d";
            let command_to_limiting_systemd_logs_to_500_mb = "journalctl --vacuum-size=500M";

            Command::new("sudo").args(command_to_clean_list_of_orphaned_packages.split_ascii_whitespace()).status().expect("Error to removing entire list of orphaned packages");
            Command::new("sudo").args(command_to_clean_pacman_cache.split_ascii_whitespace()).status().expect("Error to clearing pacman cache");
            Command::new("sudo").args(command_to_remove_unused_flatpaks.split_ascii_whitespace()).status().expect("Error to cleaning unused flatpaks");
            Command::new("sudo").args(command_to_remove_folder_coredump.split_ascii_whitespace()).status().expect("Error to remove folder: /var/lib/systemd/coredump/, folder not found");
            Command::new("sudo").args(command_to_limiting_systemd_logs_to_2_days.split_ascii_whitespace()).status().expect("Error to limiting systemd logs to 2 days");
            Command::new("sudo").args(command_to_limiting_systemd_logs_to_500_mb.split_ascii_whitespace()).status().expect("Error limiting systemd logs to 500M");
            exit(0);

        },

        "debian" => {

            let command_to_clean_apt_cache = "apt clean";
            let command_to_clean_apt_dead_packages = "apt autoclean";
            let command_to_clean_orphaned_packages = "apt autoremove -y";
            let command_to_install_deborphan = "apt install deborphan -y";
            let command_to_remove_deborphan_packages = "apt remove $(deborphan) -y";
            let command_to_remove_deborphan_to_system = "apt remove deborphan -y";
            let command_to_remove_unused_flatpaks = "flatpak uninstall --unused";
            let command_to_remove_folder_coredump = "rm -rf /var/lib/systemd/coredump/";
            let command_to_limiting_systemd_logs_to_2_days = "journalctl --vacuum-time=2d";
            let command_to_limiting_systemd_logs_to_500_mb = "journalctl --vacuum-size=500M";


            Command::new("sudo").args(command_to_clean_apt_cache.split_ascii_whitespace()).status().expect("Error to clearing apt cache");
            Command::new("sudo").args(command_to_clean_apt_dead_packages.split_ascii_whitespace()).status().expect("Error to cleaning dead packages");
            Command::new("sudo").args(command_to_clean_orphaned_packages.split_ascii_whitespace()).status().expect("Error cleaning orphaned packages");
            Command::new("sudo").args(command_to_install_deborphan.split_ascii_whitespace()).status().expect("Error to installing deborphan");
            Command::new("sudo").args(command_to_remove_deborphan_packages.split_ascii_whitespace()).status().expect("Error to cleaning orphaned packages");
            Command::new("sudo").args(command_to_remove_deborphan_packages.split_ascii_whitespace()).status().expect("Error to cleaning 2 time orphaned packages");
            Command::new("sudo").args(command_to_remove_deborphan_packages.split_ascii_whitespace()).status().expect("Error to cleaning 3 time orphaned packages");
            Command::new("sudo").args(command_to_remove_deborphan_packages.split_ascii_whitespace()).status().expect("Error to cleaning 4 time orphaned packages");
            Command::new("sudo").args(command_to_remove_deborphan_to_system.split_ascii_whitespace()).status().expect("Error to remove deborphan from system");
            Command::new("sudo").args(command_to_clean_orphaned_packages.split_ascii_whitespace()).status().expect("Error to removing deborphan dependencies");
            Command::new("sudo").args(command_to_remove_unused_flatpaks.split_ascii_whitespace()).status().expect("Error to cleaning unused flatpaks");
            Command::new("sudo").args(command_to_remove_folder_coredump.split_ascii_whitespace()).status().expect("Error to remove folder: /var/lib/systemd/coredump/, folder not found");
            Command::new("sudo").args(command_to_limiting_systemd_logs_to_2_days.split_ascii_whitespace()).status().expect("Error to limiting systemd logs to 2 days");
            Command::new("sudo").args(command_to_limiting_systemd_logs_to_500_mb.split_ascii_whitespace()).status().expect("Error limiting systemd logs to 500M");
            exit(0);

        },

        "fedora" => {

            let command_to_clean_dnf_cache = "dnf clean all";
            let command_to_clean_dnf_orphaned_packages = "dnf autoremove -y";
            let command_to_remove_unused_flatpaks = "flatpak uninstall --unused";
            let command_to_remove_folder_coredump = "rm -rf /var/lib/systemd/coredump/";
            let command_to_limiting_systemd_logs_to_2_days = "journalctl --vacuum-time=2d";
            let command_to_limiting_systemd_logs_to_500_mb = "journalctl --vacuum-size=500M";

            Command::new("sudo").args(command_to_clean_dnf_orphaned_packages.split_ascii_whitespace()).status().expect("Error removing orphaned dnf packages");
            Command::new("sudo").args(command_to_clean_dnf_cache.split_ascii_whitespace()).status().expect("Error to clean dnf cache");
            Command::new("sudo").args(command_to_remove_unused_flatpaks.split_ascii_whitespace()).status().expect("Error to cleaning unused flatpaks");
            Command::new("sudo").args(command_to_remove_folder_coredump.split_ascii_whitespace()).status().expect("Error to remove folder: /var/lib/systemd/coredump/, folder not found");
            Command::new("sudo").args(command_to_limiting_systemd_logs_to_2_days.split_ascii_whitespace()).status().expect("Error to limiting systemd logs to 2 days");
            Command::new("sudo").args(command_to_limiting_systemd_logs_to_500_mb.split_ascii_whitespace()).status().expect("Error limiting systemd logs to 500M");
            exit(0);
            
        },

        _ => {

            println!("Internal error, system not found");

        }

    }

}

pub fn install_desktop_in_system(system: &str, desktop: &str) {

    match system {

        "archlinux" => {

            match desktop {

                "lxde" => {

                    let command_to_install_archlinux_lxde = "pacman -Syu lxde lightdm lightdm-gtk-greeter adwaita-icon-theme xarchiver --noconfirm";
                    let enable_lightdm_in_systemd = "systemctl enable lightdm -f";

                    Command::new("sudo").args(command_to_install_archlinux_lxde.split_ascii_whitespace()).status().expect("Error installing minimal lxde on archlinux");
                    Command::new("sudo").args(enable_lightdm_in_systemd.split_ascii_whitespace()).status().expect("Error enabling lightdm on startup");

                },

                "lxqt" => {

                    let command_to_install_archlinux_lxqt = "pacman -Syu lxqt lightdm lightdm-gtk-greeter adwaita-icon-theme xarchiver --noconfirm";
                    let enable_lightdm_in_systemd = "systemctl enable lightdm -f";

                    Command::new("sudo").args(command_to_install_archlinux_lxqt.split_ascii_whitespace()).status().expect("Error installing minimal lxqt on archlinux");
                    Command::new("sudo").args(enable_lightdm_in_systemd.split_ascii_whitespace()).status().expect("Error enabling lightdm on startup");
                
                },

                "xfce" => {

                    let command_to_install_archlinux_xfce = "pacman -Syu lightdm lightdm-gtk-greeter xfce4-settings xfce4-pulseaudio-plugin adwaita-icon-theme exo garcon tumbler xfce4-panel xfce4-session xfce4-whiskermenu-plugin xfce4-terminal xfconf xfdesktop xfwm4 thunar file-roller --noconfirm";
                    let enable_lightdm_in_systemd = "systemctl enable lightdm -f";

                    Command::new("sudo").args(command_to_install_archlinux_xfce.split_ascii_whitespace()).status().expect("Error installing xfce minimal on archlinux");
                    Command::new("sudo").args(enable_lightdm_in_systemd.split_ascii_whitespace()).status().expect("Error enabling lightdm on startup");

                },

                "gnome" => {

                    let command_to_install_archlinux_gnome = "pacman -Syu gdm weston gnome-session gnome-terminal nautilus file-roller gnome-control-center gedit adwaita-icon-theme eog evince seahorse --noconfirm";
                    let enable_gdm_in_systemd = "systemctl enable gdm -f";
                    let disable_animations = "set org.gnome.desktop.interface enable-animations false";

                    Command::new("sudo").args(command_to_install_archlinux_gnome.split_ascii_whitespace()).status().expect("Error installing gnome minimal on archlinux");
                    Command::new("sudo").args(enable_gdm_in_systemd.split_ascii_whitespace()).status().expect("Error enabling gdm on startup");
                    Command::new("gsettings").args(disable_animations.split_ascii_whitespace()).status().expect("Error to disable animations on gnome");

                },

                "cinnamon" => {

                    let command_to_install_archlinux_cinnamon = "pacman -Syu lightdm lightdm-gtk-greeter cinnamon cinnamon-session cinnamon-desktop gnome-terminal cinnamon-control-center cinnamon-menus cinnamon-screensaver cinnamon-settings-daemon cinnamon-translations adwaita-icon-theme cjs muffin nemo nemo-fileroller file-roller --noconfirm";
                    let enable_lighdm_in_systemd = "systemctl enable lightdm -f";

                    Command::new("sudo").args(command_to_install_archlinux_cinnamon.split_ascii_whitespace()).status().expect("Error installing cinnamon minimal on archlinux");
                    Command::new("sudo").args(enable_lighdm_in_systemd.split_ascii_whitespace()).status().expect("Error enabling lightdm on startup");

                },

                "mate" => {

                    let command_to_install_archlinux_mate = "pacman -Syu lightdm lightdm-gtk-greeter mate-desktop adwaita-icon-theme mate-control-center mate-power-manager mate-screensaver mate-common mate-session-manager mate-settings-daemon mate-terminal network-manager-applet mate-panel marco caja --noconfirm";
                    let enable_lightdm_in_systemd = "systemctl enable lightdm -f";

                    Command::new("sudo").args(command_to_install_archlinux_mate.split_ascii_whitespace()).status().expect("Error installing minimal mate on archlinux");
                    Command::new("sudo").args(enable_lightdm_in_systemd.split_ascii_whitespace()).status().expect("Error enabling lightdm on startup");

                },

                "kdeplasma" => {

                    let command_to_install_archlinux_kdeplasma = "pacman -Syu sddm plasma-desktop plasma-nm konsole plasma-wayland-session kcm-fcitx kscreen ksysguard adwaita-icon-theme spectacle dolphin discover --noconfirm";
                    let enable_sddm_in_systemd = "systemctl enable sddm -f";

                    Command::new("sudo").args(command_to_install_archlinux_kdeplasma.split_ascii_whitespace()).status().expect("Error installing cinnamon minimal on archlinux");
                    Command::new("sudo").args(enable_sddm_in_systemd.split_ascii_whitespace()).status().expect("Error enabling lightdm on startup");

                },

                "bspwm" => {

                    println!("Coming soon");
                    exit(0);

                },

                "cutefish" => {

                    println!("Coming soon");
                    exit(0);

                },

                _ => {

                    println!("Internal error, system not found");
        
                }


            }

        },

        "debian" => {

            match desktop {

                "lxde" => {

                    let command_to_install_debian_lxde = "apt install lightdm lightdm-gtk-greeter lxde-core lxterminal lxappearance pavucontrol lxsession-default-apps xscreensaver policykit-1 xarchiver -y";
                    let enable_lightdm_in_systemd = "systemctl enable lightdm -f";

                    Command::new("sudo").args(command_to_install_debian_lxde.split_ascii_whitespace()).status().expect("Error installing minimal lxde on debian 11");
                    Command::new("sudo").args(enable_lightdm_in_systemd.split_ascii_whitespace()).status().expect("Error enabling lightdm on startup");

                },

                "lxqt" => {

                    let command_to_install_debian_lxqt = "apt install lightdm lightdm-gtk-greeter lxqt-core pavucontrol -y";
                    let enable_lightdm_in_systemd = "systemctl enable lightdm -f";

                    Command::new("sudo").args(command_to_install_debian_lxqt.split_ascii_whitespace()).status().expect("Error installing lxqt minimal on debian 11");
                    Command::new("sudo").args(enable_lightdm_in_systemd.split_ascii_whitespace()).status().expect("Error enabling lightdm on startup");

                },

                "xfce" => {

                    let command_to_install_debian_xfce = "apt install lightdm lightdm-gtk-greeter thunar xfce4-panel xfce4-pulseaudio-plugin xfce4-whiskermenu-plugin xfce4-session xfce4-settings xfce4-terminal pavucontrol xfconf xfdesktop4 xfwm4 adwaita-qt qt5ct --no-install-recommends -y";
                    let enable_lightdm_in_systemd = "systemctl enable lightdm -f";

                    Command::new("sudo").args(command_to_install_debian_xfce.split_ascii_whitespace()).status().expect("Error installing xfce4 minimal on debian 11");
                    Command::new("sudo").args(enable_lightdm_in_systemd.split_ascii_whitespace()).status().expect("Error enabling lightdm on startup");

                },

                "gnome" => {

                    let command_to_install_debian_gnome = "apt install gdm3 gnome-session gnome-control-center gnome-terminal gnome-tweaks nautilus adwaita-icon-theme seahorse --no-install-recommends -y";
                    let enable_gdm_in_systemd = "systemctl enable gdm -f";
                    let disable_animations = "set org.gnome.desktop.interface enable-animations false";

                    Command::new("sudo").args(command_to_install_debian_gnome.split_ascii_whitespace()).status().expect("Error installing gnome minimal on debian 11");
                    Command::new("sudo").args(enable_gdm_in_systemd.split_ascii_whitespace()).status().expect("Error enabling gdm3 on startup");
                    Command::new("gsettings").args(disable_animations.split_ascii_whitespace()).status().expect("Error to disable animations on gnome");

                },

                "cinnamon" => {

                    let command_to_install_debian_cinnamon = "apt install lightdm lightdm-gtk-greeter cinnamon-core --no-install-recommends -y";
                    let enable_lightdm_in_systemd = "systemctl enable lightdm -f";

                    Command::new("sudo").args(command_to_install_debian_cinnamon.split_ascii_whitespace()).status().expect("Error installing cinnamon minimal on debian 11");
                    Command::new("sudo").args(enable_lightdm_in_systemd.split_ascii_whitespace()).status().expect("Error enabling lightdm on startup");

                },

                "mate" => {

                    let command_to_install_debian_mate = "apt install lightdm lightdm-gtk-greeter mate-desktop-environment-core marco -y";
                    let enable_lightdm_in_systemd = "systemctl enable lightdm -f";

                    Command::new("sudo").args(command_to_install_debian_mate.split_ascii_whitespace()).status().expect("Error installing minimal mate on debian 11");
                    Command::new("sudo").args(enable_lightdm_in_systemd.split_ascii_whitespace()).status().expect("Error enabling lightdm on startup");

                },

                "kdeplasma" => {

                    let command_to_install_debian_kdeplasma = "apt install sddm kde-plasma-desktop plasma-nm plasma-workspace-wayland dolphin kwrite ark kde-spectacle okular ksysguard plasma-discover kscreen konsole --no-install-recommends -y";
                    let enable_sddm_in_systemd = "systemctl enable sddm -f";

                    Command::new("sudo").args(command_to_install_debian_kdeplasma.split_ascii_whitespace()).status().expect("Error installing minimal kde plasma on debian 11");
                    Command::new("sudo").args(enable_sddm_in_systemd.split_ascii_whitespace()).status().expect("Error enabling sddm on startup");

                },

                "bspwm" => {

                    println!("Coming soon");
                    exit(0);

                },

                "cutefish" => {

                    println!("Coming soon");
                    exit(0);

                },

                _ => {

                    println!("Internal error, system not found");
        
                }

            }

        },

        "fedora" => {

            match desktop {

                "lxde" => {

                    let command_to_install_fedora_lxde = "dnf install lightdm lightdm-gtk-greeter lxde-common lxdm openbox lxappearance lxsession lxterminal pcmanfm lxinput lxmenu-data lxpanel lxpolkit lxrandr lxtask xcompmgr xarchiver obconf network-manager-applet -y";
                    let enable_lightdm_in_systemd = "systemctl enable lightdm -f";
                    let set_graphical_login_screen_default = "systemctl set-default graphical.target";

                    Command::new("sudo").args(command_to_install_fedora_lxde.split_ascii_whitespace()).status().expect("Error installing minimal lxde on fedora 35");
                    Command::new("sudo").args(enable_lightdm_in_systemd.split_ascii_whitespace()).status().expect("Error enabling lightdm on startup");
                    Command::new("sudo").args(set_graphical_login_screen_default.split_ascii_whitespace()).status().expect("Error enabling graphical mode boot");

                },

                "lxqt" => {

                    let command_to_install_fedora_lxqt = "dnf install lightdm lightdm-gtk-greeter lxqt-about lxqt-archiver lxqt-config lxqt-notificationd lxqt-openssh-askpass lxqt-panel breeze-cursor-theme breeze-gtk breeze-icon-theme firewall-config network-manager-applet notification-daemon obconf openbox pcmanfm-qt qterminal lxqt-policykit lxqt-powermanagement lxqt-qtplugin lxqt-session lxqt-themes lxqt-themes-fedora -y";
                    let enable_lightdm_in_systemd = "systemctl enable lightdm -f";
                    let set_graphical_login_screen_default = "systemctl set-default graphical.target";

                    Command::new("sudo").args(command_to_install_fedora_lxqt.split_ascii_whitespace()).status().expect("Error installing lxqt minimal on fedora 35");
                    Command::new("sudo").args(enable_lightdm_in_systemd.split_ascii_whitespace()).status().expect("Error enabling lightdm on startup");
                    Command::new("sudo").args(set_graphical_login_screen_default.split_ascii_whitespace()).status().expect("Error enabling graphical mode boot");

                },

                "xfce" => {

                    let command_to_install_fedora_xfce = "dnf install lightdm lightdm-gtk-greeter xfwm4 xfce4-session xfdesktop xfce4-settings xfce4-terminal xfce4-whiskermenu-plugin xfce4-power-manager network-manager-applet -y";
                    let enable_lightdm_in_systemd = "systemctl enable lightdm -f";
                    let set_graphical_login_screen_default = "systemctl set-default graphical.target";

                    Command::new("sudo").args(command_to_install_fedora_xfce.split_ascii_whitespace()).status().expect("Error installing xfce4 minimal on fedora 35");
                    Command::new("sudo").args(enable_lightdm_in_systemd.split_ascii_whitespace()).status().expect("Error enabling lightdm on startup");
                    Command::new("sudo").args(set_graphical_login_screen_default.split_ascii_whitespace()).status().expect("Error enabling graphical mode boot");

                },

                "gnome" => {

                    let command_to_install_fedora_gnome = "dnf install gdm gnome-shell nautilus gnome-terminal fedora-workstation-backgrounds file-roller gnome-terminal-nautilus seahorse -y";
                    let enable_gdm_in_systemd = "systemctl enable gdm -f";
                    let set_graphical_login_screen_default = "systemctl set-default graphical.target";
                    let disable_animations = "set org.gnome.desktop.interface enable-animations false";

                    Command::new("sudo").args(command_to_install_fedora_gnome.split_ascii_whitespace()).status().expect("Error installing gnome on fedora 35");
                    Command::new("sudo").args(enable_gdm_in_systemd.split_ascii_whitespace()).spawn().expect("Error enabling gdm on startup");
                    Command::new("sudo").args(set_graphical_login_screen_default.split_ascii_whitespace()).status().expect("Error enabling graphical mode boot");
                    Command::new("gsettings").args(disable_animations.split_ascii_whitespace()).status().expect("Error to disable animations on gnome");

                },

                "cinnamon" => {

                    let command_to_install_fedora_cinnamon = "dnf install lightdm lightdm-gtk-greeter cinnamon cinnamon-desktop cinnamon-session cinnamon-menus cinnamon-screensaver gnome-terminal cinnamon-translations muffin cinnamon-control-center cjs nemo nemo-fileroller -y";
                    let enable_lightdm_in_systemd = "systemctl enable lightdm -f";
                    let set_graphical_login_screen_default = "systemctl set-default graphical.target";

                    Command::new("sudo").args(command_to_install_fedora_cinnamon.split_ascii_whitespace()).status().expect("Error installing cinnamon on fedora 35");
                    Command::new("sudo").args(enable_lightdm_in_systemd.split_ascii_whitespace()).status().expect("Error enabling lightdm on startup");
                    Command::new("sudo").args(set_graphical_login_screen_default.split_ascii_whitespace()).status().expect("Error enabling graphical mode boot");

                },

                "mate" => {

                    let command_to_install_fedora_mate = "dnf install lightdm lightdm-gtk-greeter mate-desktop mate-control-center mate-screensaver mate-power-manager mate-screenshot mate-session-manager mate-settings-daemon mate-terminal mate-panel marco caja network-manager-applet -y";
                    let enable_lightdm_in_systemd = "systemctl enable lightdm -f";
                    let set_graphical_login_screen_default = "systemctl set-default graphical.target";

                    Command::new("sudo").args(command_to_install_fedora_mate.split_ascii_whitespace()).status().expect("Error installing mate in fedora 35");
                    Command::new("sudo").args(enable_lightdm_in_systemd.split_ascii_whitespace()).status().expect("Error enabling lightdm on startup");
                    Command::new("sudo").args(set_graphical_login_screen_default.split_ascii_whitespace()).status().expect("Error enabling graphical mode boot");

                },

                "kdeplasma" => {

                    let command_to_install_fedora_kdeplasma = "dnf install sddm plasma-desktop plasma-nm konsole plasma-discover dolphin kscreen ksysguard spectacle plasma-user-manager kcm_colors kcm-fcitx -y";
                    let enable_sddm_in_systemd = "systemctl enable sddm -f";
                    let set_graphical_login_screen_default = "systemctl set-default graphical.target";

                    Command::new("sudo").args(command_to_install_fedora_kdeplasma.split_ascii_whitespace()).status().expect("Error installing kde plasma on fedora 35");
                    Command::new("sudo").args(enable_sddm_in_systemd.split_ascii_whitespace()).status().expect("Error enabling sddm on startup");
                    Command::new("sudo").args(set_graphical_login_screen_default.split_ascii_whitespace()).status().expect("Error enabling graphical mode boot");

                },

                "bspwm" => {

                    println!("Coming soon");
                    exit(0);

                },

                "cutefish" => {

                    println!("Coming soon");
                    exit(0);

                },

                _ => {

                    println!("Internal error, system not found");
        
                }

            }

        },

        _ => {

            println!("Internal error, system not found");

        }

    }

}

pub fn configure(system: &str) {

    match system {

        "archlinux" => {

            println!("Nothing to configure");
        
        },

        "debian" => {

            println!("Nothing to configure");
        
        },

        "fedora" => {

            let command_to_remove_dnf_config_file = "rm -r /etc/dnf/dnf.conf";
            let first_command_to_written_dnf_conf = r#"echo "[main]" >> /etc/dnf/dnf.conf"#;
            let second_command_to_written_dnf_conf = r#"echo "gpgcheck=1" >> /etc/dnf/dnf.conf"#;
            let third_command_to_written_dnf_conf = r#"echo "installonly_limit=3" >> /etc/dnf/dnf.conf"#;
            let fourth_command_to_written_dnf_conf = r#"echo "clean_requirements_on_remove=True" >> /etc/dnf/dnf.conf"#;
            let fifth_command_to_written_dnf_conf = r#"echo "best=False" >> /etc/dnf/dnf.conf"#;
            let sixth_command_to_written_dnf_conf = r#"echo "skip_if_unavailable=True" >> /etc/dnf/dnf.conf"#;
            let seventh_command_to_written_dnf_conf = r#"echo "fastestmirror=True" >> /etc/dnf/dnf.conf"#;
            let eighth_command_to_written_dnf_conf = r#"echo "max_parallel_downloads=7" >> /etc/dnf/dnf.conf"#;
            let ninth_command_to_written_dnf_conf = r#"echo "defaultyes=True" >> /etc/dnf/dnf.conf"#;
            let tenth_command_to_written_dnf_conf = r#"echo "install_weak_deps=false" >> /etc/dnf/dnf.conf"#;

            Command::new("sudo").args(command_to_remove_dnf_config_file.split_ascii_whitespace()).status().expect("Error to remove /etc/dnf/dnf.conf");
            Command::new("sudo").args(first_command_to_written_dnf_conf.split_ascii_whitespace()).status().expect("File: /etc/dnf/dnf.conf not found");
            Command::new("sudo").args(second_command_to_written_dnf_conf.split_ascii_whitespace()).status().expect("File: /etc/dnf/dnf.conf not found");
            Command::new("sudo").args(third_command_to_written_dnf_conf.split_ascii_whitespace()).status().expect("File: /etc/dnf/dnf.conf not found");
            Command::new("sudo").args(fourth_command_to_written_dnf_conf.split_ascii_whitespace()).status().expect("File: /etc/dnf/dnf.conf not found");
            Command::new("sudo").args(fifth_command_to_written_dnf_conf.split_ascii_whitespace()).status().expect("File: /etc/dnf/dnf.conf not found");
            Command::new("sudo").args(sixth_command_to_written_dnf_conf.split_ascii_whitespace()).status().expect("File: /etc/dnf/dnf.conf not found");
            Command::new("sudo").args(seventh_command_to_written_dnf_conf.split_ascii_whitespace()).status().expect("File: /etc/dnf/dnf.conf not found");
            Command::new("sudo").args(eighth_command_to_written_dnf_conf.split_ascii_whitespace()).status().expect("File: /etc/dnf/dnf.conf not found");
            Command::new("sudo").args(ninth_command_to_written_dnf_conf.split_ascii_whitespace()).status().expect("File: /etc/dnf/dnf.conf not found");
            Command::new("sudo").args(tenth_command_to_written_dnf_conf.split_ascii_whitespace()).status().expect("File: /etc/dnf/dnf.conf not found");

        },

        _ => {

            println!("Internal error, system not found");

        }

    }

}

pub fn exec_installation(system: &str, desktop: &str) {

    configure(system);
    remove_extra_packages(system);
    install_utils(system);
    install_desktop_in_system(system, desktop);
    remove_extra_files(system);
    Command::new("sudo").args(Some("reboot")).status().expect("Error to restarting system");

}