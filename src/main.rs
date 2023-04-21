use std::{
    process::exit,
    path::Path,
    env,
    fs
};

use colored::Colorize;
mod texts;
mod utils;

pub fn clean_systemd() {
    utils::remove_folder("/var/lib/systemd/coredump/"); /* Command To Clear SystemD Logs */
    utils::system_command("journalctl --vacuum-time=2d"); /* Command to Limit SystemD Logs to Up to 2 Days */
    utils::system_command("journalctl --vacuum-size=500M"); /* Command to Limit SystemD Logs to Up to 500MB */
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let option = &args[1].trim();

    pub const ALL_PACKAGES_TO_INSTALL_ARCHLINUX_LXDE: &str = "lxappearance lxappearance-obconf lxde-common lxde-icon-theme lxhotkey lxinput lxlauncher lxpanel lxrandr lxsession pcmanfm openbox";
    pub const ALL_PACKAGES_TO_INSTALL_ARCHLINUX_LXQT: &str = "lxqt-session lxqt-admin lxqt-config lxqt-globalkeys lxqt-notificationd lxqt-panel lxqt-policykit lxqt-powermanagement lxqt-themes obconf-qt openbox pavucontrol-qt pcmanfm-qt";
    pub const ALL_PACKAGES_TO_INSTALL_ARCHLINUX_XFCE4: &str = "xfce4-session exo garcon thunar thunar-volman tumbler xfce4-panel xfce4-power-manager xfce4-settings xfconf xfdesktop xfwm4 xfwm4-themes";
    pub const ALL_PACKAGES_TO_INSTALL_ARCHLINUX_GNOME: &str = "gnome-shell nautilus gnome-control-center gnome-text-editor";
    pub const ALL_PACKAGES_TO_INSTALL_ARCHLINUX_CINNAMON: &str = "cinnamon-session accountsservice cinnamon-control-center cinnamon-menus cinnamon-settings-daemon gnome-themes-extra gsound muffin nemo network-manager-applet polkit-gnome";
    pub const ALL_PACKAGES_TO_INSTALL_ARCHLINUX_MATE: &str = "mate-session-manager mate-media pulseaudio caja marco mate-control-center mate-desktop mate-icon-theme mate-menus mate-notification-daemon mate-panel mate-polkit mate-settings-daemon mate-themes";
    pub const ALL_PACKAGES_TO_INSTALL_ARCHLINUX_KDEPLASMA: &str = "plasma-pa breeze breeze-gtk kde-gtk-config kdecoration kdeplasma-addons khotkeys kmenuedit kpipewire kwallet-pam kwayland-integration kwin layer-shell-qt libkscreen libksysguard milou plasma-browser-integration plasma-desktop plasma-integration plasma-nm plasma-workspace polkit-kde-agent powerdevil sddm-kcm systemsettings xdg-desktop-portal-kde plasma-wayland-session kcm-fcitx kscreen dolphin";
    let all_packages_to_remove_archlinux: String = format!("{} {} {} {} {} {} {}", ALL_PACKAGES_TO_INSTALL_ARCHLINUX_LXDE.to_owned(), ALL_PACKAGES_TO_INSTALL_ARCHLINUX_LXQT.to_owned(), ALL_PACKAGES_TO_INSTALL_ARCHLINUX_XFCE4.to_owned(), ALL_PACKAGES_TO_INSTALL_ARCHLINUX_GNOME.to_owned(), ALL_PACKAGES_TO_INSTALL_ARCHLINUX_CINNAMON.to_owned(), ALL_PACKAGES_TO_INSTALL_ARCHLINUX_MATE.to_owned(), ALL_PACKAGES_TO_INSTALL_ARCHLINUX_KDEPLASMA.to_owned());
    let all_packets_do_archlinux_no_duplicate_packets = utils::remove_repeated_words(&all_packages_to_remove_archlinux);
    
    pub const ALL_PACKAGES_TO_INSTALL_DEBIAN_LXDE: &str = "adwaita-icon-theme lxappearance lxappearance-obconf gtk2-engines lxde-settings-daemon lxpanel pcmanfm lxde-icon-theme lxhotkey-gtk lxinput lxrandr lxsession-edit lxpolkit lx-session lxsession-default-apps xscreensaver policykit-1 --no-install-recommends";
    pub const ALL_PACKAGES_TO_INSTALL_DEBIAN_LXQT: &str = "papirus-icon-theme lxqt-config lxqt-globalkeys lxqt-notificationd lxqt-panel lxqt-policykit lxqt-qtplugin lxqt-session lxqt-system-theme pcmanfm-qt lxqt-powermanagement";
    pub const ALL_PACKAGES_TO_INSTALL_DEBIAN_XFCE4: &str = "adwaita-icon-theme xfce4-notifyd xfwm4 xfce4-whiskermenu-plugin xdg-user-dirs xfdesktop4 xfconf xfce4-settings xfce4-session xfce4-pulseaudio-plugin xfce4-panel libxfce4ui-utils thunar thunar-archive-plugin thunar-media-tags-plugin --no-install-recommends";
    pub const ALL_PACKAGES_TO_INSTALL_DEBIAN_GNOME: &str = "adwaita-icon-theme at-spi2-core caribou dconf-cli dconf-gsettings-backend evolution-data-server fonts-cantarell gkbd-capplet gnome-control-center gnome-session gnome-settings-daemon gnome-sushi gnome-shell network-manager-gnome nautilus --no-install-recommends";
    pub const ALL_PACKAGES_TO_INSTALL_DEBIAN_CINNAMON: &str = "cinnamon-core adwaita-icon-theme";
    pub const ALL_PACKAGES_TO_INSTALL_DEBIAN_MATE: &str = "adwaita-icon-theme caja dconf-gsettings-backend fonts-cantarell gvfs-backends marco mate-control-center mate-desktop mate-icon-theme mate-menus mate-notification-daemon mate-panel mate-polkit mate-session-manager mate-settings-daemon mate-themes";
    pub const ALL_PACKAGES_TO_INSTALL_DEBIAN_KDEPLASMA: &str = "papirus-icon-theme plasma-desktop dolphin kdialog keditbookmarks kfind plasma-workspace udisks2 upower plasma-nm plasma-workspace-wayland"; /*Fix*/ // breeze breeze-gtk adicionar eles 
    let all_packages_to_remove_debian: String = format!("{} {} {} {} {} {} {}", ALL_PACKAGES_TO_INSTALL_DEBIAN_LXDE.to_owned(), ALL_PACKAGES_TO_INSTALL_DEBIAN_LXQT.to_owned(), ALL_PACKAGES_TO_INSTALL_DEBIAN_XFCE4.to_owned(), ALL_PACKAGES_TO_INSTALL_DEBIAN_GNOME.to_owned(), ALL_PACKAGES_TO_INSTALL_DEBIAN_CINNAMON.to_owned(), ALL_PACKAGES_TO_INSTALL_DEBIAN_MATE.to_owned(), ALL_PACKAGES_TO_INSTALL_DEBIAN_KDEPLASMA.to_owned());
    let all_packets_do_debian_no_duplicate_packets = utils::remove_repeated_words(&all_packages_to_remove_debian);

    pub const ALL_PACKAGES_TO_INSTALL_FEDORA_LXDE: &str = "lxappearance lxde-common lxinput lxmenu-data lxpanel lxpolkit lxrandr xcompmgr pcmanfm network-manager-applet openbox obconf";
    pub const ALL_PACKAGES_TO_INSTALL_FEDORA_LXQT: &str = "breeze-gtk breeze-icon-theme network-manager-applet notification-daemon obconf openbox pcmanfm-qt lxqt-config lxqt-notificationd lxqt-panel lxqt-policykit lxqt-powermanagement lxqt-qtplugin lxqt-themes lxqt-session breeze-cursor-theme";
    pub const ALL_PACKAGES_TO_INSTALL_FEDORA_XFCE4: &str = "network-manager-applet xfwm4 xfce4-power-manager xfce4-settings xfce4-whiskermenu-plugin xfdesktop";
    pub const ALL_PACKAGES_TO_INSTALL_FEDORA_GNOME: &str = "gnome-shell gnome-control-center nautilus";
    pub const ALL_PACKAGES_TO_INSTALL_FEDORA_CINNAMON: &str = "cinnamon cinnamon-control-center cinnamon-desktop cinnamon-menus nemo nemo-fileroller cinnamon-translations cjs muffin";
    pub const ALL_PACKAGES_TO_INSTALL_FEDORA_MATE: &str = "mate-control-center mate-desktop mate-power-manager mate-screensaver mate-session-manager mate-settings-daemon mate-terminal network-manager-applet mate-panel marco caja";
    pub const ALL_PACKAGES_TO_INSTALL_FEDORA_KDEPLASMA: &str = "plasma-desktop plasma-nm kcm_colors kcm-fcitx kscreen ksysguard spectacle dolphin";
    let all_packages_to_remove_fedora: String = format!("{} {} {} {} {} {} {}", ALL_PACKAGES_TO_INSTALL_DEBIAN_LXDE.to_owned(), ALL_PACKAGES_TO_INSTALL_DEBIAN_LXQT.to_owned(), ALL_PACKAGES_TO_INSTALL_DEBIAN_XFCE4.to_owned(), ALL_PACKAGES_TO_INSTALL_DEBIAN_GNOME.to_owned(), ALL_PACKAGES_TO_INSTALL_DEBIAN_CINNAMON.to_owned(), ALL_PACKAGES_TO_INSTALL_DEBIAN_MATE.to_owned(), ALL_PACKAGES_TO_INSTALL_DEBIAN_KDEPLASMA.to_owned());
    let all_packets_do_fedora_no_duplicate_packets = utils::remove_repeated_words(&all_packages_to_remove_fedora);

    match &option[..] {
        "--clean-archlinux" => {
            utils::system_command("sudo pacman -Rsn $(pacman -Qdtq) --noconfirm"); /* Command To Clean Up Dead Pacman Packages */
            utils::system_command("sudo pacman -Scc --noconfirm"); /* Command To Clear Pacman Cache */
            clean_systemd();
            utils::system_command("flatpak uninstall --unused"); /* Command To Uninstall Orphaned Flatpak Packages */
            utils::remove_folder("/*/*/.var/app/*/cache/"); /* Remove cache folder content of all flatpak's */
            utils::remove_folder("/*/*/.cache/"); /* Remove cache folder content */
            exit(0);
        },

        "--clean-debian" => {
            utils::system_command("sudo apt clean"); /* Command To Clear APT Cache */
            utils::system_command("sudo apt autoclean"); /* Command To Clean Up Dead APT Packages */
            utils::system_command("sudo apt install deborphan -y"); /* Command To Install Deborphan */
            for _i in 0..4 {
                utils::system_command("sudo apt remove $(deborphan) -y"); /* Command To Clean Up Orphaned Packages With Deborphan */
            }
            utils::system_command("sudo apt remove deborphan -y"); /* Command To Uninstall Deborphan From The System */
            utils::system_command("sudo apt autoremove -y"); /* Command To Clean Up Orphaned APT Packages */
            clean_systemd();
            utils::system_command("flatpak uninstall --unused"); /* Command To Uninstall Orphaned Flatpak Packages */
            utils::remove_folder("/*/*/.var/app/*/cache/"); /* Remove cache folder content of all flatpak's */
            utils::remove_folder("/*/*/.cache/"); /* Remove cache folder content */
            exit(0);
        },

        "--clean-fedora" => {
            utils::system_command("sudo dnf clean all"); /* Command To Clear DNF Cache */
            utils::system_command("sudo dnf autoremove -y"); /* Command To Clean Up DNF Orphaned Packages */
            utils::remove_folder("/*/*/.var/app/*/cache/"); /* Remove cache folder content of all flatpak's */
            utils::remove_folder("/*/*/.cache/"); /* Remove cache folder content */
            clean_systemd();
            exit(0);
        },

        "--install-archlinux-lxde" => {
            // List Of Graphical Environment Packages And What They Are For:
            //
            // - lxappearance: Feature-Rich GTK+ Theme Switcher Of The LXDE Desktop
            // - lxappearance-obconf: Plugin For LXAppearance To Configure Openbox
            // - lxde-common: Common Files Of The LXDE Desktop
            // - lxde-icon-theme: LXDE Default Icon Theme Based On NuoveXT2
            // - lxhotkey: Keyboard Shortcuts Configurator
            // - lxinput: Small Program To Configure Keyboard And Mouse For LXDE
            // - lxlauncher: Open Source Clone Of The Asus Launcher For EeePC
            // - lxpanel: Lightweight X11 Desktop Panel For LXDE
            // - lxrandr: Monitor Configuration Tool
            // - lxsession: Lightweight X11 Session Manager
            // - pcmanfm: Extremely Fast And Lightweight File Manager
            // - openbox: Highly Configurable And lightweight X11 Window Manager
            //
            // https://archlinux.org/groups/x86_64/lxde/
            //
            utils::install_system_and_utilities(all_packets_do_archlinux_no_duplicate_packets, ALL_PACKAGES_TO_INSTALL_ARCHLINUX_LXDE, "archlinux");
            exit(0);
        },

        "--install-archlinux-lxqt" => {
            // List Of Graphical Environment Packages And What They Are For:
            //
            // - lxqt-session: The LXQt session manager
            // - lxqt-admin: LXQt System Administration Tool
            // - lxqt-config: LXQt System Configuration
            // - lxqt-globalkeys: LXQt Daemon And Library For Global Keyboard Shortcuts Registration
            // - lxqt-notificationd: LXQt Notification Daemon And Library
            // - lxqt-panel: The LXQt Desktop Panel
            // - lxqt-policykit: The LXQt PolicyKit Authentication Agent
            // - lxqt-powermanagement: LXQt Power Management Daemon
            // - lxqt-themes: LXQt Themes, Graphics And Icons
            // - obconf-qt: Openbox Configuration Tool. Qt Port Of ObConf
            // - openbox: Highly Configurable And LightWeight X11 Window Manager
            // - pavucontrol-qt: A Pulseaudio Mixer In Qt (port of pavucontrol)
            // - pcmanfm-qt: The LXQt File Manager, Qt Port Of PCManFM
            //
            // https://archlinux.org/groups/x86_64/lxqt/
            //
            utils::install_system_and_utilities(all_packets_do_archlinux_no_duplicate_packets, ALL_PACKAGES_TO_INSTALL_ARCHLINUX_LXQT, "archlinux");
            exit(0);
        },

        "--install-archlinux-xfce" => {
            // List Of Graphical Environment Packages And What They Are For:
            //
            // - xfce4-session: Xfce's session manager
            // - exo: Application Library For The Xfce Desktop Environment
            // - garcon: Freedesktop.org Compliant Menu Library
            // - thunar: Modern, Fast And easy-to-use File Manager For Xfce
            // - thunar-volman: Automatic management Of Removable Drives And Media For Thunar
            // - tumbler: Thumbnail Service Implementing The Thumbnail Management D-Bus Specification
            // - xfce4-panel: Panel For The Xfce Desktop Environment
            // - xfce4-power-manager: Power Manager For Xfce
            // - xfce4-settings: Xfce's Configuration System
            // - xfconf: D-Bus-based Configuration Storage System
            // - xfdesktop: Xfce's Desktop Manager
            // - xfwm4: Xfce's Window Manager
            // - xfwm4-themes: A Set Of Additional Themes For The Xfce Window Manager
            //
            // https://archlinux.org/groups/x86_64/xfce4/
            //
            utils::install_system_and_utilities(all_packets_do_archlinux_no_duplicate_packets, ALL_PACKAGES_TO_INSTALL_ARCHLINUX_XFCE4, "archlinux");
            exit(0);
        },

        "--install-archlinux-gnome" => {
            // List Of Graphical Environment Packages And What They Are For:
            //
            // - gnome-shell: Next Generation Desktop Shell
            // - nautilus: Default File Manager For GNOME
            // - gnome-control-center: GNOME's Main Interface To Configure Various Aspects Of The Desktop
            // - gnome-text-editor: A Simple Text Editor For The GNOME Desktop
            //
            // https://archlinux.org/groups/x86_64/gnome/
            //
            utils::install_system_and_utilities(all_packets_do_archlinux_no_duplicate_packets, ALL_PACKAGES_TO_INSTALL_ARCHLINUX_GNOME, "archlinux");
            utils::system_command("gsettings set org.gnome.desktop.interface enable-animations false");
            exit(0);
        },

        "--install-archlinux-cinnamon" => {
            // List Of Graphical Environment Packages And What They Are For:
            //
            // - cinnamon: Linux desktop which provides advanced innovative features and a traditional user experience.
            // - cinnamon-session: The Cinnamon Session Handler
            // - accountsservice: D-Bus Interface For User Account Query And Manipulation
            // - cinnamon-control-center: The Control Center For Cinnamon
            // - cinnamon-menus: Cinnamon Menu Specifications
            // - cinnamon-settings-daemon: The Cinnamon Settings Daemon
            // - gnome-themes-extra: Extra Themes For GNOME Applications
            // - gsound: Small Library For Playing System Sounds
            // - muffin: Cinnamon Window Manager Based On Mutter
            // - nemo: Cinnamon File Manager
            // - network-manager-applet: Applet For Managing Network Connections
            // - polkit-gnome: Legacy Polkit Authentication Agent For GNOME
            //
            // https://archlinux.org/packages/community/x86_64/cinnamon/
            //
            utils::install_system_and_utilities(all_packets_do_archlinux_no_duplicate_packets, ALL_PACKAGES_TO_INSTALL_ARCHLINUX_CINNAMON, "archlinux");
            exit(0);
        },

        "--install-archlinux-mate" => {
            // List Of Graphical Environment Packages And What They Are For:
            //
            // - mate-session-manager: The MATE Session Handler.
            // - mate-media: MATE Media Tools.
            // - pulseaudio: A featureful, general-purpose sound server.
            // - caja: File Manager For The MATE Desktop
            // - marco: A Window Manager For MATE
            // - mate-control-center: The Control Center For MATE
            // - mate-desktop: Library With Common API For Various MATE Modules
            // - mate-icon-theme: MATE Icon Theme
            // - mate-menus: MATE Menu Specifications
            // - mate-notification-daemon: Notification Daemon For MATE
            // - mate-panel: The MATE Panel
            // - mate-polkit: PolicyKit Integration For The MATE Desktop
            // - mate-settings-daemon: The MATE Settings Daemon
            // - mate-themes: Default Themes For The MATE Desktop
            //
            // https://archlinux.org/groups/x86_64/mate/
            //
            utils::install_system_and_utilities(all_packets_do_archlinux_no_duplicate_packets, ALL_PACKAGES_TO_INSTALL_ARCHLINUX_MATE, "archlinux");
            exit(0);
        },

        "--install-archlinux-kdeplasma" => {
            // List Of Graphical Environment Packages And What They Are For:
            //
            // - plasma-pa: Plasma applet for audio volume management using PulseAudio.
            // - breeze: Artwork, Styles And Assets For The Breeze Visual Style For The Plasma Desktop
            // - breeze-gtk: Breeze Widget Theme For GTK 2 And 3
            // - kde-gtk-config: GTK2 And GTK3 Configurator For KDE
            // - kdecoration: Plugin Based Library To Create Window Decorations
            // - kdeplasma-addons: All Kind Of Addons To Improve Your Plasma Experience
            // - khotkeys: KHotKeys	
            // - kmenuedit: KDE Menu Editor
            // - kpipewire: Components Relating To Pipewire Use In Plasma
            // - kwallet-pam: KWallet PAM Integration
            // - kwayland-integration: Provides Integration Plugins For Various KDE Frameworks For The Wayland Windowing System
            // - kwin: An Easy To Use, But Flexible And Composited Window Manager
            // - layer-shell-qt: Qt Component To Allow Applications To Make Use Of The Wayland wl-layer-shell Protocol
            // - libkscreen: KDE Screen Management Software	
            // - libksysguard: Library To Retrieve Information On The Current Status Of Computer Hardware	
            // - milou: A Dedicated Search Application Built On Top Of Baloo	
            // - plasma-browser-integration: Components Necessary To Integrate Browsers Into The Plasma Desktop
            // - plasma-desktop: KDE Plasma Desktop
            // - plasma-integration: Qt Platform Theme Integration Plugins For The Plasma Workspaces
            // - plasma-nm: Plasma applet Written In QML For Managing Network Connections
            // - plasma-workspace: KDE Plasma Workspace
            // - polkit-kde-agent: Daemon Providing A Polkit Authentication UI For KDE
            // - powerdevil: Manages The Power Consumption Settings Of A Plasma Shell
            // - sddm-kcm: 	KDE Config Module For SDDM
            // - systemsettings: KDE System Manager For Hardware, Software And Workspaces
            // - xdg-desktop-portal-kde: A Backend Implementation For xdg-desktop-portal Using Qt/KF5
            // - plasma-wayland-session: Plasma Wayland Session
            // - kcm-fcitx: KDE Config Module For Fcitx
            // - kscreen: KDE Screen Management Software
            // - dolphin: KDE File Manager
            //
            // https://archlinux.org/groups/x86_64/plasma/
            //
            utils::install_system_and_utilities(all_packets_do_archlinux_no_duplicate_packets, ALL_PACKAGES_TO_INSTALL_ARCHLINUX_KDEPLASMA, "archlinux");
            exit(0);
        },

        "--install-debian-lxde" => {
            // List Of Graphical Environment Packages And What They Are For:
            //
            // - adwaita-icon-theme: default icon theme of GNOME.
            // - lxappearance: LXDE GTK+ Theme Switcher
            // - lxappearance-obconf: LXDE GTK+ Theme Switcher (Plugin)
            // - gtk2-engines: Theme Engines For GTK+ 2.x
            // - lxde-settings-daemon: XSettings Compliant Configuration Manager For LXDE
            // - lxpanel: LXDE Panel
            // - pcmanfm: Extremely Fast And Lightweight File Manager
            // - lxde-icon-theme: LXDE Standard Icon Theme
            // - lxhotkey-gtk: LXHotkey Keyboard Shortcuts Configurator (GTK+ GUI Plugin)
            // - lxinput: LXDE Keyboard And Mouse Configuration
            // - lxrandr: LXDE Monitor Configuration Tool
            // - lxsession-edit: Configure What Application Start Up Automatically In LXDE
            // - lxpolkit: LXDE PolicyKit Authentication Agent
            // - lx-session: LXDE Session Manager And Configuration Files
            // - lxsession-default-apps: Utility To Configure LXSession And Its Default Applications
            // - xscreensaver: Screensaver Daemon And Frontend For X11
            // - policykit-1: Framework For Managing Administrative Policies And Privileges
            // 
            // https://packages.debian.org/bullseye/lxde
            //
            utils::install_system_and_utilities(all_packets_do_debian_no_duplicate_packets, ALL_PACKAGES_TO_INSTALL_DEBIAN_LXDE, "debian");
            exit(0);
        },

        "--install-debian-lxqt" => {
            // List Of Graphical Environment Packages And What They Are For:
            //
            // - papirus-icon-theme: Papirus open source icon theme for Linux.
            // - lxqt-config: LXQt System Settings Center
            // - lxqt-globalkeys: Daemon Used To Register Global Keyboard Shortcuts (Appl.)
            // - lxqt-notificationd: LXQt Notification Daemon
            // - lxqt-panel: LXQt Desktop Panel
            // - lxqt-policykit: LXQt Authentication Agent For PolicyKit
            // - lxqt-qtplugin: LXQt System Integration Plugin For Qt
            // - lxqt-session: Session Manager Component For LXQt
            // - lxqt-system-theme: System Theme For LXQt
            // - pcmanfm-qt: extremely Fast And Lightweight File And Desktop Icon Manager
            // - lxqt-powermanagement: Power Management Module For LXQt
            // 
            // https://packages.debian.org/bullseye/lxde
            //
            utils::install_system_and_utilities(all_packets_do_debian_no_duplicate_packets, ALL_PACKAGES_TO_INSTALL_DEBIAN_LXQT, "debian");
            exit(0);
        },

        "--install-debian-xfce" => {
            // List Of Graphical Environment Packages And What They Are For:
            //
            // - adwaita-icon-theme: default icon theme of GNOME.
            // - xfce4-notifyd: Simple, Visually-Appealing Notification Daemon For Xfce
            // - xfwm4: Window Manager Of The Xfce Project
            // - xfce4-whiskermenu-plugin: Alternate Menu Plugin For The Xfce Desktop Environment
            // - xdg-user-dirs: Tool To Manage Well Known User Directories
            // - xfdesktop4: Xfce Desktop Background, Icons And Root Menu Manager
            // - xfconf: Utilities For Managing Settings In Xfce
            // - xfce4-settings: Graphical Application For Managing Xfce Settings
            // - xfce4-session: Xfce4 Session Manager
            // - xfce4-pulseaudio-plugin: Xfce4 Panel Plugin To Control Pulseaudio
            // - xfce4-panel: Panel For Xfce4 Desktop Environment
            // - libxfce4ui-utils: Utility Files For LibXfce4UI
            // - thunar: File Manager For Xfce
            // - thunar-archive-plugin: Archive Plugin For Thunar File Manager
            // - thunar-media-tags-plugin: Media Tags Plugin For Thunar File Manager
            // 
            // https://packages.debian.org/bullseye/xfce4
            //
            utils::install_system_and_utilities(all_packets_do_debian_no_duplicate_packets, ALL_PACKAGES_TO_INSTALL_ARCHLINUX_XFCE4, "debian");
            exit(0);
        },

        "--install-debian-gnome" => {
            // List Of Graphical Environment Packages And What They Are For:
            //
            // - adwaita-icon-theme: Default Icon Theme Of GNOME
            // - at-spi2-core: Assistive Technology Service Provider Interface (Dbus Core)
            // - caribou: Configurable On Screen Keyboard With Scanning Mode
            // - dconf-cli: Simple Configuration Storage System - Utilities
            // - dconf-gsettings-backend: Simple Configuration Storage System - GSettings back-end
            // - evolution-data-server: Evolution Database Backend Server
            // - fonts-cantarell: Sans Serif Font Family Designed For On-Screen Readability
            // - gkbd-capplet: GNOME Control Center Tools For LibGnomeKBD
            // - gnome-control-center: Utilities To Configure The GNOME Desktop
            // - gnome-session: GNOME Session Manager - GNOME 3 Session
            // - gnome-settings-daemon: Daemon Handling The GNOME Session Settings
            // - gnome-sushi: Sushi Is A Quick Previewer For Nautilus
            // - gnome-shell: Graphical Shell For The GNOME Desktop
            // - network-manager-gnome: Network Management Framework (GNOME Frontend)
            // - nautilus: File Manager And Graphical Shell For GNOME
            // 
            // https://packages.debian.org/bullseye/gnome-core
            //
            utils::install_system_and_utilities(all_packets_do_debian_no_duplicate_packets, ALL_PACKAGES_TO_INSTALL_DEBIAN_GNOME, "debian");
            utils::system_command("gsettings set org.gnome.desktop.interface enable-animations false");
            exit(0);
        },

        "--install-debian-cinnamon" => {
            // List Of Graphical Environment Packages And What They Are For:
            //
            // - cinnamon-core: Cinnamon Desktop Environment - Essential Components
            // - adwaita-icon-theme: default icon theme of GNOME.
            // 
            // https://packages.debian.org/bullseye/cinnamon-core
            //
            utils::install_system_and_utilities(all_packets_do_debian_no_duplicate_packets, ALL_PACKAGES_TO_INSTALL_DEBIAN_CINNAMON, "debian");
            exit(0);
        },

        "--install-debian-mate" => {
            // List Of Graphical Environment Packages And What They Are For:
            //
            // - adwaita-icon-theme: default icon theme of GNOME.
            // - caja: File Manager For The MATE Desktop
            // - dconf-gsettings-backend: Simple Configuration Storage System - GSettings Back-End
            // - fonts-cantarell: Sans Serif Font Family Designed For On-Screen Readability
            // - gvfs-backends: Userspace Virtual Filesystem - Backends
            // - marco: Lightweight GTK+ Window Manager For MATE
            // - mate-control-center: Utilities To Configure The MATE Desktop
            // - mate-desktop: Library With Common API For Various MATE Modules
            // - mate-icon-theme: MATE Desktop Icon Theme
            // - mate-menus: Implementation Of The FreeDesktop Menu Specification For MATE
            // - mate-notification-daemon: Daemon To Display Passive PopUp Notifications
            // - mate-panel: Launcher And Docking Facility For MATE
            // - mate-polkit: MATE Authentication Agent For PolicyKit-1
            // - mate-session-manager: Session manager of the MATE desktop environment
            // - mate-settings-daemon: Daemon Handling The MATE Session Settings
            // - mate-themes: Official Themes For The MATE Desktop
            // 
            // https://packages.debian.org/bullseye/mate-desktop-environment-core
            //
            utils::install_system_and_utilities(all_packets_do_debian_no_duplicate_packets, ALL_PACKAGES_TO_INSTALL_FEDORA_MATE, "debian");
            exit(0);
        },

        "--install-debian-kdeplasma" => {
            // List Of Graphical Environment Packages And What They Are For:
            //
            // - papirus-icon-theme: Papirus open source icon theme for Linux.
            // - plasma-desktop: Tools And Widgets For The Desktop
            // - dolphin: File Manager
            // - kdialog: Dialog Display Utility
            // - keditbookmarks: bookmarks editor utility for KDE
            // - kfind: File Search Utility By KDE
            // - plasma-workspace: Plasma Workspace For KF5
            // - udisks2: D-Bus Service To Access And Manipulate Storage Devices
            // - upower: Abstraction For Power Management
            // - plasma-nm: Plasma Network Connections Management
            // - plasma-workspace-wayland: Plasma Workspace For KF5 - Wayland Integration
            // 
            // https://packages.debian.org/bullseye/plasma-desktop
            //
            utils::install_system_and_utilities(all_packets_do_debian_no_duplicate_packets, ALL_PACKAGES_TO_INSTALL_DEBIAN_KDEPLASMA, "debian");
            exit(0);
        },

        "--install-fedora-lxde" => {
            // List Of Graphical Environment Packages And What They Are For:
            //
            // - lxappearance: Feature-rich GTK+ Theme Switcher For LXDE
            // - lxde-common: Default Configuration Files For LXDE
            // - lxinput: Keyboard And Mouse Settings Dialog For LXDE
            // - lxmenu-data: Data Files For The LXDE Menu
            // - lxpanel: A Lightweight X11 Desktop Panel
            // - lxpolkit: Simple PolicyKit Authentication Agent
            // - lxrandr: Simple Monitor Configuration Tool
            // - xcompmgr: X11 Composite Manager
            // - pcmanfm: Extremly Fast And Lightweight File Manager
            // - network-manager-applet: A Network Control And Status Applet For NetworkManager
            // - openbox: A Highly Configurable And Standards-Compliant X11 Window Manager
            // - obconf: A Graphical Configuration Editor For The Openbox Window Manager
            // 
            // sudo dnf info {package}
            //
            utils::install_system_and_utilities(all_packets_do_fedora_no_duplicate_packets, ALL_PACKAGES_TO_INSTALL_FEDORA_LXDE, "fedora");
            exit(0);
        },

        "--install-fedora-lxqt" => {
            // List Of Graphical Environment Packages And What They Are For:
            //
            // - breeze-gtk: Breeze Widget Theme For GTK
            // - breeze-icon-theme: Breeze Icon Theme
            // - network-manager-applet: A Network Control And Status Applet For NetworkManager
            // - notification-daemon: Desktop Notification Daemon
            // - obconf: A Graphical Configuration Editor For The Openbox Window Manager
            // - openbox: A Highly Configurable And Standards-Compliant X11 Window Manager
            // - pcmanfm-qt: LxQt File Manager PCManFM
            // - lxqt-config: Config Tools For LXQt Desktop Suite
            // - lxqt-notificationd: Notification Daemon For LXQt Desktop Suite
            // - lxqt-panel: Main Panel Bar For LXQt Desktop Suite
            // - lxqt-policykit: PolicyKit Agent For LXQt Desktop Suite
            // - lxqt-powermanagement: Powermanagement Daemon For LXQt Desktop Suite
            // - lxqt-qtplugin: Qt Plugin Framework For LXQt Desktop Suite
            // - lxqt-themes: LXQt Standard Themes
            // - lxqt-session: Main Session For LXQt Desktop Suite
            // - breeze-cursor-theme: Breeze Cursor Theme
            // 
            // sudo dnf info {package}
            //
            utils::install_system_and_utilities(all_packets_do_fedora_no_duplicate_packets, ALL_PACKAGES_TO_INSTALL_FEDORA_LXQT, "fedora");
            exit(0);
        },

        "--install-fedora-xfce" => {
            // List Of Graphical Environment Packages And What They Are For:
            //
            // - xfwm4: Xfwm4 is a window manager compatible with GNOME, GNOME2, KDE2, KDE3 and Xfce.
            // - xfce4-power-manager: Xfce Power Manager uses the information and facilities provided by HAL to display icons and handle user callbacks in an interactive Xfce session. Xfce Power Preferences allows authorised users to set policy and change preferences.
            // - xfce4-settings: This package includes the settings manager applications for the Xfce desktop.
            // - xfce4-whiskermenu-plugin: Alternate application launcher for Xfce. When you open it you are shown a list of applications you have marked as favorites. You can browse through all of your installed applications by clicking on the category buttons on the side. Top level categories make browsing fast, and simple to switch between. Additionally, Whisker Menu keeps a list of the last ten applications that you’ve launched from it
            // - xfdesktop: This package includes a desktop manager for the Xfce Desktop Environment.
            // 
            // sudo dnf info @xfce-desktop
            //
            utils::install_system_and_utilities(all_packets_do_fedora_no_duplicate_packets, ALL_PACKAGES_TO_INSTALL_FEDORA_XFCE4, "fedora");
            exit(0);
        },

        "--install-fedora-gnome" => {
            // List Of Graphical Environment Packages And What They Are For:
            //
            // - gnome-shell: GNOME Shell provides core user interface functions for the GNOME 3 desktop, like switching to windows and launching applications. 
            // - gnome-control-center: This package contains configuration utilities for the GNOME desktop, which allow to configure accessibility options, desktop fonts, keyboard and mouse properties, sound setup, desktop theme and background, user interface properties, screen resolution, and other settings.
            // - nautilus: Nautilus is the file manager and graphical shell for the GNOME desktop that makes it easy to manage your files and the rest of your system.
            // 
            // sudo dnf info @gnome-desktop
            //
            utils::install_system_and_utilities(all_packets_do_fedora_no_duplicate_packets, ALL_PACKAGES_TO_INSTALL_FEDORA_GNOME, "fedora");
            utils::system_command("gsettings set org.gnome.desktop.interface enable-animations false");
            exit(0);
        },

        "--install-fedora-cinnamon" => {
            // List Of Graphical Environment Packages And What They Are For:
            //
            // - cinnamon: Cinnamon is a Linux desktop which provides advanced innovative features and a traditional user experience.
            // - cinnamon-control-center: This package contains configuration utilities for the Cinnamon desktop, which allow to configure accessibility options, desktop fonts, keyboard and mouse properties, sound setup, desktop theme and background, user interface properties, screen resolution, and other settings.
            // - cinnamon-desktop: The cinnamon-desktop package contains an internal library (libcinnamon-desktop) used to implement some portions of the CINNAMON desktop, and also some data files and other shared components of the CINNAMON user environment.
            // - cinnamon-menus: Cinnamon-menus is an implementation of the draft "Desktop Menu Specification" from freedesktop.org. This package also contains the Cinnamon menu layout configuration files, .directory files and assorted menu related utility programs, Python bindings, and a simple menu editor.
            // - nemo: Nemo is the file manager and graphical shell for the Cinnamon desktop that makes it easy to manage your files and the rest of your system. It allows to browse directories on local and remote filesystems, preview files and launch applications associated with them. It is also responsible for handling the icons on the Cinnamon desktop.
            // - nemo-fileroller: This package contains the file-roller extension for the Nemo.
            // - cinnamon-translations: Translations for Cinnamon, Nemo and Mintlocale.
            // - cjs: Cjs allows using Cinnamon libraries from Javascript. It's based on the Spidermonkey Javascript engine from Mozilla and the GObject introspection framework.
            // - muffin: Muffin is a window and compositing manager that displays and manages your desktop via OpenGL. Muffin combines a sophisticated display engine using the Clutter toolkit with solid window-management logic inherited from the Metacity window manager.
            // 
            // sudo dnf info @cinnamon-desktop
            //
            utils::install_system_and_utilities(all_packets_do_fedora_no_duplicate_packets, ALL_PACKAGES_TO_INSTALL_FEDORA_CINNAMON, "fedora");
            exit(0);
        },

        "--install-fedora-mate" => {
            // List Of Graphical Environment Packages And What They Are For:
            //
            // - mate-control-center: MATE Control Center configures system settings such as themes, keyboards shortcuts, etc.
            // - mate-desktop: The mate-desktop package contains an internal library (libmatedesktop) used to implement some portions of the MATE desktop, and also some data files and other shared components of the MATE user environment.
            // - mate-power-manager: MATE Power Manager uses the information and facilities provided by UPower displaying icons and handling user callbacks in an interactive MATE session.
            // - mate-screensaver: Mate-screensaver is a screen saver and locker that aims to have simple, sane, secure defaults and be well integrated with the desktop.
            // - mate-session-manager: This package contains a session that can be started from a display manager such as MDM. It will load all necessary applications for a full-featured user session.
            // - mate-settings-daemon: This package contains the daemon which is responsible for setting the various parameters of a MATE session and the applications that run under it.
            // - mate-terminal: Mate-terminal is a terminal emulator for MATE. It supports translucent backgrounds, opening multiple terminals in a single window (tabs) and clickable URLs.
            // - network-manager-applet: This package contains a network control and status notification area applet for use with NetworkManager.
            // - mate-panel: MATE Desktop panel applets
            // - marco: MATE Desktop window manager
            // - caja: Caja (mate-file-manager) is the file manager and graphical shell for the MATE desktop, that makes it easy to manage your files and the rest of your system. It allows to browse directories on local and remote file systems, preview files and launch applications associated with them. It is also responsible for handling the icons on the MATE desktop.
            // 
            // sudo dnf info @mate-desktop
            //
            utils::install_system_and_utilities(all_packets_do_fedora_no_duplicate_packets, ALL_PACKAGES_TO_INSTALL_FEDORA_MATE, "fedora");
            exit(0);
        },

        "--install-fedora-kdeplasma" => {
            // List Of Graphical Environment Packages And What They Are For:
            //
            // - plasma-desktop: Plasma Desktop shell.
            // - plasma-nm: Plasma applet and editor for managing your network connections in KDE 4 using the default NetworkManager service.
            // - kcm_colors: The Color Selection module is comprised of several sections * The Scheme tab, used to manage schemes * The Options tab, used to change the options of the current scheme * The Colors tab, used to change the colors of the current scheme * The state effects tabs (Inactive, Disabled) 
            // - kcm-fcitx: Kcm-fcitx is a System Settings module to manage Fcitx. You can config fcitx through "Personalization" - "Regional Settings" - "Input Method" now.
            // - kscreen: KCM and KDED modules for managing displays in KDE.
            // - ksysguard: KDE Process Management application.
            // - spectacle: Screenshot capture utility.
            // - dolphin: KDE File Manager.
            // 
            // sudo dnf info @plasma-desktop
            //
            utils::install_system_and_utilities(all_packets_do_fedora_no_duplicate_packets, ALL_PACKAGES_TO_INSTALL_FEDORA_KDEPLASMA, "fedora");
            exit(0);
        },

        "--install-submodule" => {
            let file = &args[2].trim();
            let path = Path::new(file);
            let file_name_without_extension = path.file_stem().unwrap().to_string_lossy();
            let file_name_without_extension: &str = &String::from(file_name_without_extension);
            let rename = fs::rename(&file, &file_name_without_extension);

            match rename {
                Ok(_) => {
                    println!("Package: {}  Renamed to: {}, {}", file, &file_name_without_extension, "Successfully".green().bold());
                    utils::system_command(&format!("./{}", &file_name_without_extension));
                },
                Err(_) => {
                    println!("Package: {}  Renamed to: {}, {}", file, &file_name_without_extension, "Erro".red().bold());
                    exit(1);
                }
            }
        },

        "--help" => {
            println!("{}", texts::HELP_EN_US); // Shows User List Of Mild Commands
        },

        _ => {
            println!("{}", texts::HELP_EN_US); // Shows User List Of Mild Commands
        }
    }
}