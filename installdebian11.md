# Install Debian 11

First [Download](https://www.debian.org/CD/netinst/) the Debian 11 iso, then create a bootable flash drive, now configure your BIOS to boot from the flash drive. After that you will see this screen and on it you should select "Graphical Install", to install Debian 11:

![](images/debian11/debian11-start.webp)

Now select the system language and continue the installation:

![](images/debian11/debian11-language.webp)

Now select the country where you live:

![](images/debian11/debian11-location.webp)

Now choose your keyboard layout and variant:

![](images/debian11/debian11-keymap.webp)

Now choose the name your pc will have on the network:

![](images/debian11/debian11-namenetwork.webp)

On this screen, click "continue":

![](images/debian11/debian11-domainname.webp)

Now enter the root user password that will be created:

![](images/debian11/debian11-rootpassword.webp)

Now enter your first and second name:

![](images/debian11/debian11-username.webp)

Now type only the first lowercase name that will be used in your "/home":

![](images/debian11/debian11-username1.webp)

Now enter the password of this user that will be created:

![](images/debian11/debian11-userpassword.webp)

Select time zone:

![](images/debian11/debian11-timezone.webp)

Now partition your disk according to your BIOS:

![](images/debian11/debian11-partition.webp)

On **UEFI** systems create the following partitions and then click "Continue":

Order | Type | Size
------|------|------
1°    | EFI  | 500MB
2°    | SWAP | The size of your ram memory.
3°    | ROOT | 50GB
4°    | HOME | Everything else.

On **Legacy** systems create the following partitions and then click "Continue":

Order | Type | Size
------|------|------
1°    | SWAP | The size of your ram memory.
2°    | ROOT | 50GB
3°    | HOME | Everything else.

When you are done click on "Finish partitioning and write changes to disk":

![](images/debian11/debian11-partition-end.webp)

Now check the "Yes" box and click `Enter`:

![](images/debian11/debian11-write-changes.webp)

Now wait for the system base installation to finish:

![](images/debian11/debian11-baseinstall.webp)

Check the "No" box and click `Enter`:

![](images/debian11/debian11-additional-media.webp)

Now choose the mirror closest to you and press `Enter`:

![](images/debian11/debian11-archive.webp)

![](images/debian11/debian11-archive1.webp)

Here leave empty and click "Continue":

![](images/debian11/debian11-proxy.webp)

Select "No" and click `Enter`:

![](images/debian11/debian11-telemetry.webp)

Check only this box and click "Continue":

![](images/debian11/debian11-packages.webp)

Wait now for the installation of the Debian 11 Core:

![](images/debian11/debian11-core.webp)

On this screen, select the "Yes" box and click "Continue":

![](images/debian11/debian11-grub.webp)

Now select the disk where grub will be installed and click "Continue":

![](images/debian11/debian11-install-grub.webp)

Now click on continue and remove the flash drive or installation media and follow the next steps:

![](images/debian11/debian11-end.webp)

****

## Post installation

First of all log in as **root** update your system and install nano, git and sudo and edit the file "/etc/sudoers" with the following command. And add your user to the sudo group:

    apt install nano git sudo && nano /etc/sudoers

After editing the file, type the following command:

    reboot

Now follow the Mid installation tutorial that is on the home page