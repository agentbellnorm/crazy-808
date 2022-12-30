# big ups to https://capnfabs.net/posts/cross-compiling-rust-apps-raspberry-pi/

FROM rustembedded/cross:armv7-unknown-linux-gnueabihf-0.2.1

RUN dpkg --add-architecture armhf

RUN apt-get update && apt-get install -y \
    wget \
    curl \
    unzip \
    libasound-dev:armhf \
    libc6-dev:armhf

# Download and extract protoc
RUN wget https://github.com/protocolbuffers/protobuf/releases/download/v3.14.0/protoc-3.14.0-linux-x86_64.zip
RUN unzip protoc-3.14.0-linux-x86_64.zip -d protoc3

# Install protoc
RUN mv protoc3/bin/* /usr/local/bin/ && \
    mv protoc3/include/* /usr/local/include/
ENV PROTOC=/usr/local/bin/protoc
RUN chmod -R ugo+rwx $(which protoc)

ENV PKG_CONFIG_LIBDIR_armv7_unknown_linux_gnueabihf=/usr/lib/arm-linux-gnueabihf/pkgconfig