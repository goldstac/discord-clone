FROM rust:1.80 as builder

WORKDIR /app
COPY server/Cargo.toml server/Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release
RUN rm -rf src

COPY server/src ./src
RUN cargo build --release

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y libssl3 ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/discord-clone-server /usr/local/bin/

ENV DATABASE_URL=sqlite:///data/discord.db
ENV JWT_SECRET=change-this-to-a-random-secret
EXPOSE 3000

CMD ["discord-clone-server"]
