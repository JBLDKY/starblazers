FROM rust:1.79

ENV SQLX_OFFLINE true

# Install necessary dependencies
RUN apt-get update && apt-get install -y \
    postgresql-client \
    && rm -rf /var/lib/apt/lists/*

# Install sqlx CLI
RUN cargo install sqlx-cli --no-default-features --features rustls,postgres

WORKDIR /usr/src/app

# Copy the Cargo.toml and Cargo.lock files
# COPY Cargo.toml Cargo.lock ./

# This step is optional but recommended for caching purposes:
# Create a dummy main.rs and build the dependencies to cache the dependencies
# RUN mkdir src && echo "fn main() {println!(\"If you see this, the build dependencies were cached successfully\");}" > src/main.rs

COPY . . 
RUN cargo build --bin server

# Remove the dummy main.rs
# RUN rm -f src/main.rs

# Copy the rest of the application code
#COPY . .

CMD sqlx migrate run && ./target/debug/server
#CMD ["./target/debug/server"]
