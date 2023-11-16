---
description: Avoiding building from source when you can
---

# Dependency Hell

To test if anything would work in the first place I had to write and test basic NGINX modules in C. This required installing NGINX from a source on my Linux machine. Anyone who's installed anything from source understands how time-consuming chasing up outdated dependencies can be.

The big issue even if you do manage to get everything working on your machine after a while, how can you guarantee it will work for another developer? Hence it's crucial for this barrier to be low if this project is to thrive in the open source community.

> But it works on my machine!

An easy way to guarantee it'll work on any other computer is my containerising. Developers won't be forced to use a virtualisation tool like Docker but having a clear Dockerfile in the code base will at least provide guidance on how to setup the system correctly.&#x20;

The following Dockerfile builds NGINX from the source and configures the environment for NGINX module development.&#x20;

```docker
FROM debian:stable
# FROM debian:slim-bullseye to save some bytes

# Update the package repository and install the necessary dependencies
RUN apt-get update && apt-get install -y \
    build-essential \
    wget \
    git \
    curl \
    zlib1g-dev \
    libpcre3-dev \
    libpcre2-dev \
    libssl-dev \
    llvm-dev \
    libclang-dev \
    clang \
    automake \
    autoconf \
    libtool \
    vim

# Download Nginx source code
WORKDIR /nginx

ENV NGINX_VER=1.14.2
RUN git config --global http.sslVerify false
RUN wget --no-check-certificate https://nginx.org/download/nginx-${NGINX_VER}.tar.gz
RUN tar -xzvf nginx-${NGINX_VER}.tar.gz
RUN rm nginx-${NGINX_VER}.tar.gz

# Configure and compile Nginx with dynamic module support
WORKDIR /nginx/nginx-${NGINX_VER}
RUN ./configure --with-compat
RUN make
RUN make install
RUN cp objs/nginx /usr/sbin/nginx

# default dynamic modules directory
RUN mkdir /usr/local/nginx/modules

EXPOSE 80

# Start Nginx
# Define the ENTRYPOINT as the NGINX executable
ENTRYPOINT ["nginx", "-g", "daemon off;"]
```
