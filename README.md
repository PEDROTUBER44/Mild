# MILD - Minimal Install Linux Desktop

MILD is a user-friendly and efficient text-based installer designed to install a Desktop Environment (D.E.) along with only the essential applications required for each specific graphical environment.

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
<<<<<<< HEAD


## Post Install

To start, press the keys ´Ctrl´ + ´Alt´ + ´F2´ and log in with the user you created earlier.

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


## Desktops Enviroments Fedora 37

You can now select which graphical environment you want to install on your PC:

### Lxde

  mild --install-fedora-lxde

<!-- image -->

### Lxqt

  mild --install-fedora-lxde

<!-- image -->

### Xfce4

	mild --install-fedora-xfce

<!-- image -->

### Gnome

	mild --install-fedora-gnome

<!-- image -->

### Mate

	mild --install-fedora-mate

<!-- image -->

### Kde plasma

	mild --install-fedora-kdeplasma

<!-- image -->

### Cinnamon

	mild --install-fedora-cinnamon

<!-- image -->

Upon completing all of these steps, you will have a thoroughly cleaned and optimized system that is suitable for your low-powered computer.

****
=======
>>>>>>> 6ee00b67679768d061a4e235be0bd489f8905a83
