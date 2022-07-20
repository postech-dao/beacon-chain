# Step 1: Build backend
FROM rust:1.61-alpine AS builder-backend

RUN rustup target add x86_64-unknown-linux-musl
RUN apk add --no-cache musl-dev openssl-dev

WORKDIR /src

COPY /. .

RUN USER=root cargo build -p pbc-explorer --target x86_64-unknown-linux-musl --release

# Step 2: Compose final image
FROM alpine

WORKDIR /src

COPY --from=builder-backend /src/target/x86_64-unknown-linux-musl/release/pbc-explorer ./

ENV RUST_LOG=debug 
CMD ["./pbc-explorer", "4000"]