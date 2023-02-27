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
