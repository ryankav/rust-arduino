FROM ubuntu:22.04

ENV PATH="/root/.cargo/bin:${PATH}"

RUN apt-get update && DEBIAN_FRONTEND=noninteractive apt-get install -y \
    avrdude \
    avr-libc \
    build-essential \
    curl \
    gcc-avr \
    libssl-dev \
    libudev-dev \
    openssl \
    pkg-config

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs \
    | sh \
    && rustup toolchain install nightly \
    && cargo +stable install ravedude

WORKDIR arduino

COPY . .

CMD ["/bin/bash"]

