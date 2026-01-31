FROM ubuntu:24.04

ENV LANG="en_US"

# Install bash dependencies
RUN apt update && apt install -y openjdk-21-jdk python3-pip
RUN python3 -m pip install --upgrade setuptools --break-system-packages
RUN python3 -m pip install requests --break-system-packages
RUN python3 -m pip install aurora-cli --break-system-packages
