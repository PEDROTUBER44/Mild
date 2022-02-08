#!/bin/sh

echo "Log in as root" &&
sudo su &&

cp app/mild /usr/bin/ ;
chmod +x /usr/bin/mild ;
echo "Mild installed in /usr/bin" &&