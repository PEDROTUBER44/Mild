pub const DEBIAN_CONFIG_FILE: &str = "deb http://deb.debian.org/debian bullseye main contrib non-free
deb-src http://deb.debian.org/debian bullseye main contrib non-free

deb http://deb.debian.org/debian bullseye-updates main contrib non-free
deb-src http://deb.debian.org/debian bullseye-updates main contrib non-free

deb http://security.debian.org/debian-security/ bullseye-security main contrib non-free
deb-src http://security.debian.org/debian-security/ bullseye-security main contrib non-free";
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
pub const HELP_EN_US: &str = "

Use mild --[option]-[distribution]-[interface]

    --install-archlinux-lxde => Install Minimal Lxde Desktop on ArchLinux.
    --install-archlinux-lxqt => Install Minimal Lxqt Desktop on ArchLinux.
    --install-archlinux-xfce => Install Minimal Xfce4 Desktop on ArchLinux.
    --install-archlinux-gnome => Install Minimal Gnome Desktop on ArchLinux.
    --install-archlinux-cinnamon => Install Minimal Cinnamon Desktop on ArchLinux.
    --install-archlinux-mate => Install Minimal Mate Desktop on ArchLinux.
    --install-archlinux-kdeplasma => Install Minimal Kde Plasma Desktop on ArchLinux.
    --install-archlinux-bspwm => Install Minimal Bspwm Desktop on ArchLinux.
    --install-archlinux-cutefish => Install Minimal Cutefish Desktop on ArchLinux.
    --clean-archlinux => Cleans up ArchLinux by removing cache and orphaned packages from the system
    
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

    --install-submodule => Install community scripts apart from mild.
    --build-submodule => Create mild submodules.

    --help => View a usage guide.

";