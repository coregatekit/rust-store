# Stage 1: Build the application
FROM rust:1.93-bookworm AS builder

# Install necessary build dependencies
RUN apt-get update \
	&& apt-get install -y --no-install-recommends pkg-config libssl-dev libpq-dev \
	&& rm -rf /var/lib/apt/lists/*

# Set the working directory inside the container
WORKDIR /app

# Copy the Cargo.toml and Cargo.lock files to the container
COPY Cargo.toml Cargo.lock ./

# Copy the source code into the container
COPY src ./src
RUN cargo build --release

# Stage 2: Create a minimal runtime image
FROM debian:bookworm-slim

RUN apt-get update \
	&& apt-get install -y --no-install-recommends ca-certificates libssl3 libpq5 \
	&& rm -rf /var/lib/apt/lists/*

# Copy the compiled binary from the builder stage
COPY --from=builder /app/target/release/rust-store /rust-store

# Setup a non-root user to run the application
RUN useradd -r -s /bin/false rustaceans
USER rustaceans

# Set the entry point to run the application
EXPOSE 8080
CMD [ "/rust-store" ]