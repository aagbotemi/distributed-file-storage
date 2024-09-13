FROM rust:1.75-bullseye as builder

WORKDIR /usr/src/app
COPY . .
RUN cargo build --release

FROM debian:bullseye-slim

RUN apt-get update && \
    apt-get install -y libpq-dev ca-certificates && \
    rm -rf /var/lib/apt/lists/*
    
COPY --from=builder /usr/src/app/target/release/distributed-file-storage /usr/local/bin/distributed-file-storage
COPY --from=builder /usr/src/app/migrations /usr/local/bin/migrations

CMD ["distributed-file-storage"]