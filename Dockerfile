# Use the official Rust image
FROM rust:latest AS builder

# Create a new binary project directory
RUN USER=root cargo new --bin week-3-assignment
WORKDIR /week-3-assignment

# Copy only Cargo.toml and Cargo.lock to cache dependencies
COPY Cargo.toml Cargo.lock ./
RUN cargo build --release || true

# Remove the generated `src/main.rs` to prevent conflicts when copying src files
RUN rm src/*.rs

# Copy the actual source files, excluding `fetcher.rs`
COPY ./src ./src
RUN rm -f src/fetcher.rs  # Remove fetcher.rs if it exists

# Build the application
RUN cargo build --release

# Set up the final image
FROM debian:buster-slim
# Copy the built binary to the runtime image
COPY --from=builder /week-3-assignment/target/release/week-3-assignment /usr/local/bin/week-3-assignment

# Set the startup command
CMD ["week-3-assignment"]

