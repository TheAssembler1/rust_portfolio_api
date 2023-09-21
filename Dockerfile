# Stage 1: Build the Rust Application
FROM rust:bookworm as build
WORKDIR /usr/src/portfolio_api
COPY . .
RUN cargo build --release

# Stage 2: Create the Final Image
FROM debian:bookworm-slim
RUN apt-get update -y && apt-get install -y openssl
WORKDIR /usr/local/bin/
COPY --from=build /usr/src/portfolio_api/target/release/portfolio_api .
EXPOSE 8080
CMD ["./portfolio_api"]

