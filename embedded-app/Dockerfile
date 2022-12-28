# big ups to https://capnfabs.net/posts/cross-compiling-rust-apps-raspberry-pi/

FROM rustembedded/cross:armv7-unknown-linux-gnueabihf-0.2.1

RUN apt-get update
RUN dpkg --add-architecture armhf
RUN apt-get update
RUN apt-get install --assume-yes libasound2-dev:armhf
RUN apt-get install --assume-yes libc6-dev:armhf

ENV PKG_CONFIG_LIBDIR_armv7_unknown_linux_gnueabihf=/usr/lib/arm-linux-gnueabihf/pkgconfig