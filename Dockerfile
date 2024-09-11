FROM rust:1.75 as builder

WORKDIR /usr/src/app
COPY . .
RUN cargo build --release

FROM debian:buster-slim
RUN apt-get update && apt-get install -y libpq-dev && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/src/app/target/release/distributed-file-storage /usr/local/bin/distributed-file-storage
COPY --from=builder /usr/src/app/migrations /usr/local/bin/migrations
# RUN apt-get update && apt-get install -y libpq-dev && rm -rf /var/lib/apt/lists/*
# COPY --from=builder /usr/src/app/target/release/distributed-file-storage /usr/local/bin/distributed-file-storage
# COPY --from=builder /usr/src/app/target/release/migrations /usr/local/bin/migrations

CMD ["distributed-file-storage"]