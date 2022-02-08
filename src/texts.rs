pub const HELP_EN : &str = "

Use mild --[option]-[distro]-[interface]

    --install-arch-lxde => Install Minimal Lxde Desktop on ArchLinux.

    --install-arch-lxqt => Install Minimal Lxqt Desktop on ArchLinux.

    --install-arch-xfce => Install Minimal Xfce4 Desktop on ArchLinux.

    --install-arch-gnome => Install Minimal Gnome Desktop on ArchLinux.

    --install-arch-cinnamon => Install Minimal Cinnamon Desktop on ArchLinux.

    --install-arch-mate => Install Minimal Mate Desktop on ArchLinux.

    --install-arch-kdeplasma => Install Minimal Kde Plasma Desktop on ArchLinux.

    --clean-arch => Cleans up ArchLinux by removing cache and orphaned packages from the system
    

    --install-debian-lxde => Install Minimal Lxde Desktop on Debian 11.

    --install-debian-lxqt => Install Minimal Lxqt Desktop on Debian 11.

    --install-debian-xfce => Install Minimal Xfce4 Desktop on Debian 11.

    --install-debian-gnome => Install Minimal Gnome Desktop on Debian 11.

    --install-debian-cinnamon => Install Minimal Cinnamon Desktop on Debian 11.

    --install-debian-mate => Install Minimal Mate Desktop on Debian 11.

    --install-debian-kdeplasma => Install Minimal Kde Plasma Desktop on Debian 11.

    --clean-debian => Cleans up Debian 11 by removing cache and orphaned packages from the system


    --install-fedora-lxde => Install Minimal Lxde Desktop on Fedora 35.

    --install-fedora-lxqt => Install Minimal Lxqt Desktop on Fedora 35.

    --install-fedora-xfce => Install Minimal Xfce4 Desktop on Fedora 35.

    --install-fedora-gnome => Install Minimal Gnome Desktop on Fedora 35.

    --install-fedora-cinnamon => Install Minimal Cinnamon Desktop on Fedora 35.

    --install-fedora-mate => Install Minimal Mate Desktop on Fedora 35.

    --install-fedora-kdeplasma => Install Minimal Kde Plasma Desktop on Fedora 35.

    --clean-fedora => Cleans up Fedora 35 by removing cache and orphaned packages from the system.


    --help => View a usage guide.

";

pub const DNF : &str = "
[main]
gpgcheck=1
installonly_limit=3
clean_requirements_on_remove=True
best=False
skip_if_unavailable=True
fastestmirror=True
max_parallel_downloads=7
defaultyes=True
install_weak_deps=false
";