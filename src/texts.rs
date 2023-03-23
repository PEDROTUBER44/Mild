pub const ALL_PACKAGES_TO_REMOVE_ARCHLINUX: &str = "";
pub const ALL_PACKAGES_TO_REMOVE_DEBIAN: &str = "";
pub const ALL_PACKAGES_TO_REMOVE_FEDORA: &str = "";

pub const PACMAN_CONFIG_FILE: &str = "[options]
HoldPkg = pacman glibc
Architecture = auto

CheckSpace

SigLevel = Required DatabaseOptional
LocalFileSigLevel = Optional

[core]
Include = /etc/pacman.d/mirrorlist

[extra]
Include = /etc/pacman.d/mirrorlist

[community]
Include = /etc/pacnan.d/mirrorlist

[multilib]
Include = /etc/pacnan.d/mirrorlist
";
pub const DEBIAN_CONFIG_FILE: &str = "";
pub const DNF_CONFIG_FILE: &str = r#"[main]
gpgcheck=1
installonly_limit=3
clean_requirements_on_remove=True
best=False
skip_if_unavailable=True
fastestmirror=True
max_parallel_downloads=7
defaultyes=True
install_weak_deps=false"#;

pub const DISABLE_DISPLAY_MANAGERS_CMD: &str = "sudo systemctl disable gdm -f && sudo systemctl disable lightdm -f && sudo systemctl disable sddm -f && sudo systemctl disable lxdm -f";
pub const INSTALL_UTILS_FOR_ARCHLINUX: &str = "sudo pacman -S flatpak xorg xorg-server networkmanager gvfs-mtp gvfs-goa gvfs-google exfat-utils p7zip zip unzip unrar ffmpeg gst-plugins-ugly gst-plugins-good gst-plugins-base gst-plugins-bad gst-libav gstreamer a52dec faac faad2 flac jasper lame libdca libdv libmad libmpeg2 libtheora libvorbis libxv opus wavpack x264 xvidcore --noconfirm";
pub const INSTALL_UTILS_FOR_DEBIAN: &str = "sudo apt install flatpak sudo zip unzip unrar-free network-manager xorg gvfs pulseaudio exfat-utils p7zip-full adwaita-icon-theme gstreamer1.0-plugins-base gstreamer1.0-plugins-good gstreamer1.0-plugins-ugly gstreamer1.0-plugins-bad gstreamer1.0-plugins-base ffmpeg sox twolame vorbis-tools lame faad -y";
pub const INSTALL_UTILS_FOR_FEDORA: &str = r#"sudo dnf install flatpak @base-x @multimedia network-manager-applet unrar p7zip zip unzip NetworkManager exfat-utils lame\* gvfs-mtp gvfs-goa gstreamer1-plugins-ugly gstreamer1-plugins-bad-free gstreamer1-plugins-bad-free gstreamer1-plugins-bad-freeworld gstreamer1-plugins-bad-free-extras ffmpeg gstreamer1-plugins-{bad-\*,good-\*,base} gstreamer1-plugin-openh264 gstreamer1-libav --exclude=gstreamer1-plugins-bad-free-devel --exclude=lame-devel --skip-broken -y"#;
pub const INSTALL_RPMFUSION_REPOSITORY: &str = "sudo dnf install https://download1.rpmfusion.org/nonfree/fedora/rpmfusion-nonfree-release-37.noarch.rpm https://download1.rpmfusion.org/free/fedora/rpmfusion-free-release-37.noarch.rpm -y";
pub const INSTALL_FLATHUB: &str = "flatpak remote-add --if-not-exists flathub https://flathub.org/repo/flathub.flatpakrepo";
pub const ENABLE_NETWORKMANAGER: &str = "sudo systemctl enable NetworkManager -f";
pub const ENABLE_PRELOAD: &str = "sudo systemctl enable preload -f";
pub const ENABLE_BLUETOOTH: &str = "sudo systemctl enable bluetooth -f";
pub const ENABLE_GDM: &str = "sudo systemctl enable gdm -f";
pub const ENABLE_LIGHTDM: &str = "sudo systemctl enable lightdm -f";
pub const ENABLE_SDDM: &str = "sudo systemctl enable sddm -f";
pub const ENABLE_LXDM: &str = "sudo systemctl enable lxdm -f";
pub const ENABLE_GRAPHICAL_INITIALIZATION: &str = "sudo systemctl set-default graphical.target";


pub const HELP_EN_US: &str = "

Use mild --[option]-[distribution]-[interface]

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