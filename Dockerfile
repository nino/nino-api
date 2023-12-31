FROM rust:1.75
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {println!(\"if you see this, the build broke\")}" > src/main.rs
RUN cargo build --release
RUN rm -rf target/release/deps/nino_api*
RUN rm -rf src
COPY . .
RUN cargo build --release
EXPOSE 8000
ENV ROCKET_ENV=production
CMD ["./target/release/nino-api"]
