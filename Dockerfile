FROM ubuntu:latest

RUN apt update -y
RUN apt upgrade -y
RUN apt install build-essential git cargo rustc -y
RUN git clone https://www.github.com/PEDROTUBER44/Mild.git
RUN mkdir /home/user/
RUN mv Mild/ /home/user/