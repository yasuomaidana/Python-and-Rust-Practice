# Build Stage
FROM rust:1-alpine3.20 AS builder

RUN apk add --no-cache musl-dev

RUN rustup target add x86_64-unknown-linux-musl

WORKDIR /app
COPY Cargo.toml Cargo.toml
COPY src src
RUN cargo build --release --target aarch64-unknown-linux-musl

# Runtime Stage
FROM alpine:latest
RUN apk add --no-cache util-linux
WORKDIR /app
COPY --from=builder /app/target/aarch64-unknown-linux-musl/release/blkrust-cli ./
# This command uses the install utility to copy the blkrust-cli binary 
# from the current directory to /usr/local/bin/ with the specified permissions. 
# The -m 755 option sets the file permissions to 755, which means the owner can read, 
# write, and execute the file, while others can only read and execute it. 
# This makes the blkrust-cli binary executable from anywhere in the system.
RUN install -m 755 blkrust-cli /usr/local/bin/blkrust-cli
ENTRYPOINT ["sh"]  