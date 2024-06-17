FROM rust:1.79

ENV SQLX_OFFLINE true

# Install necessary dependencies
RUN apt-get update && apt-get install -y \
    postgresql-client \
    && rm -rf /var/lib/apt/lists/*

# Install sqlx CLI
RUN cargo install sqlx-cli --no-default-features --features rustls,postgres

WORKDIR /usr/src/app

COPY . . 

RUN cargo build --bin server

CMD sqlx migrate run && ./target/debug/server
