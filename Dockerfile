FROM rust:1.75-slim-buster
RUN apt-get update && apt-get install -y pkg-config libssl-dev
COPY Cargo.toml Cargo.lock ./
RUN mkdir src \
      && echo "fn main() {println!(\"if you see this, the build broke\")}" > src/main.rs \
      && cargo build --release \
      && rm -rf target/release/deps/nino_api* \
      && rm -rf src
COPY . .
RUN cargo build --release
EXPOSE 8000
ENV ROCKET_ENV=production
CMD ["./target/release/nino-api"]
