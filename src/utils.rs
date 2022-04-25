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

    loop {

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
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Error to read user input");
        let input = input.trim().to_lowercase();

        match &input[..] {
            "y" | "yes" | ""  => break,
            "n" | "no" => {println!("Aborted installation");exit(0);},
            _ => continue
        }

    }

}

pub fn systemcommand_asuser(package: &str, command: &str, err: &str) {

    Command::new(package).args(command.split_ascii_whitespace()).status().expect(err);

}

pub fn systemcommand_asroot(command: &str, err: &str) {

    Command::new("sudo").args(command.split_ascii_whitespace()).status().expect(err);
    
}

pub fn install_aur(url: &str, folder: &str) {

    Command::new("git").args(Some("clone")).args(Some(url)).status().expect("Error git repository not found");
    systemcommand_asuser("cd", folder, "Error folder not found");
    systemcommand_asuser("makepkg", "-sicr", "Error to compile aur");
    systemcommand_asuser("cd", "..", "Error exiting directory");
    systemcommand_asuser("rm", "-rf", "Error to remove aur folder");

}

pub fn remove_extra_packages(system: &str) {

    match system {

        "archlinux" => {

            systemcommand_asroot("pacman -Rsn lxde lightdm lightdm-gtk-greeter adwaita-icon-theme xarchiver lxqt xfce4-settings xfce4-pulseaudio-plugin exo garcon tumbler xfce4-panel xfce4-session xfce4-whiskermenu-plugin xfce4-terminal xfconf xfdesktop xfwm4 thunar file-roller gdm weston gnome-session gnome-terminal nautilus-terminal nautilus gnome-control-center gedit eog evince cinnamon cinnamon-session cinnamon-desktop gnome-terminal cinnamon-control-center cinnamon-menus cinnamon-screensaver cinnamon-settings-daemon cinnamon-translations cjs muffin nemo nemo-fileroller mate-control-center mate-desktop mate-power-manager mate-screensaver mate-common mate-session-manager mate-settings-daemon mate-terminal mate-panel marco caja sddm plasma-desktop plasma-nm konsole plasma-wayland-session kcm-fcitx kscreen ksysguard spectacle dolphin discover cutefish --noconfirm", "Error removing all graphical environments");
            systemcommand_asroot("systemctl disable gdm -f", "Error disabling gdm on startup");
            systemcommand_asroot("systemctl disable lightdm -f", "Error disabling lightdm on startup");
            systemcommand_asroot("systemctl disable sddm -f", "Error disabling sddm on startup");
            
        },

        "debian" => {

            systemcommand_asroot("apt remove lightdm lightdm-gtk-greeter lxde-core lxterminal deluge file-roller mousepad gpicview gnome-disk-utility evince lxappearance pavucontrol lxsession-default-apps lxinput menu gnome-system-tools connman connman-gtk xscreensaver policykit-1 policykit-1-gnome xarchiver lxqt-core vlc ark ktorrent partitionmanager qpdfview thunar xfce4-panel xfce4-pulseaudio-plugin xfce4-whiskermenu-plugin xfce4-session xfce4-settings xfce4-terminal  thunar-archive-plugin xfconf xfdesktop4 xfwm4 adwaita-qt qt5ct xfce4-taskmanager xfce4-screenshooter gdm3 gnome-session gnome-control-center gnome-software eog totem gedit gnome-terminal gnome-tweaks nautilus adwaita-icon-theme seahorse gnome-system-monitor gnome-screenshot transmission-gtk cinnamon-core mate-desktop-environment sddm kde-plasma-desktop plasma-nm plasma-workspace-wayland systemsettings dolphin kwrite okular plasma-discover konsole kde-spectacle gwenview -y","Error removing all graphical environments");
            systemcommand_asroot("systemctl disable gdm -f", "Error disabling gdm on startup");
            systemcommand_asroot("systemctl disable lightdm -f", "Error disabling lightdm on startup");
            systemcommand_asroot("systemctl disable sddm -f", "Error disabling sddm on startup");
        
        },

        "fedora" => {

            systemcommand_asroot("dnf remove @lxde-desktop @plasma-desktop @gnome-desktop @cinnamon-desktop @lxqt-desktop @mate-desktop @xfce-desktop lxappearance lxde-common lxdm lxinput lxmenu-data lxpanel lxpolkit lxrandr xcompmgr xarchiver lxsession lxtask pcmanfm lxterminal network-manager-applet openbox obconf lightdm-gtk-greeter lightdm sddm plasma-desktop plasma-nm konsole kcm_colors kcm-fcitx kscreen ksysguard spectacle plasma-user-manager dolphin plasma-discover gdm gnome-shell nautilus gnome-terminal fedora-workstation-backgrounds file-roller gnome-terminal-nautilus cinnamon cinnamon-control-center cinnamon-desktop cinnamon-menus cinnamon-screensaver cinnamon-session nemo nemo-fileroller cinnamon-translations cjs muffin gnome-terminal breeze-cursor-theme breeze-gtk breeze-icon-theme firewall-config network-manager-applet notification-daemon obconf openbox pcmanfm-qt qterminal lxqt-about lxqt-archiver lxqt-config lxqt-notificationd lxqt-openssh-askpass lxqt-panel lxqt-policykit lxqt-powermanagement lxqt-qtplugin lxqt-session lxqt-themes lxqt-themes-fedora network-manager-applet xfwm4 xfce4-power-manager xfce4-session xfce4-settings xfce4-whiskermenu-plugin xfdesktop xfce4-terminal mate-control-center mate-desktop mate-power-manager mate-screensaver mate-screenshot mate-session-manager mate-settings-daemon mate-terminal network-manager-applet mate-panel marco caja gstreamer1-plugins-base gstreamer1-plugins-good gstreamer1-plugins-ugly gstreamer1-plugins-bad-free gstreamer1-plugins-bad-free gstreamer1-plugins-bad-freeworld gstreamer1-plugins-bad-free-extras ffmpeg -y", "Error removing graphical environments and their dependencies");
            systemcommand_asroot("systemctl disable gdm -f", "Error disabling gdm on startup");
            systemcommand_asroot("systemctl disable lightdm -f", "Error disabling lightdm on startup");
            systemcommand_asroot("systemctl disable sddm -f", "Error disabling sddm on startup");

        },

        _ => {

            println!("Internal error, system not found");

        }

    }

}

pub fn install_utils(system: &str) {

    match system {

        "archlinux" => {

            systemcommand_asroot("pacman -Syu networkmanager gvfs-mtp gvfs-goa gvfs-google exfat-utils bluez p7zip firefox zip unzip unrar system-config-printer adwaita-icon-theme xf86-video-intel libgl mesa nvidia nvidia-libgl xf86-video-amdgpu ffmpeg gst-plugins-ugly gst-plugins-good gst-plugins-base gst-plugins-bad gst-libav gstreamer git --noconfirm", "Error installing archlinux utilities");
            systemcommand_asroot("systemctl enable NetworkManager -f", "Error enabling NetworkManager autostart at system boot");
            install_aur("https://aur.archlinux.org/preload.git", "preload/");
            systemcommand_asroot("systemctl enable preload -f", "Error enabling preload deamon autostart at system boot");
            systemcommand_asroot("systemctl enable bluez -f", "Error enabling bluetooth deamon autostart at system boot");
            
        },

        "debian" => {

            systemcommand_asroot("apt install sudo zip unzip unrar-free network-manager preload firefox-esr gvfs pulseaudio gstreamer1.0-plugins-base gstreamer1.0-plugins-good gstreamer1.0-plugins-ugly gstreamer1.0-plugins-bad ffmpeg sox twolame vorbis-tools lame faad mencoder exfat-utils p7zip-full system-config-printer adwaita-icon-theme bluez -y", "Error installing debian 11 utilities");
            systemcommand_asroot("systemctl enable NetworkManager -f", "Error enabling NetworkManager autostart at system boot");
            systemcommand_asroot("systemctl enable preload -f", "Error enabling preload deamon autostart at system boot");
            systemcommand_asroot("systemctl enable bluez -f", "Error enabling bluetooth deamon autostart at system boot");

        },

        "fedora" => {

            systemcommand_asroot("dnf update -y", "Error updating fedora 35");
            systemcommand_asroot("dnf install unrar p7zip zip unzip NetworkManager fedora-workstation-backgrounds firefox exfat-utils gvfs-mtp gvfs-goa system-config-printer gstreamer1-plugins-base gstreamer1-plugins-good gstreamer1-plugins-ugly gstreamer1-plugins-bad-free gstreamer1-plugins-bad-free gstreamer1-plugins-bad-freeworld gstreamer1-plugins-bad-free-extras ffmpeg https://download1.rpmfusion.org/nonfree/fedora/rpmfusion-nonfree-release-35.noarch.rpm https://download1.rpmfusion.org/free/fedora/rpmfusion-free-release-35.noarch.rpm -y", "Error installing fedora 35 utilities");
            systemcommand_asroot("systemctl enable NetworkManager -f", "Error enabling NetworkManager autostart at system boot");
            // Add the preload
            // Add the bluetooth

        },

        _ => {

            println!("Internal error, system not found");

        }

    }

}

pub fn remove_extra_files(system: &str) {

    match system {

        "archlinux" => {

            systemcommand_asroot("mv /usr/share/applications/avahi-discover.desktop /usr/share/applications/avahi-discover.backup", "Error to rename file: avahi-discover.desktop");
            systemcommand_asroot("mv /usr/share/applications/bssh.desktop /usr/share/applications/bssh.backup", "Error to rename file: bssh.desktop");
            systemcommand_asroot("mv /usr/share/applications/bvnc.desktop /usr/share/applications/bvnc.backup", "Error to rename file: bvnc.desktop");
            systemcommand_asroot("mv /usr/share/applications/nm-connection-editor.desktop /usr/share/applications/nm-connection-editor.backup", "Error to rename file: nm-connection-editor.desktop");
            systemcommand_asroot("mv /usr/share/applications/qv4l2.desktop /usr/share/applications/qv4l2.backup", "Error to rename file: qv4l2.desktop");
            systemcommand_asroot("mv /usr/share/applications/qvidcap.desktop /usr/share/applications/qvidcap.backup", "Error to rename file: qvidcap.desktop");

        },

        "debian" => {

            systemcommand_asroot("mv /usr/share/applications/vim.desktop /usr/share/applications/vim.backup", "Error to rename file: vim.desktop");

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

            systemcommand_asroot("pacman -Rsn $(pacman -Qdtq) --noconfirm", "Error to removing entire list of orphaned packages");
            systemcommand_asroot("pacman -Scc --noconfirm", "Error to clearing pacman cache");
            systemcommand_asroot("flatpak uninstall --unused", "Error to cleaning unused flatpaks");
            systemcommand_asroot("rm -rf /var/lib/systemd/coredump/", "Error to remove folder: /var/lib/systemd/coredump/, folder not found");
            systemcommand_asroot("rm -rf $HOME/.var/app/*/cache/*", "Error removing flatpaks cache folder, folder not found");
            systemcommand_asroot("rm -rf $HOME/.cache/*", "Error removing cache folder, folder not found");
            systemcommand_asroot("journalctl --vacuum-time=2d", "Error to limiting systemd logs to 2 days");
            systemcommand_asroot("journalctl --vacuum-size=500M", "Error limiting systemd logs to 500M");
            exit(0);

        },

        "debian" => {

            systemcommand_asroot("apt clean", "Error to clearing apt cache");
            systemcommand_asroot("apt autoclean", "Error to cleaning dead packages");
            systemcommand_asroot("apt install deborphan -y", "Error to installing deborphan");
            systemcommand_asroot("apt remove $(deborphan) -y", "Error cleaning orphaned packages for the first time");
            systemcommand_asroot("apt remove $(deborphan) -y", "Error cleaning orphaned packages for the second time");
            systemcommand_asroot("apt remove $(deborphan) -y", "Error cleaning orphaned packages for the third time");
            systemcommand_asroot("apt remove $(deborphan) -y", "Error cleaning orphaned packages for the fourth time");
            systemcommand_asroot("apt remove deborphan -y", "Error to remove deborphan from system");
            systemcommand_asroot("apt autoremove -y", "Error to removing deborphan dependencies");
            systemcommand_asroot("flatpak uninstall --unused", "Error to cleaning unused flatpaks");
            systemcommand_asroot("rm -rf /var/lib/systemd/coredump/", "Error to remove folder: /var/lib/systemd/coredump/, folder not found");
            systemcommand_asroot("rm -rf $HOME/.var/app/*/cache/*", "Error removing flatpaks cache folder, folder not found");
            systemcommand_asroot("rm -rf $HOME/.cache/*", "Error removing cache folder, folder not found");
            systemcommand_asroot("journalctl --vacuum-time=2d", "Error to limiting systemd logs to 2 days");
            systemcommand_asroot("journalctl --vacuum-size=500M", "Error limiting systemd logs to 500M");
            exit(0);

        },

        "fedora" => {

            systemcommand_asroot("dnf clean all", "Error to clean dnf cache");
            systemcommand_asroot("dnf autoremove -y", "Error removing orphaned dnf packages");
            systemcommand_asroot("flatpak uninstall --unused", "Error to cleaning unused flatpaks");
            systemcommand_asroot("rm -rf /var/lib/systemd/coredump/", "Error to remove folder: /var/lib/systemd/coredump/, folder not found");
            systemcommand_asroot("rm -rf $HOME/.var/app/*/cache/*", "Error removing flatpaks cache folder, folder not found");
            systemcommand_asroot("rm -rf $HOME/.cache/*", "Error removing cache folder, folder not found");
            systemcommand_asroot("journalctl --vacuum-time=2d", "Error to limiting systemd logs to 2 days");
            systemcommand_asroot("journalctl --vacuum-size=500M", "Error limiting systemd logs to 500M");
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

                    systemcommand_asroot("pacman -Syu xorg lxde lightdm lightdm-gtk-greeter adwaita-icon-theme xarchiver mousepad totem atril gnome-screenshot gnome-disk-utility bluez transmission-gtk --noconfirm", "Error installing minimal lxde on archlinux");
                    systemcommand_asroot("systemctl enable lightdm -f", "Error enabling lightdm on startup");

                },

                "lxqt" => {

                    systemcommand_asroot("pacman -Syu xorg lxqt lightdm lightdm-gtk-greeter adwaita-icon-theme xarchiver vlc --noconfirm", "Error installing minimal lxqt on archlinux");
                    systemcommand_asroot("systemctl enable lightdm -f", "Error enabling lightdm on startup");

                },

                "xfce" => {

                    systemcommand_asroot("pacman -Syu xorg lightdm lightdm-gtk-greeter xfce4-settings xfce4-pulseaudio-plugin adwaita-icon-theme exo garcon tumbler xfce4-panel xfce4-session xfce4-whiskermenu-plugin xfce4-terminal xfconf xfdesktop xfwm4 thunar file-roller --noconfirm", "Error installing xfce minimal on archlinux");
                    systemcommand_asroot("systemctl enable lightdm -f", "Error enabling lightdm on startup");

                },

                "gnome" => {

                    systemcommand_asroot("pacman -Syu gdm weston gnome-session gnome-terminal nautilus file-roller gnome-control-center gedit adwaita-icon-theme eog evince seahorse --noconfirm", "Error installing gnome minimal on archlinux");
                    systemcommand_asroot("systemctl enable gdm -f", "Error enabling gdm on startup");
                    systemcommand_asuser("gsettings", "set org.gnome.desktop.interface enable-animations false", "Error to disable animations on gnome");

                },

                "cinnamon" => {

                    systemcommand_asroot("pacman -Syu xorg lightdm lightdm-gtk-greeter cinnamon cinnamon-session cinnamon-desktop gnome-terminal cinnamon-control-center cinnamon-menus cinnamon-screensaver cinnamon-settings-daemon cinnamon-translations adwaita-icon-theme cjs muffin nemo nemo-fileroller file-roller --noconfirm", "Error installing cinnamon minimal on archlinux");
                    systemcommand_asroot("systemctl enable lightdm -f", "Error enabling lightdm on startup");

                },

                "mate" => {

                    systemcommand_asroot("pacman -Syu xorg lightdm lightdm-gtk-greeter mate-desktop adwaita-icon-theme mate-control-center mate-power-manager mate-screensaver mate-common mate-session-manager mate-settings-daemon mate-terminal network-manager-applet mate-panel marco caja --noconfirm", "Error installing minimal mate on archlinux");
                    systemcommand_asroot("systemctl enable lightdm -f", "Error enabling lightdm on startup");

                },

                "kdeplasma" => {

                    systemcommand_asroot("pacman -Syu weston sddm plasma-desktop plasma-nm konsole plasma-wayland-session kcm-fcitx kscreen ksysguard adwaita-icon-theme spectacle dolphin discover --noconfirm", "Error installing cinnamon minimal on archlinux");
                    systemcommand_asroot("systemctl enable sddm -f", "Error enabling lightdm on startup");

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

                    systemcommand_asroot("apt install xorg lightdm lightdm-gtk-greeter lxde-core lxterminal deluge file-roller mousepad gpicview gnome-disk-utility evince lxappearance pavucontrol lxsession-default-apps lxinput menu gnome-system-tools connman connman-gtk xscreensaver policykit-1 policykit-1-gnome xarchiver -y", "Error installing minimal lxde on debian 11");
                    systemcommand_asroot("systemctl enable lightdm -f", "Error enabling lightdm on startup");

                },

                "lxqt" => {

                    systemcommand_asroot("apt install xorg lightdm lightdm-gtk-greeter lxqt-core vlc ark ktorrent connman partitionmanager qpdfview pavucontrol -y", "Error installing lxqt minimal on debian 11");
                    systemcommand_asroot("systemctl enable lightdm -f", "Error enabling lightdm on startup");

                },

                "xfce" => {

                    systemcommand_asroot("apt install xorg lightdm lightdm-gtk-greeter thunar xfce4-panel xfce4-pulseaudio-plugin xfce4-whiskermenu-plugin xfce4-session xfce4-settings xfce4-terminal pavucontrol mousepad thunar-archive-plugin evince xfconf xfdesktop4 xfwm4 adwaita-qt qt5ct xfce4-taskmanager xfce4-screenshooter --no-install-recommends -y", "Error installing xfce4 minimal on debian 11");
                    systemcommand_asroot("apt install vlc gnome-disk-utility xarchiver ristretto transmission -y", "Error to install more xfce4 packages in debian 11");
                    systemcommand_asroot("systemctl enable lightdm -f", "Error enabling lightdm on startup");

                },

                "gnome" => {

                    systemcommand_asroot("apt install weston gdm3 gnome-session gnome-control-center gnome-software eog totem evince gedit gnome-terminal gnome-tweaks nautilus adwaita-icon-theme seahorse gnome-system-monitor gnome-screenshot file-roller transmission-gtk gnome-disk-utility --no-install-recommends -y", "Error installing gnome minimal on debian 11");
                    systemcommand_asroot("systemctl enable gdm -f", "Error enabling gdm3 on startup");
                    systemcommand_asuser("gsettings", "set org.gnome.desktop.interface enable-animations false", "Error to disable animations on gnome");

                },

                "cinnamon" => {

                    systemcommand_asroot("apt install xorg lightdm lightdm-gtk-greeter cinnamon-core gnome-terminal eog totem evince gedit gnome-system-monitor gnome-screenshot file-roller transmission-gtk gnome-disk-utility --no-install-recommends -y", "Error installing cinnamon minimal on debian 11");
                    systemcommand_asroot("systemctl enable lightdm -f", "Error enabling lightdm on startup");

                },

                "mate" => {

                    systemcommand_asroot("apt install xorg lightdm lightdm-gtk-greeter mate-desktop-environment gnome-disk-utility transmission-gtk file-roller totem -y", "Error installing minimal mate on debian 11");
                    systemcommand_asroot("systemctl enable lightdm -f", "Error enabling lightdm on startup");

                },

                "kdeplasma" => {

                    systemcommand_asroot("apt install weston sddm kde-plasma-desktop plasma-nm plasma-workspace-wayland systemsettings dolphin kwrite ark okular plasma-discover konsole ktorrent kde-spectacle gwenview -y", "Error installing minimal kde plasma on debian 11");
                    systemcommand_asroot("systemctl enable sddm -f", "Error enabling sddm on startup");

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

                    systemcommand_asroot("dnf install lightdm lightdm-gtk-greeter lxde-common lxdm openbox lxappearance lxsession lxterminal pcmanfm lxinput lxmenu-data lxpanel lxpolkit lxrandr lxtask xcompmgr xarchiver obconf network-manager-applet -y", "Error installing minimal lxde on fedora 35");
                    systemcommand_asroot("systemctl enable lightdm -f", "Error enabling lightdm on startup");
                    

                },

                "lxqt" => {
                    
                    systemcommand_asroot("dnf install lightdm lightdm-gtk-greeter lxqt-about lxqt-archiver lxqt-config lxqt-notificationd lxqt-openssh-askpass lxqt-panel breeze-cursor-theme breeze-gtk breeze-icon-theme firewall-config network-manager-applet notification-daemon obconf openbox pcmanfm-qt qterminal lxqt-policykit lxqt-powermanagement lxqt-qtplugin lxqt-session lxqt-themes lxqt-themes-fedora -y", "Error installing lxqt minimal on fedora 35");
                    systemcommand_asroot("systemctl enable lightdm -f", "Error enabling lightdm on startup");
                    systemcommand_asroot("systemctl set-default graphical.target", "Error enabling graphical mode boot");

                },

                "xfce" => {

                    systemcommand_asroot("dnf install lightdm lightdm-gtk-greeter xfwm4 xfce4-session xfdesktop xfce4-settings xfce4-terminal xfce4-whiskermenu-plugin xfce4-power-manager network-manager-applet -y", "Error installing xfce4 minimal on fedora 35");
                    systemcommand_asroot("systemctl enable lightdm -f", "Error enabling lightdm on startup");
                    systemcommand_asroot("systemctl set-default graphical.target", "Error enabling graphical mode boot");

                },

                "gnome" => {

                    systemcommand_asroot("dnf install gdm gnome-shell nautilus gnome-terminal fedora-workstation-backgrounds file-roller gnome-terminal-nautilus seahorse -y", "Error installing gnome on fedora 35");
                    systemcommand_asroot("systemctl enable gdm -f", "Error enabling gdm on startup");
                    Command::new("gsettings").args("set org.gnome.desktop.interface enable-animations false".split_ascii_whitespace()).status().expect("Error to disable animations on gnome");
                    systemcommand_asroot("systemctl set-default graphical.target", "Error enabling graphical mode boot");

                },

                "cinnamon" => {

                    systemcommand_asroot("dnf install lightdm lightdm-gtk-greeter cinnamon cinnamon-desktop cinnamon-session cinnamon-menus cinnamon-screensaver gnome-terminal cinnamon-translations muffin cinnamon-control-center cjs nemo nemo-fileroller -y", "Error installing cinnamon on fedora 35");
                    systemcommand_asroot("systemctl enable lightdm -f", "Error enabling lightdm on startup");
                    systemcommand_asroot("systemctl set-default graphical.target", "Error enabling graphical mode boot");

                },

                "mate" => {

                    systemcommand_asroot("dnf install lightdm lightdm-gtk-greeter mate-desktop mate-control-center mate-screensaver mate-power-manager mate-screenshot mate-session-manager mate-settings-daemon mate-terminal mate-panel marco caja network-manager-applet -y", "Error installing mate in fedora 35");
                    systemcommand_asroot("systemctl enable lightdm -f", "Error enabling lightdm on startup");
                    systemcommand_asroot("systemctl set-default graphical.target", "Error enabling graphical mode boot");

                },

                "kdeplasma" => {

                    systemcommand_asroot("dnf install sddm plasma-desktop plasma-nm konsole plasma-discover dolphin kscreen ksysguard spectacle plasma-user-manager kcm_colors kcm-fcitx -y", "Error installing kde plasma on fedora 35");
                    systemcommand_asroot("systemctl enable sddm -f", "Error enabling sddm on startup");
                    systemcommand_asroot("systemctl set-default graphical.target", "Error enabling graphical mode boot");

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

            systemcommand_asroot("rm -r /etc/dnf/dnf.conf", "Error to remove /etc/dnf/dnf.conf");
            systemcommand_asroot(r#"echo "[main]" >> /etc/dnf/dnf.conf"#, "Error writing to /etc/dnf/dnf.conf file");
            systemcommand_asroot(r#"echo "gpgcheck=1" >> /etc/dnf/dnf.conf"#, "Error writing to /etc/dnf/dnf.conf file");
            systemcommand_asroot(r#"echo "installonly_limit=3" >> /etc/dnf/dnf.conf"#, "Error writing to /etc/dnf/dnf.conf file");
            systemcommand_asroot(r#"echo "clean_requirements_on_remove=True" >> /etc/dnf/dnf.conf"#, "Error writing to /etc/dnf/dnf.conf file");
            systemcommand_asroot(r#"echo "best=False" >> /etc/dnf/dnf.conf"#, "Error writing to /etc/dnf/dnf.conf file");
            systemcommand_asroot(r#"echo "skip_if_unavailable=True" >> /etc/dnf/dnf.conf"#, "Error writing to /etc/dnf/dnf.conf file");
            systemcommand_asroot(r#"echo "fastestmirror=True" >> /etc/dnf/dnf.conf"#, "Error writing to /etc/dnf/dnf.conf file");
            systemcommand_asroot(r#"echo "max_parallel_downloads=7" >> /etc/dnf/dnf.conf"#, "Error writing to /etc/dnf/dnf.conf file");
            systemcommand_asroot(r#"echo "defaultyes=True" >> /etc/dnf/dnf.conf"#, "Error writing to /etc/dnf/dnf.conf file");
            systemcommand_asroot(r#"echo "install_weak_deps=false" >> /etc/dnf/dnf.conf"#, "Error writing to /etc/dnf/dnf.conf file");

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
