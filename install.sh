#!/bin/sh

echo "Log in as root" ;
sleep 5 ;
sudo su &&
cp app/mild /usr/bin/ &&
chmod +x /usr/bin/mild &&
echo "Mild has been installed" ;