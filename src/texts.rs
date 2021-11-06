pub const HELP : &str = "
    Use mild --[option]-[distro]-[interface]
    
    --install-arch-lxde => Install minimal lxde on archlinux.
    
    --install-arch-lxqt => Install minimal lxqt on archlinux.
    
    --install-arch-xfce => Install minimal xfce on archlinux.
    
    --install-arch-gnome => Install minimal gnome on archlinux.
    
    --install-arch-cinnamon => Install minimal cinnamon on archlinux.
    
    --install-arch-mate => Install minimal mate on archlinux.
    
    --install-arch-kdeplasma => Install minimal kde plasma on archlinux.
    
    
    --install-debian-lxde => Install minimal lxde on debian.
    
    --install-debian-lxqt => Install minimal lxqt on debian.
    
    --install-debian-xfce => Install minimal xfce on debian.
    
    --install-debian-gnome => Install minimal gnome on debian.
    
    --install-debian-cinnamon => Install minimal cinnamon on debian.
    
    --install-debian-mate => Install minimal mate on debian.
    
    --install-debian-kdeplasma => Install minimal kde plasma on debian.
    
    
    --install-fedora-lxde => Install minimal lxde on fedora.
    
    --install-fedora-lxqt => Install minimal lxqt on fedora.
    
    --install-fedora-xfce => Install minimal xfce on fedora.
    
    --install-fedora-gnome => Install minimal gnome on fedora.
    
    --install-fedora-cinnamon => Install minimal cinnamon on fedora.
    
    --install-fedora-mate => Install minimal mate on fedora.
    
    --install-fedora-kdeplasma => Install minimal kde plasma on fedora.
    
    --help => View a usage guide
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