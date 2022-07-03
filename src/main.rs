use std::{
    process::{
        exit // Importing the standard exit library to exit the program
    },
    env // Importing the standard env library to capture user arguments
};
mod texts;
mod utils;

fn main() {
    const ALL_PACKAGES_TO_REMOVE_ARCHLINUX: &str = "lxde lightdm lightdm-gtk-greeter adwaita-icon-theme xarchiver lxqt xfce4-settings xfce4-pulseaudio-plugin exo garcon tumbler xfce4-panel xfce4-session xfce4-whiskermenu-plugin xfce4-terminal xfconf xfdesktop xfwm4 thunar file-roller gdm weston gnome-session gnome-terminal nautilus-terminal nautilus gnome-control-center gedit eog evince cinnamon cinnamon-session cinnamon-desktop gnome-terminal cinnamon-control-center cinnamon-menus cinnamon-screensaver cinnamon-settings-daemon cinnamon-translations cjs muffin nemo nemo-fileroller mate-control-center mate-desktop mate-power-manager mate-screensaver mate-common mate-session-manager mate-settings-daemon mate-terminal mate-panel marco caja sddm plasma-desktop plasma-nm konsole plasma-wayland-session kcm-fcitx kscreen ksysguard spectacle dolphin discover cutefish";
    const ALL_PACKAGES_TO_REMOVE_DEBIAN: &str = "lightdm lightdm-gtk-greeter lxde-core lxterminal deluge file-roller mousepad gpicview gnome-disk-utility evince lxappearance pavucontrol lxsession-default-apps lxinput menu gnome-system-tools connman connman-gtk xscreensaver policykit-1 policykit-1-gnome xarchiver lxqt-core vlc ark ktorrent partitionmanager qpdfview thunar xfce4-panel xfce4-pulseaudio-plugin xfce4-whiskermenu-plugin xfce4-session xfce4-settings xfce4-terminal  thunar-archive-plugin xfconf xfdesktop4 xfwm4 adwaita-qt qt5ct xfce4-taskmanager xfce4-screenshooter gdm3 gnome-session gnome-control-center gnome-software eog totem gedit gnome-terminal gnome-tweaks nautilus adwaita-icon-theme seahorse gnome-system-monitor gnome-screenshot transmission-gtk cinnamon-core mate-desktop-environment sddm kde-plasma-desktop plasma-nm plasma-workspace-wayland systemsettings dolphin kwrite okular plasma-discover konsole kde-spectacle gwenview";
    const ALL_PACKAGES_TO_REMOVE_FEDORA: &str = "@lxde-desktop @plasma-desktop @gnome-desktop @cinnamon-desktop @lxqt-desktop @mate-desktop @xfce-desktop lxappearance lxde-common lxdm lxinput lxmenu-data lxpanel lxpolkit lxrandr xcompmgr xarchiver lxsession lxtask pcmanfm lxterminal network-manager-applet openbox obconf lightdm-gtk-greeter lightdm sddm plasma-desktop plasma-nm konsole kcm_colors kcm-fcitx kscreen ksysguard spectacle plasma-user-manager dolphin plasma-discover gdm gnome-shell nautilus gnome-terminal fedora-workstation-backgrounds file-roller gnome-terminal-nautilus cinnamon cinnamon-control-center cinnamon-desktop cinnamon-menus cinnamon-screensaver cinnamon-session nemo nemo-fileroller cinnamon-translations cjs muffin gnome-terminal breeze-cursor-theme breeze-gtk breeze-icon-theme firewall-config network-manager-applet notification-daemon obconf openbox pcmanfm-qt qterminal lxqt-about lxqt-archiver lxqt-config lxqt-notificationd lxqt-openssh-askpass lxqt-panel lxqt-policykit lxqt-powermanagement lxqt-qtplugin lxqt-session lxqt-themes lxqt-themes-fedora network-manager-applet xfwm4 xfce4-power-manager xfce4-session xfce4-settings xfce4-whiskermenu-plugin xfdesktop xfce4-terminal mate-control-center mate-desktop mate-power-manager mate-screensaver mate-screenshot mate-session-manager mate-settings-daemon mate-terminal network-manager-applet mate-panel marco caja gstreamer1-plugins-base gstreamer1-plugins-good gstreamer1-plugins-ugly gstreamer1-plugins-bad-free gstreamer1-plugins-bad-free gstreamer1-plugins-bad-freeworld gstreamer1-plugins-bad-free-extras ffmpeg";
    let args: Vec<String> = env::args().collect();
    let option = &args[1].trim();

    match &option[..] {

        "--install-arch-lxde" => {
            utils::show_the_changes_that_will_be_made_to_user(ALL_PACKAGES_TO_REMOVE_ARCHLINUX, "lxde lightdm lightdm-gtk-greeter adwaita-icon-theme xarchiver");
            utils::exec_installation("archlinux","lxde");
            exit(0);
        },

        "--install-arch-lxqt" => {
            utils::show_the_changes_that_will_be_made_to_user(ALL_PACKAGES_TO_REMOVE_ARCHLINUX, "lxqt lightdm lightdm-gtk-greeter xfce4-settings xfce4-pulseaudio-plugin adwaita-icon-theme exo garcon tumbler xfce4-panel xfce4-session xfce4-whiskermenu-plugin xfce4-terminal xfconf xfdesktop xfwm4 thunar file-roller");
            utils::exec_installation("archlinux","lxqt");
            exit(0);
        },

        "--install-arch-xfce" => {
            utils::show_the_changes_that_will_be_made_to_user(ALL_PACKAGES_TO_REMOVE_ARCHLINUX, "lightdm lightdm-gtk-greeter xfce4-settings xfce4-pulseaudio-plugin adwaita-icon-theme exo garcon tumbler xfce4-panel xfce4-session xfce4-whiskermenu-plugin xfce4-terminal xfconf xfdesktop xfwm4 thunar file-roller");
            utils::exec_installation("archlinux","xfce");
            exit(0);
        },

        "--install-arch-gnome" => {
            utils::show_the_changes_that_will_be_made_to_user(ALL_PACKAGES_TO_REMOVE_ARCHLINUX, "gdm weston gnome-session gnome-terminal nautilus file-roller gnome-control-center gedit adwaita-icon-theme eog evince seahorse");
            utils::exec_installation("archlinux","gnome");
            exit(0);
        },

        "--install-arch-cinnamon" => {
            utils::show_the_changes_that_will_be_made_to_user(ALL_PACKAGES_TO_REMOVE_ARCHLINUX, "lightdm lightdm-gtk-greeter cinnamon cinnamon-session cinnamon-desktop gnome-terminal cinnamon-control-center cinnamon-menus cinnamon-screensaver cinnamon-settings-daemon cinnamon-translations adwaita-icon-theme cjs muffin nemo nemo-fileroller file-roller");
            utils::exec_installation("archlinux","cinnamon");
            exit(0);
        },

        "--install-arch-mate" => {
            utils::show_the_changes_that_will_be_made_to_user(ALL_PACKAGES_TO_REMOVE_ARCHLINUX, "lightdm lightdm-gtk-greeter mate-desktop adwaita-icon-theme mate-control-center mate-power-manager mate-screensaver mate-common mate-session-manager mate-settings-daemon mate-terminal network-manager-applet mate-panel marco caja");
            utils::exec_installation("archlinux","mate");
            exit(0);
        },

        "--install-arch-kdeplasma" => {
            utils::show_the_changes_that_will_be_made_to_user(ALL_PACKAGES_TO_REMOVE_ARCHLINUX, "sddm plasma-desktop plasma-nm konsole plasma-wayland-session kcm-fcitx kscreen ksysguard adwaita-icon-theme spectacle dolphin discover");
            utils::exec_installation("archlinux","kdeplasma");
            exit(0);
        },

        "--install-arch-bspwm" => {
            utils::show_the_changes_that_will_be_made_to_user(ALL_PACKAGES_TO_REMOVE_ARCHLINUX, "");
            utils::exec_installation("archlinux","bspwm");
            exit(0);
        },

        "--install-arch-cutefish" => {
            utils::show_the_changes_that_will_be_made_to_user(ALL_PACKAGES_TO_REMOVE_ARCHLINUX, "");
            utils::exec_installation("archlinux","cutefish");
            exit(0);
        },

        "--clean-arch" => utils::clean_system("archlinux"),


        "--install-debian-lxde" => {
            utils::show_the_changes_that_will_be_made_to_user(ALL_PACKAGES_TO_REMOVE_DEBIAN, "lightdm lightdm-gtk-greeter lxde-core lxterminal deluge file-roller mousepad gpicview gnome-disk-utility evince lxappearance pavucontrol lxsession-default-apps lxinput menu gnome-system-tools connman connman-gtk xscreensaver policykit-1 policykit-1-gnome xarchiver");
            utils::exec_installation("debian","lxde");
            exit(0);
        },

        "--install-debian-lxqt" => {
            utils::show_the_changes_that_will_be_made_to_user(ALL_PACKAGES_TO_REMOVE_DEBIAN, "lightdm lightdm-gtk-greeter lxqt-core vlc ark ktorrent connman partitionmanager qpdfview pavucontrol");
            utils::exec_installation("debian","lxqt");
            exit(0);
        },

        "--install-debian-xfce" => {
            utils::show_the_changes_that_will_be_made_to_user(ALL_PACKAGES_TO_REMOVE_DEBIAN, "lightdm lightdm-gtk-greeter thunar xfce4-panel xfce4-pulseaudio-plugin xfce4-whiskermenu-plugin xfce4-session xfce4-settings xfce4-terminal pavucontrol mousepad thunar-archive-plugin evince xfconf xfdesktop4 xfwm4 adwaita-qt qt5ct xfce4-taskmanager xfce4-screenshooter");
            utils::exec_installation("debian","xfce");
            exit(0);
        },

        "--install-debian-gnome" => {
            utils::show_the_changes_that_will_be_made_to_user(ALL_PACKAGES_TO_REMOVE_DEBIAN, "gdm3 gnome-session gnome-control-center gnome-software eog totem evince gedit gnome-terminal gnome-tweaks nautilus adwaita-icon-theme seahorse gnome-system-monitor gnome-screenshot file-roller transmission-gtk gnome-disk-utility");
            utils::exec_installation("debian","gnome");
            exit(0);
        },

        "--install-debian-cinnamon" => {
            utils::show_the_changes_that_will_be_made_to_user(ALL_PACKAGES_TO_REMOVE_DEBIAN, "lightdm lightdm-gtk-greeter cinnamon-core gnome-terminal eog totem evince gedit gnome-system-monitor gnome-screenshot file-roller transmission-gtk gnome-disk-utility");
            utils::exec_installation("debian","cinnamon");
            exit(0);
        },

        "--install-debian-mate" => {
            utils::show_the_changes_that_will_be_made_to_user(ALL_PACKAGES_TO_REMOVE_DEBIAN, "lightdm lightdm-gtk-greeter mate-desktop-environment gnome-disk-utility transmission-gtk file-roller totem");
            utils::exec_installation("debian","mate");
            exit(0);
        },

        "--install-debian-kdeplasma" => {
            utils::show_the_changes_that_will_be_made_to_user(ALL_PACKAGES_TO_REMOVE_DEBIAN, "sddm kde-plasma-desktop plasma-nm plasma-workspace-wayland systemsettings dolphin kwrite ark okular plasma-discover konsole ktorrent kde-spectacle gwenview");
            utils::exec_installation("debian","kdeplasma");
            exit(0);
        },

        "--install-debian-bspwm" => {
            utils::show_the_changes_that_will_be_made_to_user(ALL_PACKAGES_TO_REMOVE_DEBIAN, "");
            utils::exec_installation("debian","bspwm");
            exit(0);
        },

        "--install-debian-cutefish" => {
            utils::show_the_changes_that_will_be_made_to_user(ALL_PACKAGES_TO_REMOVE_DEBIAN, "");
            utils::exec_installation("debian","cutefish");
            exit(0);
        },

        "--clean-debian" => utils::clean_system("debian"),


        "--install-fedora-lxde" => {
            utils::show_the_changes_that_will_be_made_to_user(ALL_PACKAGES_TO_REMOVE_FEDORA, "lightdm lightdm-gtk-greeter lxde-common lxdm openbox lxappearance lxsession lxterminal pcmanfm lxinput lxmenu-data lxpanel lxpolkit lxrandr lxtask xcompmgr xarchiver obconf network-manager-applet");
            utils::exec_installation("fedora","lxde");
            exit(0);
        },

        "--install-fedora-lxqt" => {
            utils::show_the_changes_that_will_be_made_to_user(ALL_PACKAGES_TO_REMOVE_FEDORA, "lightdm lightdm-gtk-greeter lxqt-about lxqt-archiver lxqt-config lxqt-notificationd lxqt-openssh-askpass lxqt-panel breeze-cursor-theme breeze-gtk breeze-icon-theme firewall-config network-manager-applet notification-daemon obconf openbox pcmanfm-qt qterminal lxqt-policykit lxqt-powermanagement lxqt-qtplugin lxqt-session lxqt-themes lxqt-themes-fedora");
            utils::exec_installation("fedora","lxqt");
            exit(0);
        },

        "--install-fedora-xfce" => {
            utils::show_the_changes_that_will_be_made_to_user(ALL_PACKAGES_TO_REMOVE_FEDORA, "lightdm lightdm-gtk-greeter xfwm4 xfce4-session xfdesktop xfce4-settings xfce4-terminal xfce4-whiskermenu-plugin xfce4-power-manager network-manager-applet");
            utils::exec_installation("fedora","xfce");
            exit(0);
        },

        "--install-fedora-gnome" => {
            utils::show_the_changes_that_will_be_made_to_user(ALL_PACKAGES_TO_REMOVE_FEDORA, "gdm gnome-shell nautilus gnome-terminal fedora-workstation-backgrounds file-roller gnome-terminal-nautilus seahorse");
            utils::exec_installation("fedora","gnome");
            exit(0);
        },

        "--install-fedora-cinnamon" => {
            utils::show_the_changes_that_will_be_made_to_user(ALL_PACKAGES_TO_REMOVE_FEDORA, "lightdm lightdm-gtk-greeter cinnamon cinnamon-desktop cinnamon-session cinnamon-menus cinnamon-screensaver gnome-terminal cinnamon-translations muffin cinnamon-control-center cjs nemo nemo-fileroller");
            utils::exec_installation("fedora","cinnamon");
            exit(0);
        },

        "--install-fedora-mate" => {
            utils::show_the_changes_that_will_be_made_to_user(ALL_PACKAGES_TO_REMOVE_FEDORA, "lightdm lightdm-gtk-greeter mate-desktop mate-control-center mate-screensaver mate-power-manager mate-screenshot mate-session-manager mate-settings-daemon mate-terminal mate-panel marco caja network-manager-applet");
            utils::exec_installation("fedora","mate");
            exit(0);
        },

        "--install-fedora-kdeplasma" => {
            utils::show_the_changes_that_will_be_made_to_user(ALL_PACKAGES_TO_REMOVE_FEDORA, "sddm plasma-desktop plasma-nm konsole plasma-discover dolphin kscreen ksysguard spectacle plasma-user-manager kcm_colors kcm-fcitx");
            utils::exec_installation("fedora","kdeplasma");
            exit(0);
        },

        "--install-fedora-bspwm" => {
            utils::show_the_changes_that_will_be_made_to_user(ALL_PACKAGES_TO_REMOVE_FEDORA, "");
            utils::exec_installation("fedora","bspwm");
            exit(0);
        },

        "--install-fedora-cutefish" => {
            utils::show_the_changes_that_will_be_made_to_user(ALL_PACKAGES_TO_REMOVE_FEDORA, "");
            utils::exec_installation("fedora","cutefish");
            exit(0);
        },

        "--clean-fedora" => utils::clean_system("fedora"),


        "--help" => {println!("{}",texts::HELP_EN_US);},

        _ => {println!("{}",texts::HELP_EN_US);}
    }
}