pub const ALL_PACKAGES_TO_REMOVE_ARCHLINUX: &str = "lxde lightdm lightdm-gtk-greeter adwaita-icon-theme xarchiver lxqt xfce4-settings xfce4-pulseaudio-plugin exo garcon tumbler xfce4-panel xfce4-session xfce4-whiskermenu-plugin xfce4-terminal xfconf xfdesktop xfwm4 thunar file-roller gdm weston gnome-session gnome-terminal nautilus-terminal nautilus gnome-control-center gedit eog evince cinnamon cinnamon-session cinnamon-desktop gnome-terminal cinnamon-control-center cinnamon-menus cinnamon-screensaver cinnamon-settings-daemon cinnamon-translations cjs muffin nemo nemo-fileroller mate-control-center mate-desktop mate-power-manager mate-screensaver mate-common mate-session-manager mate-settings-daemon mate-terminal mate-panel marco caja sddm plasma-desktop plasma-nm konsole plasma-wayland-session kcm-fcitx kscreen ksysguard spectacle dolphin discover cutefish";
pub const ALL_PACKAGES_TO_INSTALL_ARCHLINUX_LXDE: &str = "lxde lightdm lightdm-gtk-greeter adwaita-icon-theme xarchiver";
pub const ALL_PACKAGES_TO_INSTALL_ARCHLINUX_LXQT: &str = "lxqt lightdm lightdm-gtk-greeter xfce4-settings xfce4-pulseaudio-plugin adwaita-icon-theme exo garcon tumbler xfce4-panel xfce4-session xfce4-whiskermenu-plugin xfce4-terminal xfconf xfdesktop xfwm4 thunar file-roller";
pub const ALL_PACKAGES_TO_INSTALL_ARCHLINUX_XFCE: &str = "lightdm lightdm-gtk-greeter xfce4-settings xfce4-pulseaudio-plugin adwaita-icon-theme exo garcon tumbler xfce4-panel xfce4-session xfce4-whiskermenu-plugin xfce4-terminal xfconf xfdesktop xfwm4 thunar file-roller";
pub const ALL_PACKAGES_TO_INSTALL_ARCHLINUX_GNOME: &str = "gdm weston gnome-session gnome-terminal nautilus file-roller gnome-control-center gedit adwaita-icon-theme eog evince seahorse";
pub const ALL_PACKAGES_TO_INSTALL_ARCHLINUX_CINNAMON: &str = "lightdm lightdm-gtk-greeter cinnamon cinnamon-session cinnamon-desktop gnome-terminal cinnamon-control-center cinnamon-menus cinnamon-screensaver cinnamon-settings-daemon cinnamon-translations adwaita-icon-theme cjs muffin nemo nemo-fileroller file-roller";
pub const ALL_PACKAGES_TO_INSTALL_ARCHLINUX_MATE: &str = "lightdm lightdm-gtk-greeter mate-desktop adwaita-icon-theme mate-control-center mate-power-manager mate-screensaver mate-common mate-session-manager mate-settings-daemon mate-terminal network-manager-applet mate-panel marco caja";
pub const ALL_PACKAGES_TO_INSTALL_ARCHLINUX_KDEPLASMA: &str = "sddm plasma-desktop plasma-nm konsole plasma-wayland-session kcm-fcitx kscreen ksysguard adwaita-icon-theme spectacle dolphin discover";
pub const ALL_PACKAGES_TO_INSTALL_ARCHLINUX_BSPWM: &str = "";
pub const ALL_PACKAGES_TO_INSTALL_ARCHLINUX_CUTEFISH: &str = "";

pub const ALL_PACKAGES_TO_REMOVE_DEBIAN: &str = "lightdm lightdm-gtk-greeter lxde-core lxterminal deluge file-roller mousepad gpicview gnome-disk-utility evince lxappearance pavucontrol lxsession-default-apps lxinput menu gnome-system-tools connman connman-gtk xscreensaver policykit-1 policykit-1-gnome xarchiver lxqt-core vlc ark ktorrent partitionmanager qpdfview thunar xfce4-panel xfce4-pulseaudio-plugin xfce4-whiskermenu-plugin xfce4-session xfce4-settings xfce4-terminal  thunar-archive-plugin xfconf xfdesktop4 xfwm4 adwaita-qt qt5ct xfce4-taskmanager xfce4-screenshooter gdm3 gnome-session gnome-control-center gnome-software eog totem gedit gnome-terminal gnome-tweaks nautilus adwaita-icon-theme seahorse gnome-system-monitor gnome-screenshot transmission-gtk cinnamon-core mate-desktop-environment sddm kde-plasma-desktop plasma-nm plasma-workspace-wayland systemsettings dolphin kwrite okular plasma-discover konsole kde-spectacle gwenview";
pub const ALL_PACKAGES_TO_INSTALL_DEBIAN_LXDE: &str = "lightdm lightdm-gtk-greeter lxde-core lxterminal deluge file-roller mousepad gpicview gnome-disk-utility evince lxappearance pavucontrol lxsession-default-apps lxinput menu gnome-system-tools connman connman-gtk xscreensaver policykit-1 policykit-1-gnome xarchiver";
pub const ALL_PACKAGES_TO_INSTALL_DEBIAN_LXQT: &str = "lightdm lightdm-gtk-greeter lxqt-core vlc ark ktorrent connman partitionmanager qpdfview pavucontrol";
pub const ALL_PACKAGES_TO_INSTALL_DEBIAN_XFCE: &str = "lightdm lightdm-gtk-greeter thunar xfce4-panel xfce4-pulseaudio-plugin xfce4-whiskermenu-plugin xfce4-session xfce4-settings xfce4-terminal pavucontrol mousepad thunar-archive-plugin evince xfconf xfdesktop4 xfwm4 adwaita-qt qt5ct xfce4-taskmanager xfce4-screenshooter";
pub const ALL_PACKAGES_TO_INSTALL_DEBIAN_GNOME: &str = "gdm3 gnome-session gnome-control-center gnome-software eog totem evince gedit gnome-terminal gnome-tweaks nautilus adwaita-icon-theme seahorse gnome-system-monitor gnome-screenshot file-roller transmission-gtk gnome-disk-utility";
pub const ALL_PACKAGES_TO_INSTALL_DEBIAN_CINNAMON: &str = "lightdm lightdm-gtk-greeter cinnamon-core gnome-terminal eog totem evince gedit gnome-system-monitor gnome-screenshot file-roller transmission-gtk gnome-disk-utility";
pub const ALL_PACKAGES_TO_INSTALL_DEBIAN_MATE: &str = "lightdm lightdm-gtk-greeter mate-desktop-environment gnome-disk-utility transmission-gtk file-roller totem";
pub const ALL_PACKAGES_TO_INSTALL_DEBIAN_KDEPLASMA: &str = "sddm kde-plasma-desktop plasma-nm plasma-workspace-wayland systemsettings dolphin kwrite ark okular plasma-discover konsole ktorrent kde-spectacle gwenview";
pub const ALL_PACKAGES_TO_INSTALL_DEBIAN_BSPWM: &str = "";
pub const ALL_PACKAGES_TO_INSTALL_DEBIAN_CUTEFISH: &str = "";

pub const ALL_PACKAGES_TO_REMOVE_FEDORA: &str = "@lxde-desktop @plasma-desktop @gnome-desktop @cinnamon-desktop @lxqt-desktop @mate-desktop @xfce-desktop lxappearance lxde-common lxdm lxinput lxmenu-data lxpanel lxpolkit lxrandr xcompmgr xarchiver lxsession lxtask pcmanfm lxterminal network-manager-applet openbox obconf lightdm-gtk-greeter lightdm sddm plasma-desktop plasma-nm konsole kcm_colors kcm-fcitx kscreen ksysguard spectacle plasma-user-manager dolphin plasma-discover gdm gnome-shell nautilus gnome-terminal fedora-workstation-backgrounds file-roller gnome-terminal-nautilus cinnamon cinnamon-control-center cinnamon-desktop cinnamon-menus cinnamon-screensaver cinnamon-session nemo nemo-fileroller cinnamon-translations cjs muffin gnome-terminal breeze-cursor-theme breeze-gtk breeze-icon-theme firewall-config network-manager-applet notification-daemon obconf openbox pcmanfm-qt qterminal lxqt-about lxqt-archiver lxqt-config lxqt-notificationd lxqt-openssh-askpass lxqt-panel lxqt-policykit lxqt-powermanagement lxqt-qtplugin lxqt-session lxqt-themes lxqt-themes-fedora network-manager-applet xfwm4 xfce4-power-manager xfce4-session xfce4-settings xfce4-whiskermenu-plugin xfdesktop xfce4-terminal mate-control-center mate-desktop mate-power-manager mate-screensaver mate-screenshot mate-session-manager mate-settings-daemon mate-terminal network-manager-applet mate-panel marco caja gstreamer1-plugins-base gstreamer1-plugins-good gstreamer1-plugins-ugly gstreamer1-plugins-bad-free gstreamer1-plugins-bad-free gstreamer1-plugins-bad-freeworld gstreamer1-plugins-bad-free-extras ffmpeg";
pub const ALL_PACKAGES_TO_INSTALL_FEDORA_LXDE: &str = "lightdm lightdm-gtk-greeter lxde-common lxdm openbox lxappearance lxsession lxterminal pcmanfm lxinput lxmenu-data lxpanel lxpolkit lxrandr lxtask xcompmgr xarchiver obconf network-manager-applet";
pub const ALL_PACKAGES_TO_INSTALL_FEDORA_LXQT: &str = "lightdm lightdm-gtk-greeter lxqt-about lxqt-archiver lxqt-config lxqt-notificationd lxqt-openssh-askpass lxqt-panel breeze-cursor-theme breeze-gtk breeze-icon-theme firewall-config network-manager-applet notification-daemon obconf openbox pcmanfm-qt qterminal lxqt-policykit lxqt-powermanagement lxqt-qtplugin lxqt-session lxqt-themes lxqt-themes-fedora";
pub const ALL_PACKAGES_TO_INSTALL_FEDORA_XFCE: &str = "lightdm lightdm-gtk-greeter xfwm4 xfce4-session xfdesktop xfce4-settings xfce4-terminal xfce4-whiskermenu-plugin xfce4-power-manager network-manager-applet";
pub const ALL_PACKAGES_TO_INSTALL_FEDORA_GNOME: &str = "gdm gnome-shell nautilus gnome-terminal fedora-workstation-backgrounds file-roller gnome-terminal-nautilus seahorse";
pub const ALL_PACKAGES_TO_INSTALL_FEDORA_CINNAMON: &str = "lightdm lightdm-gtk-greeter cinnamon cinnamon-desktop cinnamon-session cinnamon-menus cinnamon-screensaver gnome-terminal cinnamon-translations muffin cinnamon-control-center cjs nemo nemo-fileroller";
pub const ALL_PACKAGES_TO_INSTALL_FEDORA_MATE: &str = "lightdm lightdm-gtk-greeter mate-desktop mate-control-center mate-screensaver mate-power-manager mate-screenshot mate-session-manager mate-settings-daemon mate-terminal mate-panel marco caja network-manager-applet";
pub const ALL_PACKAGES_TO_INSTALL_FEDORA_KDEPLASMA: &str = "sddm plasma-desktop plasma-nm konsole plasma-discover dolphin kscreen ksysguard spectacle plasma-user-manager kcm_colors kcm-fcitx";
pub const ALL_PACKAGES_TO_INSTALL_FEDORA_BSPWM: &str = "";
pub const ALL_PACKAGES_TO_INSTALL_FEDORA_CUTEFISH: &str = "";

pub const HELP_EN_US: &str = "

Use mild --[option]-[distro]-[interface]

    --install-arch-lxde => Install Minimal Lxde Desktop on ArchLinux.

    --install-arch-lxqt => Install Minimal Lxqt Desktop on ArchLinux.

    --install-arch-xfce => Install Minimal Xfce4 Desktop on ArchLinux.

    --install-arch-gnome => Install Minimal Gnome Desktop on ArchLinux.

    --install-arch-cinnamon => Install Minimal Cinnamon Desktop on ArchLinux.

    --install-arch-mate => Install Minimal Mate Desktop on ArchLinux.

    --install-arch-kdeplasma => Install Minimal Kde Plasma Desktop on ArchLinux.

    --install-arch-bspwm => Install Minimal Bspwm Desktop on ArchLinux.

    --install-arch-cutefish => Install Minimal Cutefish Desktop on ArchLinux.

    --clean-arch => Cleans up ArchLinux by removing cache and orphaned packages from the system
    

    --install-debian-lxde => Install Minimal Lxde Desktop on Debian 11.

    --install-debian-lxqt => Install Minimal Lxqt Desktop on Debian 11.

    --install-debian-xfce => Install Minimal Xfce4 Desktop on Debian 11.

    --install-debian-gnome => Install Minimal Gnome Desktop on Debian 11.

    --install-debian-cinnamon => Install Minimal Cinnamon Desktop on Debian 11.

    --install-debian-mate => Install Minimal Mate Desktop on Debian 11.

    --install-debian-kdeplasma => Install Minimal Kde Plasma Desktop on Debian 11.

    --install-debian-bspwm => Install Minimal Bspwm Desktop on Debian 11.

    --install-debian-cutefish => Install Minimal Cutefish Desktop on Debian 11.

    --clean-debian => Cleans up Debian 11 by removing cache and orphaned packages from the system


    --install-fedora-lxde => Install Minimal Lxde Desktop on Fedora 35.

    --install-fedora-lxqt => Install Minimal Lxqt Desktop on Fedora 35.

    --install-fedora-xfce => Install Minimal Xfce4 Desktop on Fedora 35.

    --install-fedora-gnome => Install Minimal Gnome Desktop on Fedora 35.

    --install-fedora-cinnamon => Install Minimal Cinnamon Desktop on Fedora 35.

    --install-fedora-mate => Install Minimal Mate Desktop on Fedora 35.

    --install-fedora-kdeplasma => Install Minimal Kde Plasma Desktop on Fedora 35.

    --install-fedora-bspwm => Install Minimal Bspwm Desktop on Fedora 35.

    --install-fedora-cutefish => Install Minimal Cutefish Desktop on Fedora 35.

    --clean-fedora => Cleans up Fedora 35 by removing cache and orphaned packages from the system.


    --help => View a usage guide.

";