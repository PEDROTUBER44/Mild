# MILD - Minimal Install Linux Desktop

MILD is a user-friendly and efficient text-based installer designed to install a Desktop Environment (D.E.) along with only the essential applications required for each specific graphical environment.

## Fedora 37
To install Fedora 37 with mild, begin by downloading the netinstall version directly from the official website of the distribution. Next, create a bootable pendrive using a program like balena etcher. Please note that this step will delete all the data on your pendrive, so proceed with caution. Once the bootable pendrive is ready, restart your computer and boot it from the pendrive. You will then see a screen where you can select the option “Test this media & install Fedora 37”. This option verifies whether the ISO file on the pendrive is corrupted or not, and if it is not, it will proceed with installing the system.

![](media/fedora/desktopenviroments/installscreen.webp)

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
