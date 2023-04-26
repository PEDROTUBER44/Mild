# MILD - Minimal Install Linux Desktop

MILD is a user-friendly and efficient text-based installer designed to install a Desktop Environment (D.E.) along with only the essential applications required for each specific graphical environment.


## ArchLinux

To install ArchLinux with mild, start by downloading the iso directly from the distribution's official website. Then create a bootable USB stick using a program like balena etcher. Note that this step will delete all data on your USB stick, so proceed with caution. When the bootable USB stick is ready, restart your computer and boot from the USB stick. You will see a screen where you can select the option “Arch Linux install medium (x86_64, UEFI)”. This option verifies if the ISO file of the pendrive is corrupted or not, if not, it will proceed with the installation of the system.

![image](https://user-images.githubusercontent.com/77652411/225134880-42febd9e-0400-4cbc-b2a3-45626c5cd30e.jpg)

Then, proceed to follow the step-by-step instructions below to complete the pre-installation of Archlinux:

## UEFI

-Archlinux Install UEFI Video-

For optimal system performance, it is recommended to use the partition scheme provided below:

Order |  Type  | Size
------|--------|--------
  1°  |  efi   | 500MB
  2°  |  swap  | The Size Of Your Ram Memory
  3°  | system | Everything Else


## BIOS/LEGACY

-Archlinux Install BIOS Video-

For optimal system performance, it is recommended to use the partition scheme provided below:

Order |  Type  | Size
------|--------|------
  1°  |  bios  | 2MIB
  2°  |  swap  | The Size Of Your Ram Memory
  3°  | system | Everything Else


## Post Install

To start, press the keys `Ctrl` + `Alt` + `F2` and log in with the user you created earlier.

-Tela de Login do Archlinux-

Select the option that best suits your needs: install the precompiled package or manually compile it on your own machine:

### Make

Before proceeding, it is important to update your system to the latest version available. To do this, type the following command:

	sudo pacman -Syu

Install the Rust and git compiler with this command:

	sudo pacman -Syu curl rust git -y

To clone the Mild repository (Minimal Installation Linux Desktop), use the **git** command and run the following instruction:

	git clone https://www.github.com/PEDROTUBER44/Mild.git

To access the Mild folder, use the following command:

	cd Mild/

Prior to modifying the **build.sh** and **install.sh** files, grant them execution permissions by running the following command:

	chmod +x build.sh && chmod +x install.sh

To conclude, execute the "build.sh" and "install.sh" files by running the following command:

	./build.sh && ./install.sh

### Uninstall

To remove Mild and its build dependencies, use the following commands:

	sudo pacman -Rsn curl rust git -y && cd .. && rm -rf Mild/ && sudo rm -r /bin/mild

You can now select the graphical environment that you want to install on your computer [Desktop Enviroments](#desktops-enviroments-archlinux).


### Non-Make

Before proceeding, it is important to update your system to the latest version available. To do this, type the following command:

	sudo pacman -Syu

Install the Rust and git compiler with this command:

	sudo pacman -Syu git -y

To clone the Mild repository (Minimal Installation Linux Desktop), use the **git** command and run the following instruction:

	git clone https://www.github.com/PEDROTUBER44/Mild.git

To access the Mild folder, use the following command:

	cd Mild/

Prior to modifying the **install.sh** file, grant them execution permissions by running the following command:

	chmod +x install.sh

To conclude, execute the "install.sh" file by running the following command:

	./install.sh

### Uninstall

To remove Mild and its build dependencies, use the following commands:

	sudo pacman -Rsn git -y && cd .. && rm -rf Mild/ && sudo rm -r /bin/mild

You can now select the graphical environment that you want to install on your computer [Desktop Enviroments](#desktops-enviroments-archlinux).


## Desktops Enviroments Archlinux

You can now select which graphical environment you want to install on your PC:

### Lxde

	mild --install-archlinux-lxde

![Archlinux Lxde](https://user-images.githubusercontent.com/77652411/228049028-75174e0b-68f2-4d68-b7eb-06fbcff807a6.png)


### Lxqt

	mild --install-archlinux-lxqt

![Archlinux Lxqt](https://user-images.githubusercontent.com/77652411/228049259-ab447cd7-35ad-4808-ba0f-5b47fb4d1fb5.png)


### Xfce4

	mild --install-archlinux-xfce

![Archlinux Xfce4](https://user-images.githubusercontent.com/77652411/229943552-e92f2377-8f35-452d-8e24-7704ae9fb8f1.png)

### Gnome

	mild --install-archlinux-gnome

![Archlinux Gnome](https://user-images.githubusercontent.com/77652411/229943723-8d202583-08b1-49db-bc75-655edbdace7c.png)

### Mate

	mild --install-archlinux-mate

![Archlinux Mate](https://user-images.githubusercontent.com/77652411/229944507-a749d511-c6a4-440b-8699-8bd3d1aa16ed.png)

### Kde plasma

	mild --install-archlinux-kdeplasma

![Archlinux Kdeplasma](https://user-images.githubusercontent.com/77652411/229943802-0fd94189-aff4-4847-b5a8-2fe672e5d57c.png)

### Cinnamon

	mild --install-archlinux-cinnamon

![Archlinux Cinnamon](https://user-images.githubusercontent.com/77652411/229954613-245e0c05-ef5a-4f96-9d40-55d271ca1e66.png)

Upon completing all of these steps, you will have a thoroughly cleaned and optimized system that is suitable for your low-powered computer.

****

## Debian 11

To install Debian 11 with mild, begin by downloading the netinstall version directly from the official website of the distribution. Next, create a bootable pendrive using a program like balena etcher. Please note that this step will delete all the data on your pendrive, so proceed with caution. Once the bootable pendrive is ready, restart your computer and boot it from the pendrive. You will then see a screen where you can select the option “Graphical Install”. This option verifies whether the ISO file on the pendrive is corrupted or not, and if it is not, it will proceed with installing the system.

![](https://user-images.githubusercontent.com/77652411/234676690-7fbb5a6b-691b-4ca1-a534-0557e2acd05b.png)


Then, proceed to follow the step-by-step instructions below to complete the pre-installation of Debian 11:


## UEFI

[](https://user-images.githubusercontent.com/77652411/234674635-d07bbff0-7ef8-4e54-8370-dc53cbb7f2ed.webm)

For optimal system performance, it is recommended to use the partition scheme provided below:

Order |  Type  | Size
------|--------|--------
  1°  |  efi   | 500MB
  2°  |  swap  | The Size Of Your Ram Memory
  3°  | system | Everything Else


## BIOS/LEGACY

-Debian 11 Install Legacy Video-

For optimal system performance, it is recommended to use the partition scheme provided below:

Order |  Type  | Size
------|--------|------
  1°  |  bios  | 2MIB
  2°  |  swap  | The Size Of Your Ram Memory
  3°  | system | Everything Else


## Post Install

To start, press the keys `Ctrl` + `Alt` + `F2` and log in with the user you created earlier.

-Tela de login do debian 11-

Select the option that best suits your needs: install the precompiled package or manually compile it on your own machine:

### Make

Before proceeding, it is important to update your system to the latest version available. To do this, type the following command:

	sudo apt update -y

Install the Rust and git compiler with this command:

	sudo apt install curl rust git -y

To clone the Mild repository (Minimal Installation Linux Desktop), use the **git** command and run the following instruction:

	git clone https://www.github.com/PEDROTUBER44/Mild.git

To access the Mild folder, use the following command:

	cd Mild/

Prior to modifying the **build.sh** and **install.sh** files, grant them execution permissions by running the following command:

	chmod +x build.sh && chmod +x install.sh

To conclude, execute the "build.sh" and "install.sh" files by running the following command:

	./build.sh && ./install.sh

### Uninstall

To remove Mild and its build dependencies, use the following commands:

	sudo apt remove curl rust git -y && cd .. && rm -rf Mild/ && sudo rm -r /bin/mild

You can now select the graphical environment that you want to install on your computer [Desktop Enviroments](#desktops-enviroments-debian-11).


### Non-Make

Before proceeding, it is important to update your system to the latest version available. To do this, type the following command:

	sudo apt update -y

Install the Rust and git compiler with this command:

	sudo apt install git -y

To clone the Mild repository (Minimal Installation Linux Desktop), use the **git** command and run the following instruction:

	git clone https://www.github.com/PEDROTUBER44/Mild.git

To access the Mild folder, use the following command:

	cd Mild/

Prior to modifying the **install.sh** file, grant them execution permissions by running the following command:

	chmod +x install.sh

To conclude, execute the "build.sh" and "install.sh" files by running the following command:

	./install.sh

### Uninstall

To remove Mild and its build dependencies, use the following commands:

	sudo apt remove git -y && cd .. && rm -rf Mild/ && sudo rm -r /bin/mild

You can now select the graphical environment that you want to install on your computer [Desktop Enviroments](#desktops-enviroments-debian-11).


## Desktops Enviroments Debian 11

You can now select which graphical environment you want to install on your PC:

### Lxde

	mild --install-debian-lxde

-Imagem Archlinux Lxde-

### Lxqt

	mild --install-debian-lxqt

-Imagem Archlinux Lxqt-

### Xfce4

	mild --install-debian-xfce

-Imagem Archlinux Xfce-

### Gnome

	mild --install-debian-gnome

-Imagem Archlinux Gnome-

### Mate

	mild --install-debian-mate

-Imagem Archlinux Mate-

### Kde plasma

	mild --install-debian-kdeplasma

-Imagem Archlinux KDE Plasma-

### Cinnamon

	mild --install-debian-cinnamon

-Imagem Archlinux Cinnamon-

Upon completing all of these steps, you will have a thoroughly cleaned and optimized system that is suitable for your low-powered computer.

****

## Fedora 37

To install Fedora 37 with mild, begin by downloading the netinstall version directly from the official website of the distribution. Next, create a bootable pendrive using a program like balena etcher. Please note that this step will delete all the data on your pendrive, so proceed with caution. Once the bootable pendrive is ready, restart your computer and boot it from the pendrive. You will then see a screen where you can select the option “Test this media & install Fedora 37”. This option verifies whether the ISO file on the pendrive is corrupted or not, and if it is not, it will proceed with installing the system.

![](Tela de Instalação)

Then, proceed to follow the step-by-step instructions below to complete the pre-installation of Fedora 37:

## UEFI

https://user-images.githubusercontent.com/77652411/219970332-cd97dc53-d04b-40ed-9278-6342dc98a207.mp4

For optimal system performance, it is recommended to use the partition scheme provided below:

Order |  Type  | Size
------|--------|--------
  1°  |  efi   | 500MB
  2°  |  swap  | The Size Of Your Ram Memory
  3°  | system | Everything Else


## BIOS/LEGACY

https://user-images.githubusercontent.com/77652411/219970336-0c494982-bbe7-43a6-ac76-2cf44b9f3ad8.mp4

For optimal system performance, it is recommended to use the partition scheme provided below:

Order |  Type  | Size
------|--------|------
  1°  |  bios  | 2MIB
  2°  |  swap  | The Size Of Your Ram Memory
  3°  | system | Everything Else


## Post Install

To start, press the keys `Ctrl` + `Alt` + `F2` and log in with the user you created earlier.

![](Tela de login no terminal)

Select the option that best suits your needs: install the precompiled package or manually compile it on your own machine:

### Make

Before proceeding, it is important to update your system to the latest version available. To do this, type the following command:

	sudo dnf update -y

Install the Rust and git compiler with this command:

	sudo dnf install cargo curl rust git -y

To clone the Mild repository (Minimal Installation Linux Desktop), use the **git** command and run the following instruction:

	git clone https://www.github.com/PEDROTUBER44/Mild.git

To access the Mild folder, use the following command:

	cd Mild/

Prior to modifying the **build.sh** and **install.sh** files, grant them execution permissions by running the following command:

	chmod +x build.sh && chmod +x install.sh

To conclude, execute the "build.sh" and "install.sh" files by running the following command:

	./build.sh && ./install.sh

### Uninstall

To remove Mild and its build dependencies, use the following commands:

	sudo dnf remove cargo curl rust git -y && cd .. && rm -rf Mild/ && sudo rm -r /bin/mild

You can now select the graphical environment that you want to install on your computer [Desktop Enviroments](#desktops-enviroments-fedora-37).


### Non-Make

Before proceeding, it is important to update your system to the latest version available. To do this, type the following command:

	sudo apt update -y

Install the Rust and git compiler with this command:

	sudo apt install git -y

To clone the Mild repository (Minimal Installation Linux Desktop), use the **git** command and run the following instruction:

	git clone https://www.github.com/PEDROTUBER44/Mild.git

To access the Mild folder, use the following command:

	cd Mild/

Prior to modifying the **install.sh** file, grant them execution permissions by running the following command:

	chmod +x install.sh

To conclude, execute the "build.sh" and "install.sh" files by running the following command:

	./install.sh

### Uninstall

To remove Mild and its build dependencies, use the following commands:

	sudo apt remove git -y && cd .. && rm -rf Mild/ && sudo rm -r /bin/mild

You can now select the graphical environment that you want to install on your computer [Desktop Enviroments](#desktops-enviroments-fedora-37).


## Desktops Enviroments Fedora 37

You can now select which graphical environment you want to install on your PC:

### Lxde

	mild --install-fedora-lxde

![Fedora Lxde](https://user-images.githubusercontent.com/77652411/221678282-f1d474da-1a93-434f-804d-507bc441233c.png)

### Lxqt

	mild --install-fedora-lxde

![Fedora Lxqt](https://user-images.githubusercontent.com/77652411/221678322-69a32256-0e68-42be-813e-b5ab2f711142.png)

### Xfce4

	mild --install-fedora-xfce

![Fedora Xfce](https://user-images.githubusercontent.com/77652411/221678384-0b0f66e7-5147-41df-afbc-f81804afe7be.png)

### Gnome

	mild --install-fedora-gnome

![Fedora Gnome](https://user-images.githubusercontent.com/77652411/221678430-4c5b8405-67be-4ee9-b38b-9d81348263b7.png)

### Mate

	mild --install-fedora-mate

![Fedora Mate](https://user-images.githubusercontent.com/77652411/221678489-4d97f4e2-896f-4fac-9a73-5a58a029f284.png)

### Kde plasma

	mild --install-fedora-kdeplasma

![Fedora KdePlasma](https://user-images.githubusercontent.com/77652411/221678545-9b22b4cd-9222-4d24-aad9-7d2e77fbc740.png)

### Cinnamon

	mild --install-fedora-cinnamon

![Fedora Cinnamon](https://user-images.githubusercontent.com/77652411/221678614-31c97602-796a-475e-94d9-a1258f28b37d.png)

Upon completing all of these steps, you will have a thoroughly cleaned and optimized system that is suitable for your low-powered computer.

****
