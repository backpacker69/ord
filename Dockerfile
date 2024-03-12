FROM rust:1.75.0-bookworm as builder

WORKDIR /usr/src/ord

COPY . .

RUN cargo build --bin ord --release

FROM debian:bookworm-slim

COPY --from=builder /usr/src/ord/target/release/ord /usr/local/bin
RUN apt-get update && apt-get install -y openssl

ENV RUST_BACKTRACE=1
ENV RUST_LOG=info

# Prevents `VOLUME $DIR/index-data/` being created as owned by `root`
RUN mkdir -p "$DIR/index-data/"

# Expose volume containing all `index-data` data
VOLUME $DIR/index-data/

# REST interface
EXPOSE 8080

# Set the entrypoint
ENTRYPOINT ["ord"]

CMD ["--data-dir", "/index-data", "server", "--http-port=8080"]
