FROM rust:1.79

ENV SQLX_OFFLINE true

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

CMD ["./target/debug/server"]
