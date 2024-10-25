# Use the official Rust image
FROM rust:latest AS builder

# Create a new empty shell project
RUN USER=root cargo new --bin myapp
WORKDIR /myapp

# Copy source files and compile dependencies
COPY Cargo.toml Cargo.lock ./
RUN cargo build --release
RUN rm src/*.rs

# Copy the source code and build the application
COPY ./src ./src
RUN cargo build --release

# Set up the final image
FROM debian:buster-slim
COPY --from=builder /myapp/target/release/myapp /usr/local/bin/myapp

# Set the startup command
CMD ["week-3-assignment"]
