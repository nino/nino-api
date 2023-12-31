FROM rust:1.75-slim-buster AS BUILDER
RUN apt-get update && apt-get install -y pkg-config libssl-dev
WORKDIR /usr/src/nino-api
COPY Cargo.toml Cargo.lock Rocket.toml ./
ENV ROCKET_ENV=production
RUN mkdir src \
      && echo "fn main() {println!(\"if you see this, the build broke\")}" > src/main.rs \
      && cargo build --release \
      && rm -rf target/release/deps/nino_api* \
      && rm -rf src
COPY src ./src
RUN cargo build --release

FROM debian:buster-slim
RUN apt-get update && apt-get install -y libssl-dev \
    && apt-get install -y ca-certificates && apt-get clean && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/src/nino-api/target/release/nino-api /usr/local/bin/nino-api
COPY Rocket.toml .
EXPOSE 8000
ENV ROCKET_ENV=production
CMD ["nino-api"]
