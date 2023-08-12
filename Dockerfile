# Main official image of rust
FROM rust:1.70 as builder

# Adding new rust project to /opt
WORKDIR /opt
# RUN USER=root cargo new --bin backson
# WORKDIR /opt/backson

# Copying toml and lock to build the project depts for the caching stage
#COPY ./Cargo.lock ./Cargo.lock
#COPY ./Cargo.toml ./Cargo.toml
#RUN cargo build --release

# Remove blank project with dependencies built and adding the real one
# RUN rm ./src/*.rs
# RUN rm ./target/release/deps/axum_backend*

# Adding the new one and building it on top of the cached dependencies
ADD ./Cargo.toml ./Cargo.toml
ADD ./src ./src
RUN cargo build --release

# Setting up the runner machine
FROM debian:stable-20230522 as runner

# Installing Postgres Dependencies
RUN apt-get update && apt-get install -y \
    postgresql  \
    gcc

# Copying the executable file from builder stage
# WORKDIR /opt/backson
WORKDIR /opt
# COPY --from=builder /opt/backson/target/release/axum_backend .
COPY --from=builder /opt/target/release/axum_backend .

# Run the executable
# CMD ["/opt/backson/axum_backend"]
CMD ["/opt/axum_backend"]
