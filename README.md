# MILD - Minimal Install Linux Desktop
MILD is a simple and straightforward text-mode installer that aims to install a "D.E."(Desktop Environment) with only the apps really necessary for each graphical environment. See below how to install in your favorite distro:

## Fedora 34
First download fedora 34 iso netinstall from the official website, after that make a bootable pendrive, with fedora 34 iso netinstall and configure in BIOS, to boot from pendrive. After that you will see this screen and on it you should select "Test this media & install Fedora 34", to check if the iso on the pendrive is corrupted and install Fedora 34:

![](imagens/iniciofedora34.webp)

After that you will see this screen and on it select your language, and after that click on continue:

![](imagens/idiomafedora34.webp)

After that you will see the screen below and on it select "Installation destination", and then select the disk that will be installed as well as partitioned for the installation of Fedora 34, **Attention we are not responsible for any data loss**, and we recommend to back up your data on a separate drive and preferably disconnected from your pc at least during installation, and also check the box "Advanced Customization ( Blivet GUI )", and then click "Done":

![](imagens/configsfedora34.webp)

Right after it will open the disk partitioning screen, and in it select the free space and click "+" to create a partition.

![](imagens/discofedora34.webp)

On **UEFI** systems make the following partitions (Remembering that the order does not matter) and then click "Done":

Order | Type | Size
------|------|--------
  1°  | efi  | 500MB  
  2°  | swap | The size of your ram.
  3°  | root | 70GB   
  4°  | home | Everything else

![](imagens/discouefifedora34.webp)

On **Legacy** systems make the following partitions (Remembering that the order does not matter) and then click "Done":

Order | Type | Size
------|------|--------  
  1°  | swap | The size of your ram.
  2°  | root | 70GB   
  3°  | home | Everything else

![](imagens/discolegacyfedora34.webp)

Then you will return to this screen, there you can "Network and hostname" to configure the wifi if you are on a notebook or want to change the hostname which by default is "fedora" and you can also select the keyboard layout that by default is already determined automatically by clicking "Keyboard" if the keyboard is not correct.

![](imagens/redefedora34.webp)

After these settings click on "Program selection" and just check "Minimum installation" like the image below and if you want to use wifi, check also "Common NetworkManager sub-modules":

![](imagens/1-pacotesfedora34.webp)

And if your PC doesn't have wifi, just select "Minimum installation":

![](imagens/2-pacotesfedora34.webp)

Now click "Root Password" to create the root user password, and click "Done":

![](imagens/rootfedora34.webp)

Now click "User creation" to create the password and user, and click "Done":

![](imagens/usuariofedora34.webp)

After that click on "Start installation" and wait a few minutes, remembering that the time may vary depending on your pc's power and your internet speed. After completion, restart your pc and remove the pendrive.

### Pós instalação

Now you will be at the terminal screen, to finish the system installation, first login as user **root** after that, type the following commands:

**The 1° to install git:**

	dnf install git -y

**The 2° to clone this repository:**

	git clone https://github.com/PEDROTUBER44/mild.git

**The 3° to enter into the cloned repository folder:**

	cd mild

**The 4° to give execute permission to the mild installation file:**

	chmod +x install.sh

**The 5° to install mild on your PC:**

	./install.sh

****

Now choose which graphical environment will be installed on your PC:

### Lxde

	mild --install-fedora-lxde

![](imagens/fedora-lxde.webp)

---

### Lxqt

	mild --install-fedora-lxqt

![](imagens/fedora-lxqt.webp)

---

### Xfce

	mild --install-fedora-xfce

![](imagens/fedora-xfce.webp)

---

### Gnome

	mild --install-fedora-gnome

![](imagens/fedora-gnome.webp)

---

### Mate

	mild --install-fedora-mate

![](imagens/fedora-mate.webp)

---

### Kde Plasma

	mild --install-fedora-kdeplasma

![](imagens/fedora-kdeplasma.webp)

---

### Cinnamon

	mild --install-fedora-cinnamon

![](imagens/fedora-cinnamon.webp)

After that you will have a clean desktop environment with no redundant apps on your **Fedora 34 !**

****

## Debian 11

First download debian 11 iso netinstall from the official site, after that make a bootable pendrive, with debian 11 iso netinstall and configure in BIOS, to boot from pendrive. After that you will see this screen and on it you should select "Graphical Install" to install debian graphically:

![](imagens/iniciodebian11.webp)

After that, select the system language, which in my case I chose "Portuguese (Brazil)":

![](imagens/idiomadebian11.webp)

Now select your location:

![](imagens/localidadedebian11.webp)

Now select your keyboard layout:

![](imagens/layoutdotecladodebian11.webp)

Now give your PC on the network a name:

![](imagens/hostnamedebian11.webp)

In this part click on "Continue":

![](imagens/dominiodebian11.webp)

Now write the root password:

![](imagens/rootdebian11.webp)

Write your name that will be used for your username (No spaces):

![](imagens/contadebian11.webp)

Escreva o nome do usuário novamente (Sem espaços):

![](imagens/usuariodebian11.webp)

Type the username again (No spaces):

![](imagens/senhausuariodebian11.webp)

Select the time zone closest to you (In my case I'm closer to São Paulo):

![](imagens/horariodebian11.webp) 

Now partition your disk according to your bios:

On **UEFI** systems make the following partitions (Remembering that the order does not matter) and select "Finish partitioning and write changes to disk", then click "Continue":

Order | Type | Size
------|------|--------
  1°  | efi  | 500MB  
  2°  | swap | 2GB    
  3°  | root | 70GB   
  4°  | home | Todo o resto*

On **Legacy** systems make the following partitions (Remembering that the order does not matter) and select "Finish partitioning and write changes to disk", then click "Continue":

Order | Type | Size
------|------|--------  
  1°  | swap | 2GB    
  2°  | root | 70GB   
  3°  | home | Todo o resto*

![](imagens/escreverdebian11.webp)

Now wait for the installation of the base system:

![](imagens/basicodebian11.webp)

On this screen select "No" and click "Continue":

![](imagens/midiaadicionaldebian11.webp)

Select the country for the installer to list the closest repositories:

![](imagens/repositoriodebian11.webp)

Now select "deb.debian.org" and click "Continue":

![](imagens/1-repositoriodebian11.webp)

Leave blank and click "Continue" if you don't know or don't use a proxy:

![](imagens/proxydebian11.webp)

Now check "No" and click "Continue":

![](imagens/telemetriadebian11.webp)

Select "Default System Utilities" and click "Continue":

![](imagens/softwaredebian11.webp)

Select "Yes" and click "Continue" to install grub:

![](imagens/grubdebian11.webp)

Now select the disk that will be installed grub:

![](imagens/discogrubdebian11.webp)

After completion click "Continue" and remove your pendrive:

![](imagens/fimdebian11.webp)

### Pós instalação

Now you will be at the terminal screen, to finish the system installation, first login as user **root** after that, type the following commands:

**The 1° to install git:**

	dnf install git -y

**The 2° to clone this repository:**

	git clone https://github.com/PEDROTUBER44/mild.git

**The 3° to enter into the cloned repository folder:**

	cd mild

**The 4° to give execute permission to the mild installation file:**

	chmod +x install.sh

**The 5° to install mild on your PC:**

	./install.sh

****

Now choose which graphical environment will be installed on your PC:

### Lxde

	mild --install-debian-lxde

![](imagens/debian-lxde.webp)

---

### Lxqt

	mild --install-debian-lxqt

![](imagens/debian-lxqt.webp)

---

### Xfce

	mild --install-debian-xfce

![](imagens/debian-xfce.webp)

---

### Gnome

	mild --install-debian-gnome

![](imagens/debian-gnome.webp)

---

### Mate

	mild --install-debian-mate

![](imagens/debian-mate.webp)

---

### Kde Plasma

	mild --install-debian-kdeplasma

![](imagens/debian-kdeplasma.webp)

---

### Cinnamon

	mild --install-debian-cinnamon

![](imagens/debian-cinnamon.webp)

After that you will have a clean desktop environment with no redundant apps on your **Debian 11 !**


****

## ArchLinux

First download the ArchLinux iso from the official website, after that make a bootable pendrive, with the archlinux iso and configure in the BIOS, to boot from the pendrive. After that you will see this screen and on it you should select "Arch Linux install medium" to install ArchLinux:

![](imagens/inicioarchlinux.webp)

Now install Arch Linux as you usually install but when you install the clone graphical interface and install mild as **root**, with the following commands:

**The 1° to install git:**

	dnf install git -y

**The 2° to clone this repository:**

	git clone https://github.com/PEDROTUBER44/mild.git

**The 3° to enter into the cloned repository folder:**

	cd mild

**The 4° to give execute permission to the mild installation file:**

	chmod +x install.sh

**The 5° to install mild on your PC:**

	./install.sh

****

Now choose which graphical environment will be installed on your PC:

### Lxde

	mild --install-arch-lxde

![](imagens/archlinux-lxde.webp)

---

### Lxqt

	mild --install-arch-lxqt

![](imagens/archlinux-lxqt.webp)

---

### Xfce

	mild --install-arch-xfce

![](imagens/archlinux-xfce.webp)

---

### Gnome

	mild --install-arch-gnome

![](imagens/archlinux-gnome.webp)

---

### Mate

	mild --install-arch-mate

![](imagens/archlinux-mate.webp)

---

### Kde Plasma

	mild --install-arch-kdeplasma

![](imagens/archlinux-kdeplasma.webp)

---

### Cinnamon

	mild --install-arch-cinnamon

![](imagens/archlinux-cinnamon.webp)

After that you will have a clean desktop environment with no redundant apps on your **Arch Linux !**

****

MILD - Minimal Install Linux Desktop
