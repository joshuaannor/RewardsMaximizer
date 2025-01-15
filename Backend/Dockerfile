# 1. This tells docker to use the Rust official image
FROM rust:1.78-slim-buster

# 2. Copy the files in your machine to the Docker image
COPY ./ ./

WORKDIR "/RewardsMaxAPI"

# Build your program for release
RUN cargo build

EXPOSE 8080

# Run the binary
CMD ["./target/debug/Rewards-Maximizer-API"]