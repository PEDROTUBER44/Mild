# Install ArchLinux

First [Download](https://archlinux.org/download/) the ArchLinux iso, then create a bootable flash drive and configure the BIOS to boot from the flash drive. Now follow the tutorial according to your PC:

## Install ArchLinux on UEFI

Now on this screen select the option "Arch Linux install medium (x86_64,UEFI)", to install ArchLinux on your PC:

![](images/archlinux/archlinux-start-uefi.webp)

Now type the following command to comfigure the keyboard layout if it is wrong:

    loadkeys br-abnt2

Then type this command to list all disks plugged into the PC. With the unit name in hand, go to the next step:

    fdisk -l

Now partition the disk you got in the previous command with the following command. **Attention we are not responsible for any loss of data. We recommend that before partitioning the disk you backup your data**:

    cfdisk /dev/sda

On this screen select the **GPT** partition scheme:

![](images/archlinux/archlinux-partition-scheme.webp)

As your PC's BIOS is **UEFI**, create the following partitions, after you have created these partitions, click **write** and then **quit**:

Order | Type | Size
------|------|------
1°    | EFI  | 500MB
2°    | SWAP | The size of your ram memory.
3°    | ROOT | 50GB
4°    | HOME | Everything else.

![](images/archlinux/archlinux-cfdisk-uefi.webp)

Now format the **EFI** partition with the following command:

    mkfs.vfat /dev/sda1

Also format the **SWAP** partition with the following command:

    mkswap /dev/sda2 -L "SWAP" -f

Now format the **ROOT** partition with the following command:

    mkfs.xfs /dev/sda3 -L "ROOT" -f

Finally format the **HOME** partition with the following command:

    mkfs.xfs /dev/sda4 -L "HOME" -f

Now mount the **ROOT** partition on /mnt with the command:

    mount /dev/sda3 /mnt

Create now create the directories **/home/** and **/boot/efi/** with the command:

    mkdir -p /mnt/home/ && mkdir -p /mnt/boot/efi/

Now mount the **/mnt/boot/efi/** directory on the **EFI** partition with the command:

    mount /dev/sda1 /mnt/boot/efi

Now mount the **/home** directory on the **HOME** partition with the command:

    mount /dev/sda4 /mnt/home

Now enable the swap partition with the command:

    swapon /dev/sda2

Now download the basic packages like: Kernel, NetworkManager, Grub... with the command:

    pacstrap /mnt nano git base base-devel linux-zen linux-firmware dosfstools os-prober mtools network-manager-applet networkmanager wpa_supplicant wireless_tools dialog sudo grub-efi-x86_64 efibootmgr

Now generate the fstab table that tells the system where each partition is mounted:

    genfstab -U -p /mnt >> /mnt/etc/fstab

Now log into your system:

    arch-chroot /mnt

Change the system date and time with the time zone closest to you in my case São Paulo is the time zone closest to where I am:

    ln -sf /usr/share/zoneinfo/America/Sao_Paulo /etc/localtime

Synchronize system time with **BIOS**:

    hwclock --systohc

Now uncomment in the file "/etc/locale.gen" the language that will be used in the system:

    nano /etc/locale.gen

Enter this command to generate the locale:

    locale-gen

Now send some environment variables to the "vconsole.conf" and "/etc/locale.conf" files:

    echo LANG=pt_BR.UTF-8 >> /etc/locale.conf && echo KEYMAP=br-abnt2 >> /etc/vconsole.conf

Send a variable containing the name of your PC on the network to the "/etc/hostname" file. Replace mycomputername with whatever name you want:

    echo "mycomputername" >> /etc/hostname

Now edit the "/etc/hosts" file and add the following lines inside the file. Replace mycomputername with whatever name you want:

    127.0.0.1   localhost.localdomain   localhost
    ::1 localhost.localdomain   localhost
    127.0.1.1   mycomputername.localdomain  mycomputername

Now set the **root** user password:

    passwd

Add your username now. Replace "mild" with the desired username:

    useradd -m -g users -G wheel mild

Now set your user password. Replace "mild" with the username created above:

    passwd mild

Now add your user to the sudo group in the "/etc/sudoers" file removing all the lines and putting only these. Replace "mild" with your username:

    root ALL=(ALL:ALL) ALL
    mild ALL=(ALL:ALL) ALL
    @includedir /etc/sudoers.d

Now install grub on your PC with the command:

    grub-install --target=x86_64-efi --efi-directory=/boot/efi --bootloader-id=arch_grub --recheck

After this command type this:

    cp /usr/share/locale/en@quot/LC_MESSAGES/grub.mo /boot/grub/locale/en.mo

And finally this command:

    grub-mkconfig -o /boot/grub/grub.cfg

Now type **exit** and then **reboot**.

## Install ArchLinux on Legacy

Now on this screen select the option "Arch Linux install medium (x86_64,BIOS)", to install ArchLinux on your PC:

![](images/archlinux/archlinux-start-legacy.webp)

Now type the following command to configure the keyboard layout if it is wrong:

    loadkeys br-abnt2

Then type this command to list all disks plugged into the PC. With the unit name in hand, go to the next step:

    fdisk -l

Now partition the disk you got in the previous command with the following command. **Attention we are not responsible for any loss of data. We recommend that before partitioning the disk you backup your data**:

    cfdisk /dev/sda

On this screen select the **DOS** partition scheme:

![](images/archlinux/archlinux-partition-scheme.webp)

As your PC's BIOS is **Legacy** create the following partitions, after you have created these partitions click **write** and then **quit**:

Order | Type | Size
------|------|------
1°    | SWAP | The size of your ram memory.
2°    | ROOT | 50GB
3°    | HOME | Everything else.

![](images/archlinux/archlinux-cfdisk-legacy.webp)

Also format the **SWAP** partition with the following command:

    mkswap /dev/sda1 -L "SWAP" -f

Now format the **ROOT** partition with the following command:

    mkfs.xfs /dev/sda2 -L "ROOT" -f

Finally format the **HOME** partition with the following command:

    mkfs.xfs /dev/sda3 -L "HOME" -f

Now mount the **ROOT** partition on /mnt with the command:

    mount /dev/sda2 /mnt

Create now create the directories **/home** and **/boot** with the command:

    mkdir -p /mnt/home/ && mkdir -p /mnt/boot/

Now mount the **/home** directory on the **HOME** partition with the command:

    mount /dev/sda3 /mnt/home

Now enable the swap partition with the command:

    swapon /dev/sda1

Now download the basic packages like: Kernel, NetworkManager, Grub... with the command:

    pacstrap /mnt nano git base base-devel linux-zen linux-firmware dosfstools os-prober mtools network-manager-applet networkmanager wpa_supplicant wireless_tools dialog sudo grub

Now generate the fstab table that tells the system where each partition is mounted:

    genfstab -U -p /mnt >> /mnt/etc/fstab

Now log into your system:

    arch-chroot /mnt

Change the system date and time with the time zone closest to you in my case São Paulo is the time zone closest to where I am:

    ln -sf /usr/share/zoneinfo/America/Sao_Paulo /etc/localtime

Synchronize system time with **BIOS**:

    hwclock --systohc

Now uncomment in the file "/etc/locale.gen" the language that will be used in the system:

    nano /etc/locale.gen

Now send some environment variables to the files "vconsole.conf" and "/etc/locale.conf":

    echo LANG=pt_BR.UTF-8 >> /etc/locale.conf && echo KEYMAP=br-abnt2 >> /etc/vconsole.conf

Send a variable containing the name of your PC on the network to the "/etc/hostname" file. Replace mycomputername with whatever name you want:

    echo "mycomputername" >> /etc/hostname

Now edit the "/etc/hosts" file and add the following lines inside the file. Replace mycomputername with whatever name you want:

    127.0.0.1   localhost.localdomain   localhost
    ::1 localhost.localdomain   localhost
    127.0.1.1   mycomputername.localdomain  mycomputername

Now set the **root** user password:

    passwd

Create your user now. Replace "mild" with the desired username:

    useradd -m -g users -G wheel mild

Now set your user password. Replace "mild" with the username you created above:

    passwd mild

Now add your user to the sudo group in the "/etc/sudoers" file removing all the lines and putting only these. Replace "mild" with your username:

    root ALL=(ALL:ALL) ALL
    mild ALL=(ALL:ALL) ALL
    @includedir /etc/sudoers.d

Now install grub on your PC with the command:

    grub-install --target=i386-pc --recheck /dev/sda --force

And right after this command:

    cp /usr/share/locale/en@quot/LC_MESSAGES/grub.mo /boot/grub/locale/en.mo

And finally this command:

    grub-mkconfig -o /boot/grub/grub.cfg

Now type **exit** and then **reboot**.

Now follow the Mid installation tutorial that is on the home page