FROM rust:1.48.0

RUN git clone https://github.com/ShoichiTeshima/actix-web-test.git
WORKDIR /actix-web-test

RUN cargo install --path .
RUN cargo build

RUN git pull
RUN cargo build


RUN git pull
RUN cargo build

# COPY Cargo.toml ./
# COPY Cargo.lock ./
# RUN git pull
# RUN cargo install --path .
