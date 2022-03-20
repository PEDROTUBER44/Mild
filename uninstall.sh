#!/bin/sh

echo "Log in as root" ;
sleep 5 ;
sudo su &&
rm -r /usr/bin/mild &&
echo "Mild was successfully removed" ;