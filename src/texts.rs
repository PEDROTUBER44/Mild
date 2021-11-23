pub const HELP_EN : &str = "

Use mild --[option]-[distro]-[interface]

    --install-arch-lxde => Install Minimal Lxde Desktop on ArchLinux.

    --install-arch-lxqt => Install Minimal Lxqt Desktop on ArchLinux.

    --install-arch-xfce => Install Minimal Xfce4 Desktop on ArchLinux.

    --install-arch-gnome => Install Minimal Gnome Desktop on ArchLinux.

    --install-arch-cinnamon => Install Minimal Cinnamon Desktop on ArchLinux.

    --install-arch-mate => Install Minimal Mate Desktop on ArchLinux.

    --install-arch-kdeplasma => Install Minimal Kde Plasma Desktop on ArchLinux.
    

    --install-debian-lxde => Install Minimal Lxde Desktop on Debian 11.

    --install-debian-lxqt => Install Minimal Lxqt Desktop on Debian 11.

    --install-debian-xfce => Install Minimal Xfce4 Desktop on Debian 11.

    --install-debian-gnome => Install Minimal Gnome Desktop on Debian 11.

    --install-debian-cinnamon => Install Minimal Cinnamon Desktop on Debian 11.

    --install-debian-mate => Install Minimal Mate Desktop on Debian 11.

    --install-debian-kdeplasma => Install Minimal Kde Plasma Desktop on Debian 11.


    --install-fedora-lxde => Install Minimal Lxde Desktop on Fedora 35.

    --install-fedora-lxqt => Install Minimal Lxqt Desktop on Fedora 35.

    --install-fedora-xfce => Install Minimal Xfce4 Desktop on Fedora 35.

    --install-fedora-gnome => Install Minimal Gnome Desktop on Fedora 35.

    --install-fedora-cinnamon => Install Minimal Cinnamon Desktop on Fedora 35.

    --install-fedora-mate => Install Minimal Mate Desktop on Fedora 35.

    --install-fedora-kdeplasma => Install Minimal Kde Plasma Desktop on Fedora 35.

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

pub const HELP_PT_BR : &str = "

Use mild --[opção]-[distro]-[interface]

    --install-arch-lxde => Para instalar o Lxde Desktop Minimal no ArchLinux.

    --install-arch-lxqt => Para instalar o Lxqt Desktop Minimal no ArchLinux.

    --install-arch-xfce => Para instalar o Xfce4 Desktop Minimal no ArchLinux.

    --install-arch-gnome => Para instalar o Gnome Desktop Minimal no ArchLinux.

    --install-arch-cinnamon => Para instalar o Cinnamon Desktop Minimal no ArchLinux.

    --install-arch-mate => Para instalar o Mate Desktop Minimal no ArchLinux.

    --install-arch-kdeplasma => Para instalar o Kde Plasma Desktop Minimal no ArchLinux.


    --install-debian-lxde => Para instalar o Lxde Desktop Minimal no Debian 11.

    --install-debian-lxqt => Para instalar o Lxqt Desktop Minimal no Debian 11.

    --install-debian-xfce => Para instalar o Xfce4 Desktop Minimal no Debian 11.

    --install-debian-gnome => Para instalar o Gnome Desktop Minimal no Debian 11.

    --install-debian-cinnamon => Para instalar o Cinnamon Desktop Minimal no Debian 11.

    --install-debian-mate => Para instalar o Mate Desktop Minimal no Debian 11.

    --install-debian-kdeplasma => Para instalar o Kde Plasma Desktop Minimal no Debian 11.


    --install-fedora-lxde => Para instalar o Lxde Desktop Minimal no Fedora 35.

    --install-fedora-lxqt => Para instalar o Lxqt Desktop Minimal no Fedora 35.

    --install-fedora-xfce => Para instalar o Xfce4 Minimal no Fedora 35.

    --install-fedora-gnome => Para instalar o Gnome Desktop Minimal no Fedora 35.

    --install-fedora-cinnamon => Para instalar o Cinnamon Desktop Minimal no Fedora 35.

    --install-fedora-mate => Para instalar o Mate Desktop Minimal no Fedora 35.

    --install-fedora-kdeplasma => Para instalar o Kde Plasma Desktop Minimal no Fedora 35.

    --help => Para exibir esse guia de uso.

";
