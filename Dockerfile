FROM debian:buster as base

RUN apt update && apt install -y curl
RUN curl --proto '=https' -sSf https://sh.rustup.rs | sh -s -- --default-toolchain none -y
ENV PATH="/root/.cargo/bin:${PATH}"
RUN rustup toolchain install stable --allow-downgrade --profile minimal

RUN apt install -y gcc

COPY . /app
RUN cd /app && cargo build --release


FROM debian:buster

COPY --from=base /app/target/release/vps_bench /bin/

RUN apt update && apt install -y python3 python3-pip && pip3 install speedtest-cli
